use std::{marker::PhantomData, ops::AddAssign} ;

use bevy::{app::App, ecs::{query::ROQueryItem, relationship::Relationship, schedule::{IntoScheduleConfigs, ScheduleConfigs}, system::{ScheduleSystem, SystemParamItem}}, utils::default} ;
use frunk::{HList, Poly};
use nalgebra::{Const, DefaultAllocator, DimMin, DimName, RealField, allocator::Allocator};
use num_traits::Zero;
use physics_basic::{ rotation::{AllocatorSyncVMSq, AllocatorVM, AngularMomentum, ConstDimToSoDimT, DimSquare, DimToSoDim, angular_momentum_from_momentum_pos}, stat_to_change_type::{HMapStatToChangeTypeZ, MapStatToChangeTypeZ}, stats::{Momentum, Pos}};
use wacky_bag_hlist::{impl_func_closure, h_list_helpers::{HMapP, MapToPhantom}};
use wacky_bag_bevy::{stat_component::{change::Change, stat::Stat}, system::{processing_system::ScheduleConfigsProcessing, propagate_relationship::{PropagateChangeLeafToRoot, PropagateLeafToRoot, PropagateLeafToRootApplySysParam, PropagateLeafToRootFromSysParam, propagate_leaf_to_root}}, utils::system_param_with_query::{SystemParamWithQuery, SystemParamWithQueryMergeT, SystemParamWithQueryT}};

use crate::{multi_body::{attach::AttachTo, propagate_position::AttachToPos}, physics::bundle::PhyBodyStatisticBundleDetermining, schedule::schedule_apply_change};

// pub fn change_propagate_leaf_to_root<T>(
// 	ps:ParamSet<(
// 		Query<(&Change<T>,&AttachTo),Without<AttachedEntities>>,
// 		Query<(&Change<T>,&AttachedEntities,Option<&AttachTo>)>,
// 	)>,
// 	update_sources_set:
// 	Local<DashMap<Entity,usize,EntityHash>>,
// 	//Local<Parallel<EntityHashSet>>, 
// 	//EntityHash 
// 	//DashSet<Entity,EntityHash>>,
// 	// mut update_sources:Local<Parallel<Vec<Entity>>>,
// 	update_tasks:Local<(Parallel<Vec<(Entity,PropagateChangeLeafToRoot<T>)>>,Parallel<Vec<(Entity,PropagateChangeLeafToRoot<T>)>>)>,
// 	// mut update_tasks_cache:Local<Parallel<Vec<(Entity,T)>>>,
// )
// 	where T:Send+Sync+AddAssign+'static+Zero
// {
// 	propagate_leaf_to_root::<PropagateChangeLeafToRoot<T>,AttachTo>(ps, update_sources_set, update_tasks);
// }

pub struct PropagateMomentum<Num,const DIM:usize>
where 
	Num:RealField+Copy,
	Const<DIM>: DimToSoDim + DimName + DimSquare,
	DefaultAllocator: 
		AllocatorSyncVMSq<ConstDimToSoDimT<DIM>,Num>
{
	m:Momentum<Num,DIM>, 
	agm: AngularMomentum<Num,DIM>,
	src_pos:Pos<Num,DIM>
}

