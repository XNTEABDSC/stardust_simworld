use std::{ops::Add, str};

use bevy::ecs::{resource::Resource, system::ResMut};
use frunk::{Func, HList, Poly, ToMut, hlist::{self, HMappable, HZippable}};
use statistic_physics::{formulas::{calculate_matters_state, calculate_matters_state_inplace_m}, matters::{MattersBasic, MattersFull}};
use wacky_bag::{structures::n_dim_array::t_n_dim_array::TNDimArray, utils::{output_func::HMappableFrom, select_zip::HSelectZippable, type_fn::{ChainFunc, ReverseFunc}}};
use wacky_bag_bevy::{stat_component::{change::Change, stat::Stat}, utils::{stat_for_hlist::{HApplyChange, MapFromStatMut, MapFromStatRef, MapToChange, MapToStat}, thread_scope::ComputeTaskPoolScopeCreater}};


use crate::grid::grid::GridResource;

// use statistic_physics::matters::Matters;

// pub type GridGasStat<const DIM:usize>=HList!(Matters<DIM>);


#[derive(Resource)]
pub struct GridGasResource<const DIM:usize>(pub GridResource<DIM,
	
	<
		<MattersFull::<DIM> as HMappable<Poly<MapToStat>>>::Output
		as Add<
		<MattersBasic::<DIM> as HMappable<Poly<MapToChange>>>::Output
	>
	>::Output
>);

pub fn grid_gas_apply_changes<const DIM:usize>(mut grid_gas:ResMut<GridGasResource<DIM>>){
	grid_gas.0.for_each_mut_parallel(&|g,_|{
		let (cs,r)
			:(<<MattersBasic::<DIM> as HMappable<Poly<MapToChange>>>::Output as ToMut>::Output,_)
			=g.to_mut().sculpt();
		let (ss,r)
			:(<<MattersBasic::<DIM> as HMappable<Poly<MapToStat>>>::Output as ToMut>::Output,_)
			=r.sculpt();
		cs.zip(ss).map(Poly(HApplyChange));
		
		calculate_matters_state_inplace_m(
			HMappableFrom::output_map(
				g.to_mut().sculpt().0,
				Poly(MapFromStatMut)
			)
		);
		
	}, &ComputeTaskPoolScopeCreater);
}