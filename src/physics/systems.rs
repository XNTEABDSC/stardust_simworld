use std::marker::PhantomData;

use bevy::{app::{App, PluginGroup, PluginGroupBuilder}, ecs::{schedule::ScheduleConfigs, system::ScheduleSystem}};
use frunk::Poly ;
use nalgebra::{Const, DefaultAllocator, DimName, RealField};
use physics_basic::{body::{calculate_angular_state, calculate_position_state, calculate_rotation_matrix, collect_angular_state_det_agi_agm_change_agv, collect_position_state_det_pos_momentum_change_vel}, rotation::{AllocatorSyncVMSq, ConstDimToSoDimT, DimSquare, DimToSoDim}};
use statistic_physics::formulas::{calculate_density, calculate_vel_var};
use wacky_bag_hlist::{chain_fn::ChainFunc, h_list_helpers::{FoldVecPush, HMapP, HTypeMapP, HZip, MapToPhantom}, };
use wacky_bag::{structures::owned::Owned, utils::{default_of::default, }};
use wacky_bag_bevy::{stat_component::stat_apply_change::StatChangeToApplyChanges, system::{hlist_fn_to_system::{to_calculate_stat_system_with_processing, to_collect_change_system_with_processing}, multi_sets::FoldScheduleConfigsInSet, processing_system::MapToProcessingSystemSet }, utils::stat_for_hlist::{MapToDetermining, MapToQFWith} };


use crate::{physics::bundle::PhyBodyStatisticBundleDetermining, schedule::schedule_apply_change};

// fn test_2(app:&mut App){
// 	// let dwa=CalculateSystem(calculate_position_state);
// 	let dwa=
// 	CalculateSystemTest::<_>(calculate_position_state,Default::default());
// 	// test_is_system(dwa);
// 	// let awd=IntoSystem::into_system(dwa);
// 	// app.add_systems(schedule_pre_sim(), IntoSystem::into_system(CalculateSystem(calculate_position_state)));
// 	// app.add_systems(schedule_pre_sim(), systems)
// 	let awd=IntoSystem::into_system(dwa);
// }

// fn is_into_system<F,A:SystemInput,B,M>(f:&F)
// 	where F:IntoSystem<A,B,M>
// {}

// fn is_system_param_function<T,M>(v:&T)
// 	where T:SystemParamFunction<M>
// {

// }

pub fn calculate_position_state_plugin<Num:RealField+Copy,const DIM:usize>(app:&mut App){
	app.add_systems(schedule_apply_change(), to_calculate_stat_system_with_processing(calculate_position_state::<Num,DIM>));
}

pub fn calculate_angular_state_plugin<Num,const DIM:usize>(app:&mut App)
where
	Num:RealField+Copy,
	Const<DIM>: DimToSoDim + DimName + DimSquare,
	DefaultAllocator: AllocatorSyncVMSq<ConstDimToSoDimT<DIM>,Num>,
    ConstDimToSoDimT<DIM>:DimSquare,
{
	app.add_systems(schedule_apply_change(), to_calculate_stat_system_with_processing(calculate_angular_state::<Num,DIM>));
}

pub fn calculate_vel_var_plugin<Num:RealField+Copy,const DIM:usize>(app:&mut App){
	app.add_systems(schedule_apply_change(), to_calculate_stat_system_with_processing(calculate_vel_var::<Num,DIM>));
}

pub fn calculate_density_plugin<Num:RealField+Copy>(app:&mut App){
	app.add_systems(schedule_apply_change(), to_calculate_stat_system_with_processing(calculate_density::<Num>));
}

pub fn calculate_rotation_matrix_plugin<Num:RealField+Copy,const DIM:usize>(app:&mut App)
where
	Const<DIM>: DimToSoDim + DimName + DimSquare,
	DefaultAllocator: AllocatorSyncVMSq<ConstDimToSoDimT<DIM>,Num>,
    ConstDimToSoDimT<DIM>:DimSquare,
{
	app.add_systems(schedule_apply_change(), to_calculate_stat_system_with_processing(calculate_rotation_matrix::<Num,DIM>));

}

pub fn collect_position_state_det_pos_momentum_change_vel_plugin<Num:RealField+Copy,const DIM:usize>(app:&mut App){
	// let dwa:<HList!(Vel<Num,DIM>) as HMappable<Poly<HTypeFnToMapper< ChainFunc< MapToChange, ReverseFunc<MapFromMut>> >>> >::Output;
	// let a:bevy::ecs::world::Mut<'static,i32>;
	// let b=MapDerefMut::call(&mut a);
	// a.der
	// let wat:<bevy::ecs::world::Mut<'static,i32> as std::ops::Dere>::Target;
	// let dwa:<frunk::HCons<bevy::ecs::world::Mut<'static, wacky_bag_bevy::stat_component::change::Change<physics_basic::stats::Vel<Num, DIM>>>, HNil> as HMappable<Poly<MapDerefMut>>>::Output;
	app.add_systems(schedule_apply_change(), to_collect_change_system_with_processing(collect_position_state_det_pos_momentum_change_vel::<Num,DIM>));
}

