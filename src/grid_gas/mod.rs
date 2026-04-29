use std::marker::PhantomData;

use bevy::app::{App, Plugin, PluginGroup, PluginGroupBuilder};
use nalgebra::RealField;
use wacky_bag::math::normal_cdf::NormalCdfConsts;

use crate::{grid_gas::{resource::{GridGasResource, grid_gas_apply_changes_plugin}, spread::grid_gas_spread}, schedule::schedule_sim};

pub mod edge_type;
pub mod spread;
pub mod resource;
pub mod at_grid_gas;

pub struct GridGasPlugins<Num,const DIM:usize,Marker,EdgeType>{
	pub p:PhantomData<([Num;DIM],Marker)>,
	pub resource:GridGasResource<Num,DIM>,
	pub edge_type:EdgeType
}

impl<Num,const DIM:usize,Marker,EdgeType> PluginGroup for GridGasPlugins<Num,DIM,Marker,EdgeType> 
where 
	Num:RealField+Copy+Default+NormalCdfConsts<Marker>,
	Marker:Send+Sync+'static,
	EdgeType:Plugin

{
	fn build(self) -> PluginGroupBuilder {
		PluginGroupBuilder::start::<Self>()
			.add(move |app: &mut App|{app.insert_resource(self.resource.clone());})
			.add(grid_gas_apply_changes_plugin::<Num,DIM>)
			
			.add(at_grid_gas::plugin::<Num,DIM>)
			
			.add(|app:&mut App|{app.add_systems(schedule_sim(), grid_gas_spread::<Num,DIM,Marker>);})
			
			.add(self.edge_type)
	}
}