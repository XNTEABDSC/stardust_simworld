use std::{marker::PhantomData, ops::AddAssign} ;

use bevy::{app::App, ecs::{schedule::{IntoScheduleConfigs, ScheduleConfigs}, system::ScheduleSystem}, utils::default} ;
use frunk::{HList, Poly};
use nalgebra::{Const, DefaultAllocator, DimMin, DimName, RealField, allocator::Allocator};
use num_traits::Zero;
use physics_basic::{rotation::{DimNameToSoDimName, DimNameToSoDimNameType}, stat_to_change_type::{HMapStatToChangeTypeZ, MapStatToChangeTypeZ}};
use wacky_bag_hlist::{impl_func_clause, h_list_helpers::{HMapP, MapToPhantom}};
use wacky_bag_bevy::{stat_component::change::Change, system::{processing_system::ScheduleConfigsProcessing, propagate_relationship::{PropagateChangeLeafToRoot, propagate_leaf_to_root}}};

use crate::{multi_body::attach::AttachTo, physics::bundle::PhyBodyStatisticBundleDetermining, schedule::schedule_apply_change};

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

pub fn change_propagate_leaf_to_root_systen<T>()->ScheduleConfigs<ScheduleSystem>
where T:Send+Sync+AddAssign+'static+Zero{
	propagate_leaf_to_root::<PropagateChangeLeafToRoot<T>,AttachTo>.into_configs()
	.config_processing::<
		HList!(),
		HList!(Change<T>),
		HList!()
	>()
}

pub fn change_propagate_leaf_to_root_plugin<Num,const DIM:usize>(app:&mut App)
where 
	Num:RealField+Copy,
	Const<DIM>: DimNameToSoDimName + DimName + DimMin<Const<DIM>, Output = Const<DIM>>,
	DefaultAllocator: Allocator<DimNameToSoDimNameType<DIM>, DimNameToSoDimNameType<DIM>,Buffer<Num>:Sync+Send>+Allocator<DimNameToSoDimNameType<DIM>,Buffer<Num>:Sync+Send>,
    DimNameToSoDimNameType<DIM>:
        DimMin<DimNameToSoDimNameType<DIM>, Output = DimNameToSoDimNameType<DIM>>,
{
	// HMapStatToChangeTypeZ
	let dwa=default::<
		HMapP<HMapStatToChangeTypeZ<PhyBodyStatisticBundleDetermining<Num,DIM>,_>,MapToPhantom>
	>();
	let cfgsh=dwa.map(Poly(
		impl_func_clause!(<T>{where T:Send+Sync+AddAssign+'static+Zero}:(PhantomData<T>)->(ScheduleConfigs<ScheduleSystem>)
		|_a|{
			change_propagate_leaf_to_root_systen::<T>()
		}
	)
	));
	cfgsh.foldl(
		Poly(impl_func_clause!(<'a>:((&'a mut App,ScheduleConfigs<ScheduleSystem>))->(&'a mut App) |(app,s)|{app.add_systems(schedule_apply_change(),s)})), 
		app
	);
}