use bevy::{app::App, ecs::{entity::Entity, query::QueryItem, schedule::{IntoScheduleConfigs, ScheduleConfigs}, system::{ScheduleSystem, SystemParamItem}}};
use bevy_ecs_macros::Component;
use frunk::HList;
use nalgebra::{Const, DefaultAllocator, RealField, allocator::Allocator};
use physics_basic::{rotation::{ConstDimToSoDimT, DimSquare, DimToSoDim, Rotation, RotationMatrix}, stats::Pos};
use wacky_bag_bevy::{stat_component::{cache_set::{CacheSet, set_cache_set_system_cfg}, stat::Stat}, system::{processing_system::ScheduleConfigsProcessing, propagate_relationship::{PropagateRootToLeafMut, PropagateRootToLeafMutBeginSysParam, PropagateRootToLeafMutProcessSysParam, propagate_root_to_leaf, propagate_root_to_leaf_mut}}, utils::system_param_with_query::{SystemParamWithQuery, SystemParamWithQueryMerge, SystemParamWithQueryT}};

use crate::{multi_body::attach::{AttachTo, AttachToWith}, schedule::schedule_apply_change};

type SPQMerge<A,B>=<A as SystemParamWithQueryMerge<B>>::Merge;


pub type AttachToPos<Num,const DIM:usize>=AttachToWith<Pos<Num,DIM>>;
// #[derive(Debug,Component)]
// pub struct AttachToPos<Num,const DIM:usize>(pub Pos<Num,DIM>);
pub type AttachToRotation<Num,const DIM:usize> = AttachToWith<Rotation<Num,DIM>>;
// #[derive(Debug,Component)]
// pub struct AttachToRotation<Num:RealField,const DIM:usize>(pub Rotation<Num,DIM>)
// where Const<DIM>:DimToSoDim,DefaultAllocator:Allocator<ConstDimToSoDimT<DIM>>;

#[derive(Debug, Clone)]
pub struct PropagatePositionRotation<Num:RealField,const DIM:usize>
where Const<DIM>:DimToSoDim,DefaultAllocator:Allocator<ConstDimToSoDimT<DIM>>
{
	pos:Pos<Num,DIM>,
	rot:Rotation<Num,DIM>,
	rot_mat:RotationMatrix<Num,DIM>
}

impl<Num: RealField + Copy, const DIM: usize> Copy for PropagatePositionRotation<Num, DIM>
where Const<DIM>:DimToSoDim,DefaultAllocator:Allocator<ConstDimToSoDimT<DIM>,Buffer<Num>:Copy>
{
}
 
