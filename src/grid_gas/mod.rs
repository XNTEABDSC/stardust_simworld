use std::marker::PhantomData;

use bevy::app::{App, PluginGroup, PluginGroupBuilder};
use nalgebra::{Const, DefaultAllocator, DimName, RealField, allocator::Allocator};
use physics_basic::rotation::{DimNameToSoDimName, DimNameToSoDimNameType};
use wacky_bag::math::normal_cdf::NormalCdfConsts;
use wacky_bag_bevy::utils::plugin_add_systems::plugin_add_systems;

use crate::{grid_gas::{interact_grid_gas_body::interact_grid_gas_body, resource::{GridGasResource, grid_gas_calculate_plugin}}, schedule::schedule_sim};

pub mod edge_type;
pub mod spread;
pub mod resource;
pub mod at_grid_gas;
pub mod interact_grid_gas_body;


pub struct GridGasPlugins<Num,const DIM:usize,Marker>
where 
	Num:RealField+Copy+NormalCdfConsts<Marker>,
	Marker:Send+Sync+'static,
{
	pub p:PhantomData<Marker>,
	pub resource:GridGasResource<Num,DIM>,
}

// pub struct GridGasPluginsWithEdge<Num,const DIM:usize,Marker,EdgeType>
// where 
// 	Num:RealField+Copy+NormalCdfConsts<Marker>,
// 	Marker:Send+Sync+'static,
// 	EdgeType:PluginGroup
// {
// 	pub p:PhantomData<([Num;DIM],Marker)>,
// 	pub resource:GridGasResource<Num,DIM>,
// 	pub edge_type:EdgeType
// }


impl<Num,const DIM:usize,Marker> PluginGroup for GridGasPlugins<Num,DIM,Marker> 
where 
	Num:RealField+Copy+NormalCdfConsts<Marker>,
	Marker:Send+Sync+'static,
    Const<DIM>: DimNameToSoDimName + DimName,
    DefaultAllocator: Allocator<DimNameToSoDimNameType<DIM>>
        + Allocator<DimNameToSoDimNameType<DIM>, DimNameToSoDimNameType<DIM>, Buffer<Num>:Send+Sync >,

{
	fn build(self) -> PluginGroupBuilder {
		PluginGroupBuilder::start::<Self>()
			.add(move |app: &mut App|{app.insert_resource(self.resource.clone());})
			.add(grid_gas_calculate_plugin::<Num,DIM>)
			
			.add(at_grid_gas::plugin::<Num,DIM>)
			
			.add(spread::grid_gas_spread_plugin::<Num,DIM,Marker>)

			.add(plugin_add_systems(schedule_sim(), interact_grid_gas_body::<Num,DIM>))
	}
}

// impl<Num,const DIM:usize,Marker,EdgeType> PluginGroup for GridGasPluginsWithEdge<Num,DIM,Marker,EdgeType> 
// where 
// 	Num:RealField+Copy+NormalCdfConsts<Marker>,
// 	Marker:Send+Sync+'static,
// 	EdgeType:PluginGroup

// {
// 	fn build(self) -> PluginGroupBuilder {
// 		PluginGroupBuilder::start::<Self>()
// 			.add(move |app: &mut App|{app.insert_resource(self.resource.clone());})
// 			.add(grid_gas_apply_changes_plugin::<Num,DIM>)
			
// 			.add(at_grid_gas::plugin::<Num,DIM>)
			
// 			.add(|app:&mut App|{app.add_systems(schedule_sim(), grid_gas_spread::<Num,DIM,Marker>);})
			
// 			// .add(self.edge_type.build())
// 			.add_group(self.edge_type)
// 	}
// }