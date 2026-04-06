use std::ops::Add;

use bevy::{app::App, ecs::{resource::Resource, system::ResMut}};
use frunk::{Poly, ToMut, hlist::HMappable};
use nalgebra::RealField;
use statistic_physics::{formulas::calculate_matters_state_inplace_m, matters::{MattersBasic, MattersFull}};
use wacky_bag::{structures::n_dim_array::t_n_dim_array::TNDimArrayForEachParallel, utils::output_func::HMappableFrom};
use wacky_bag_bevy::utils::{stat_for_hlist::{HApplyChange, MapFromStatMut, MapToChange, MapToStat}, thread_scope::ComputeTaskPoolScopeCreater};


use crate::{grid::grid::GridResource, schedule::schedule_apply_change};

// use statistic_physics::matters::Matters;

// pub type GridGasStat<const DIM:usize>=HList!(Matters<DIM>);
pub type GridGasDatas<Num,const DIM:usize>=
	<
		<MattersFull::<Num,DIM> as HMappable<Poly<MapToStat>>>::Output
		as Add<<MattersBasic::<Num,DIM> as HMappable<Poly<MapToChange>>>::Output
	>
	>::Output;

#[derive(Resource)]
pub struct GridGasResource<Num,const DIM:usize>(pub GridResource<DIM,GridGasDatas<Num,DIM>>);

pub fn grid_gas_apply_changes<Num:RealField+Copy+Default,const DIM:usize>(mut grid_gas:ResMut<GridGasResource<Num,DIM>>){
	grid_gas.0.for_each_mut_parallel(&|g,_|{
		let (changes,r)
			:(<<MattersBasic::<Num,DIM> as HMappable<Poly<MapToChange>>>::Output as ToMut>::Output,_)
			=g.to_mut().sculpt();
		let (stats,_r)
			:(<<MattersBasic::<Num,DIM> as HMappable<Poly<MapToStat>>>::Output as ToMut>::Output,_)
			=r.sculpt();
		changes.zip(stats).map(Poly(HApplyChange));
		
		calculate_matters_state_inplace_m(
			HMappableFrom::output_map(
				g.to_mut().sculpt().0,
				Poly(MapFromStatMut)
			)
		);
		
	}, &ComputeTaskPoolScopeCreater);
}

pub fn grid_gas_apply_changes_plugin<Num:RealField+Copy+Default,const DIM:usize>(app:&mut App){
	app.add_systems(schedule_apply_change(), grid_gas_apply_changes::<Num,DIM>);
}