impl<Num:RealField+Copy,const DIM:usize> PropagateRootToLeafMut<AttachTo> for PropagatePositionRotation<Num,DIM>
where Const<DIM>:DimToSoDim+DimSquare,DefaultAllocator:Allocator<ConstDimToSoDimT<DIM>,Buffer<Num> : Send+Sync+Copy>
{
	type BeginSysParam=SystemParamWithQueryT<
		(
			&'static Stat<Pos<Num,DIM>>,
			&'static Stat<Rotation<Num,DIM>>,
			&'static mut CacheSet<Stat<RotationMatrix<Num,DIM>>>
		),(),
		()
		>
		;

	fn process_data_begin(
		mut values: 
			QueryItem<
				<SPQMerge<PropagateRootToLeafMutBeginSysParam<AttachTo>, Self::BeginSysParam> as SystemParamWithQuery>::D
			>,
		_others: 
			&SystemParamItem<
				<SPQMerge<PropagateRootToLeafMutBeginSysParam<AttachTo>, Self::BeginSysParam> as SystemParamWithQuery>::P
			>
	)->
	// Vec<(Entity,Self)>
	impl FnMut(Entity)->Self
	{
		
		let s=Self{
			pos:values.1.0.0,
			rot_mat:values.1.2.get_or_else_set(||RotationMatrix::from_so(&values.1.1.0).into()).0,
			rot:values.1.1.0,
		};
		return move |_e|s.clone();
	}

	type ProcessSysParam=SystemParamWithQueryT<
		(
			Option<&'static AttachToPos<Num,DIM>>,
			Option<&'static AttachToRotation<Num,DIM>>,
			&'static mut Stat<Pos<Num,DIM>>,
			&'static mut Stat<Rotation<Num,DIM>>,
			&'static mut CacheSet<Stat<RotationMatrix<Num,DIM>>>
		),(),
		()
		>
		;

	fn process_data<'w,'s>(
			mut self,
			values: 
				QueryItem<
					<SPQMerge<PropagateRootToLeafMutProcessSysParam<AttachTo>, Self::ProcessSysParam> as SystemParamWithQuery>::D
				>,
			_others: 
				&SystemParamItem<
					<SPQMerge<PropagateRootToLeafMutProcessSysParam<AttachTo>, Self::ProcessSysParam> as SystemParamWithQuery>::P
				>)
		->
		// Vec<(Entity,Self)>
		impl FnMut(Entity)->Self
		{
		let (dp,dr,mut sp,mut sr,mut rm) = values.1;
		if let Some(dp) = dp {
			self.pos = Pos( self.rot_mat.0 * dp.0.0 );
		}
		if let Some(dr) = dr {
			self.rot = Rotation( dr.0.0 + self.rot.0);
		}
		sp.0=self.pos;
		sr.0=self.rot;
		self.rot_mat = rm.set_and_get(RotationMatrix::from_so(&self.rot).into()).0;
		return move |_e|self.clone();
	}
	

	// type DataBegin =(
	// 	&'static Stat<Pos<Num,DIM>>,
	// 	&'static Stat<Rotation<Num,DIM>>,
	// 	&'static mut Stat<RotationMatrix<Num,DIM>>
	// );

	// // type DataBegin =(
	// // 	&'static CacheSet<Stat<Pos<Num,DIM>>>,
	// // 	&'static CacheSet<Stat<Rotation<Num,DIM>>>
	// // );

	// type Data =(
	// 	&'static mut Stat<Pos<Num,DIM>>,
	// 	&'static mut Stat<Rotation<Num,DIM>>,
	// 	Option<&'static AttachToPos<Num,DIM>>,
	// 	Option<&'static AttachToRotation<Num,DIM>>,
	// );

	// fn from_data<'w,'s>(values:&QueryItem<'w,'s,Self::DataBegin>)->Self {
	// 	Self{
	// 		pos:values.0.0,
	// 		rot:values.1.0,
	// 		rot_mat:values.2.0
	// 	}
	// }

	// fn process_data<'w,'s>(&mut self,values:&ROQueryItem<'w,'s,Self::Data>) {
	// 	let (p,r,apm,arm)=values;
	// 	if let Some(ap) = apm {
	// 		self.pos = Pos( self.rot_mat.0 * ap.0.0 );
	// 	}
	// 	if let Some(ar) = arm {
	// 		self.rot = Rotation( ar.0.0 + self.rot.0);
	// 	}
	// 	*p.0.lock().unwrap()
	// 	=Some(Stat(self.pos));
	// 	*r.0.lock().unwrap()
	// 	=Some(Stat(self.rot));
		
	// }
}

pub fn propagate_position_rotation_system<Num:RealField+Copy,const DIM:usize>()->ScheduleConfigs<ScheduleSystem>
where Const<DIM>:DimToSoDim+DimSquare,DefaultAllocator:Allocator<ConstDimToSoDimT<DIM>,Buffer<Num> : Send+Sync+Copy>
{
	propagate_root_to_leaf_mut::<PropagatePositionRotation<Num,DIM>,AttachTo>.into_configs()
	.config_processing::<
		HList!(AttachToPos<Num,DIM>,AttachToRotation<Num,DIM>),
		HList!(PropagatePositionRotation<Num,DIM>, Stat<Pos<Num,DIM>>, Stat<Rotation<Num,DIM>>, CacheSet<Stat<RotationMatrix<Num,DIM>>>),
		HList!()
	>()
}

pub fn propagate_position_rotation_plugin<Num:RealField+Copy,const DIM:usize>(app:&mut App)
where Const<DIM>:DimToSoDim+DimSquare,DefaultAllocator:Allocator<ConstDimToSoDimT<DIM>,Buffer<Num> : Send+Sync+Copy>
{
	app.add_systems(schedule_apply_change(), propagate_position_rotation_system::<Num,DIM>());
	app.add_systems(schedule_apply_change(), 
		(
			set_cache_set_system_cfg::<Stat<RotationMatrix<Num,DIM>>,()>(),
		)
	);
}