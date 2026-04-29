// use bevy::app::Plugins;

use bevy::app::{PluginGroup, PluginGroupBuilder};
use nalgebra::RealField;

use crate::grid::{at_grid::entity_at_grid_plugin, grid::GridData};

pub mod grid;
pub mod at_grid;
//pub mod grid_plugin;

pub struct GridPlugins<Num,const DIM:usize>{
	pub grid_data:GridData<Num,DIM>
}

impl<Num,const DIM:usize> PluginGroup for GridPlugins<Num,DIM> 
	where Num:RealField+Copy
{
	fn build(self) -> PluginGroupBuilder {
		PluginGroupBuilder::start::<Self>()
			.add(move |app: &mut bevy::app::App|{app.insert_resource(self.grid_data.clone());})
			.add(entity_at_grid_plugin::<Num,DIM>)
	}
}