pub fn collect_angular_state_det_agi_agm_change_agv_plugin<Num,const DIM:usize>(app:&mut App)
where
	Num:RealField+Copy,
	Const<DIM>: DimToSoDim + DimName,
	DefaultAllocator: AllocatorSyncVMSq<ConstDimToSoDimT<DIM>,Num>,
    ConstDimToSoDimT<DIM>:DimSquare,
{
	app.add_systems(schedule_apply_change(), to_collect_change_system_with_processing(collect_angular_state_det_agi_agm_change_agv::<Num,DIM>));
}
#[derive(Debug,Clone, Copy)]
pub struct CalculateSystemsPlugins<Num,const DIM:usize>(pub PhantomData<[Num;DIM]>)
where
	Num:RealField+Copy,
	Const<DIM>: DimToSoDim + DimName,
	DefaultAllocator: AllocatorSyncVMSq<ConstDimToSoDimT<DIM>,Num>,
    ConstDimToSoDimT<DIM>:DimSquare;

impl<Num,const DIM:usize> Default for CalculateSystemsPlugins<Num,DIM>
where
	Num:RealField+Copy,
	Const<DIM>: DimToSoDim + DimName,
	DefaultAllocator: AllocatorSyncVMSq<ConstDimToSoDimT<DIM>,Num>,
    ConstDimToSoDimT<DIM>:DimSquare,
{
	fn default() -> Self {
		Self(Default::default())
	}
}

impl<Num,const DIM:usize> PluginGroup for CalculateSystemsPlugins<Num,DIM> 
where
	Num:RealField+Copy,
	Const<DIM>: DimToSoDim + DimName + DimSquare,
	DefaultAllocator: AllocatorSyncVMSq<ConstDimToSoDimT<DIM>,Num>,
    ConstDimToSoDimT<DIM>:DimSquare,
{
	fn build(self) -> PluginGroupBuilder {
		// let dwa:fn(&mut App)=calculate_position_state_plugin::<Num,DIM>;
		let res=
		PluginGroupBuilder::start::<Self>()
			.add(calculate_position_state_plugin::<Num,DIM>)
			.add(calculate_angular_state_plugin::<Num,DIM>)
			.add(calculate_vel_var_plugin::<Num,DIM>)
			.add(calculate_density_plugin::<Num>)
			.add(calculate_rotation_matrix_plugin::<Num,DIM>)
			.add(collect_position_state_det_pos_momentum_change_vel_plugin::<Num,DIM>)
			.add(collect_angular_state_det_agi_agm_change_agv_plugin::<Num,DIM>)
			.add(spawn_stat_apply_change_system_plugin::<Num,DIM>)
		;
		
		res
	}
}

pub fn spawn_stat_apply_change_system_plugin<Num,const DIM:usize>(app:&mut App)
where 
	Num:RealField+Copy,
	Const<DIM>: DimToSoDim + DimName,
	DefaultAllocator: AllocatorSyncVMSq<ConstDimToSoDimT<DIM>,Num>,
    ConstDimToSoDimT<DIM>:DimSquare,
{
	let cfgsh=
	default::<
	HMapP<
		HZip<
		HZip<
			PhyBodyStatisticBundleDetermining<Num,DIM>,
			_>,
			HTypeMapP<PhyBodyStatisticBundleDetermining<Num,DIM>,ChainFunc<MapToDetermining,MapToQFWith>>
		>,
		MapToPhantom>
	>()
	.map(Poly(StatChangeToApplyChanges));

	let cfgsh=
	cfgsh.zip( 
		default::<HMapP< HMapP<PhyBodyStatisticBundleDetermining<Num,DIM>,MapToDetermining>,MapToProcessingSystemSet>>() )
	.map(
		Poly(FoldScheduleConfigsInSet)
	);

	// cfgsh.zip(default::<HMapP<PhyBodyStatisticBundleDetermining<Num,DIM>,MapToPhantom>>()).map(
	// 	impl_func_closure!(<T>:
	// 		((ScheduleConfigs<ScheduleSystem>,PhantomData<T>))
	// 		|(sys,a)|{

	// 		}
	// 	)
	// );

	let Owned(cfgs)=cfgsh.foldl(Poly(FoldVecPush), Owned(Vec::<ScheduleConfigs<ScheduleSystem>>::new()));
	let cfg= ScheduleConfigs::Configs { configs: cfgs, collective_conditions:default(),metadata:default() };
	
	app.add_systems(schedule_apply_change(), cfg);
}

