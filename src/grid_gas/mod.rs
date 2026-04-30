use std::marker::PhantomData;

use bevy::app::{App, Plugin, PluginGroup, PluginGroupBuilder, Plugins};
use nalgebra::RealField;
use wacky_bag::math::normal_cdf::NormalCdfConsts;

use crate::{grid_gas::{resource::{GridGasResource, grid_gas_apply_changes_plugin}, spread::grid_gas_spread}, schedule::schedule_sim};

pub mod edge_type;
pub mod spread;
pub mod resource;
pub mod at_grid_gas;


pub struct GridGasPlugins<Num,const DIM:usize,Marker>
where 
	Num:RealField+Copy+NormalCdfConsts<Marker>,
	Marker:Send+Sync+'static,
{
	pub p:PhantomData<Marker>,
	pub resource:GridGasResource<Num,DIM>,
}

pub struct GridGasPluginsWithEdge<Num,const DIM:usize,Marker,EdgeType>
where 
	Num:RealField+Copy+NormalCdfConsts<Marker>,
	Marker:Send+Sync+'static,
	EdgeType:PluginGroup
{
	pub p:PhantomData<([Num;DIM],Marker)>,
	pub resource:GridGasResource<Num,DIM>,
	pub edge_type:EdgeType
}


impl<Num,const DIM:usize,Marker> PluginGroup for GridGasPlugins<Num,DIM,Marker> 
where 
	Num:RealField+Copy+NormalCdfConsts<Marker>,
	Marker:Send+Sync+'static,

{
	fn build(self) -> PluginGroupBuilder {
		PluginGroupBuilder::start::<Self>()
			.add(move |app: &mut App|{app.insert_resource(self.resource.clone());})
			.add(grid_gas_apply_changes_plugin::<Num,DIM>)
			
			.add(at_grid_gas::plugin::<Num,DIM>)
			
			.add(spread::grid_gas_spread_plugin::<Num,DIM,Marker>)
	}
}

impl<Num,const DIM:usize,Marker,EdgeType> PluginGroup for GridGasPluginsWithEdge<Num,DIM,Marker,EdgeType> 
where 
	Num:RealField+Copy+NormalCdfConsts<Marker>,
	Marker:Send+Sync+'static,
	EdgeType:PluginGroup

{
	fn build(self) -> PluginGroupBuilder {
		PluginGroupBuilder::start::<Self>()
			.add(move |app: &mut App|{app.insert_resource(self.resource.clone());})
			.add(grid_gas_apply_changes_plugin::<Num,DIM>)
			
			.add(at_grid_gas::plugin::<Num,DIM>)
			
			.add(|app:&mut App|{app.add_systems(schedule_sim(), grid_gas_spread::<Num,DIM,Marker>);})
			
			// .add(self.edge_type.build())
			.add_group(self.edge_type)
	}
}