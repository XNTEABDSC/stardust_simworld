use std::marker::PhantomData;

use bevy::{app::{PluginGroup, PluginGroupBuilder, Plugins}, ecs::resource::Resource};
use frunk::HList;
use nalgebra::RealField;
use physics_basic::stats::Vel;
use statistic_physics::{formulas::GasCellSpreadToSideType, stats::{Density, VelVar1Dir, VelVarSq1Dir}};
use wacky_bag::math::normal_cdf::NormalCdfConsts;

use crate::{grid_gas::spread::{grid_gas_spread_edge_const, grid_gas_spread_edge_wall}, schedule::schedule_sim};


#[derive(Resource)]
pub struct GridGasEdgeWall<Num,const DIM:usize,Marker>{
	pub p:PhantomData<([Num;DIM],Marker)>
}

impl<Num, const DIM: usize, Marker> Default for GridGasEdgeWall<Num, DIM, Marker> {
    fn default() -> Self {
		Self { p: Default::default() }
	}
}



impl<Num, const DIM: usize, Marker> Clone for GridGasEdgeWall<Num, DIM, Marker> {
    fn clone(&self) -> Self {
		Self { p: self.p.clone() }
	}
}

impl<Num,const DIM:usize,Marker> PluginGroup for GridGasEdgeWall<Num,DIM,Marker>
	where Num:RealField+Copy+Default+NormalCdfConsts<Marker>,Marker:Send+Sync+'static

{
	fn build(self) -> PluginGroupBuilder {
		PluginGroupBuilder::start::<Self>()
			.add(move |app: &mut bevy::app::App| {app.insert_resource(self.clone());} )
			.add(|app: &mut bevy::app::App| {app.add_systems(schedule_sim(), grid_gas_spread_edge_wall::<Num,DIM,Marker>);} )
	}
}

/// not implemented yet
// #[derive(Resource,Default)]

// pub struct GridGasEdgeLoop<Num,const DIM:usize,Marker>{
// 	pub p:PhantomData<([Num;DIM],Marker)>
// }



#[derive(Resource)]
pub struct GridGasEdgeConst<Num,const DIM:usize,Marker>
	where Num:RealField+Copy
{
	pub const_matters:GasCellSpreadToSideType<Num,DIM>,
	pub p:PhantomData<Marker>
}

impl<Num, const DIM: usize, Marker> Clone for GridGasEdgeConst<Num, DIM, Marker>
where Num:RealField+Copy
{
    fn clone(&self) -> Self {
		Self { const_matters: self.const_matters.clone(), p: self.p.clone() }
	}
}


impl<Num,const DIM:usize,Marker> PluginGroup for GridGasEdgeConst<Num,DIM,Marker>
	where Num:RealField+Copy+Default+NormalCdfConsts<Marker>,Marker:Send+Sync+'static
{
	fn build(self) -> PluginGroupBuilder {
		PluginGroupBuilder::start::<Self>()
			.add(move |app: &mut bevy::app::App| {app.insert_resource(self.clone());} )
			.add(|app: &mut bevy::app::App| {app.add_systems(schedule_sim(), grid_gas_spread_edge_const::<Num,DIM,Marker>);} )
	}
}

// pub trait GridGasEdgeType {
// 	type Resource:Resource;

// }