impl<Num,const DIM:usize,R:Relationship> PropagateLeafToRoot<R> for PropagateMomentum<Num,DIM> 
where 
	Num:RealField+Copy,
	Const<DIM>: DimToSoDim + DimName + DimSquare,
	DefaultAllocator: 
		AllocatorSyncVMSq<ConstDimToSoDimT<DIM>,Num>,
{
	type FromSysParam=SystemParamWithQueryT<
		(
			&'static Change<Momentum<Num,DIM>>, 
			&'static Change<AngularMomentum<Num,DIM>>, 
			// &'static AttachToPos<Num,DIM> no this is relative to parent, not world
			&'static Stat<Pos<Num,DIM>>
		),
		(),
		()
	>;

	fn from_data(
			values: 
				ROQueryItem<
					<SystemParamWithQueryMergeT<PropagateLeafToRootFromSysParam<R>, Self::FromSysParam> as SystemParamWithQuery>::D
				>,
			_others:
				&SystemParamItem<
					<SystemParamWithQueryMergeT<PropagateLeafToRootFromSysParam<R>, Self::FromSysParam> as SystemParamWithQuery>::P
				>
		)->Self {
		let m=values.1.0.get_and_reset_ref();
		let agm = values.1.1.get_and_reset_ref();
		let pos=values.1.2.0;
		Self { agm: agm,m: m ,src_pos:pos}
	}

	type ApplySysParam=SystemParamWithQueryT<
		(
			&'static Change<Momentum<Num,DIM>>, 
			&'static Change<AngularMomentum<Num,DIM>>, 
			&'static Stat<Pos<Num,DIM>>
		),
		(),
		()
	>;

	fn apply_to_data(
			self,
			values:
				ROQueryItem<
					<SystemParamWithQueryMergeT<PropagateLeafToRootApplySysParam<R>, Self::ApplySysParam> as SystemParamWithQuery>::D
				>,
			//SystemParamWithQueryROItem<'w,'s,SPQMerge<PropagateLeafToRootApplySysParam<R>, Self::ApplySysParam>>,
			_others:
				&SystemParamItem<
					<SystemParamWithQueryMergeT<PropagateLeafToRootApplySysParam<R>, Self::ApplySysParam> as SystemParamWithQuery>::P
				>
		) {
		values.1.0.add_change(self.m);
		values.1.1.add_change(self.agm + angular_momentum_from_momentum_pos(&(self.src_pos-values.1.2.0), &self.m));
	}
}

pub fn momentum_change_propagate_leaf_to_root_system_cfg<Num,const DIM:usize>()->ScheduleConfigs<ScheduleSystem>
where 
	Num:RealField+Copy,
	Const<DIM>: DimToSoDim + DimName + DimSquare,
	DefaultAllocator: 
		AllocatorSyncVMSq<ConstDimToSoDimT<DIM>,Num>,
{
	propagate_leaf_to_root::<PropagateMomentum<Num,DIM>,AttachTo>.into_configs()
	.config_processing::<
		HList!(),
		HList!(Change<Momentum<Num,DIM>>,Change<AngularMomentum<Num,DIM>>),
		HList!()
	>()
}

pub fn all_change_propagate_leaf_to_root_system_cfg<T>()->ScheduleConfigs<ScheduleSystem>
where T:Send+Sync+AddAssign+'static+Zero
{
	propagate_leaf_to_root::<PropagateChangeLeafToRoot<T>,AttachTo>.into_configs()
	.config_processing::<
		HList!(),
		HList!(Change<T>),
		HList!()
	>()
}

pub fn all_change_propagate_leaf_to_root_plugin<Num,const DIM:usize>(app:&mut App)
where 
	Num:RealField+Copy,
	Const<DIM>: DimToSoDim + DimName + DimSquare,
	DefaultAllocator: 
		AllocatorSyncVMSq<ConstDimToSoDimT<DIM>,Num>,
	//Allocator<DimNameToSoDimNameType<DIM>, DimNameToSoDimNameType<DIM>,Buffer<Num>:Sync+Send>+Allocator<DimNameToSoDimNameType<DIM>,Buffer<Num>:Sync+Send>,
{
	// HMapStatToChangeTypeZ
	let dwa=default::<
		HMapP<HMapStatToChangeTypeZ<PhyBodyStatisticBundleDetermining<Num,DIM>,_>,MapToPhantom>
	>();
	let cfgsh=dwa.map(Poly(
		impl_func_closure!(<T>{where T:Send+Sync+AddAssign+'static+Zero}:(PhantomData<T>)->(ScheduleConfigs<ScheduleSystem>)
			|_a|{
				all_change_propagate_leaf_to_root_system_cfg::<T>()
			}
		)
	));
	cfgsh.foldl(
		Poly(impl_func_closure!(<'a>:((&'a mut App,ScheduleConfigs<ScheduleSystem>))->(&'a mut App) 
			|(app,s)|{app.add_systems(schedule_apply_change(),s)}
		)), 
		app
	);
}