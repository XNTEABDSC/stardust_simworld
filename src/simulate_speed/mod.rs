use bevy::app::{PluginGroup, PluginGroupBuilder};

use crate::simulate_speed::{set_simulate_speed_even::set_simulate_speed_even_plugin, simulate_speed::SimulateSpeed};

pub mod simulate_speed;
pub mod set_simulate_speed_even;



pub struct SimulateSpeedPlugin<Num>{
	pub global_speed:SimulateSpeed<Num>
}

impl<Num> PluginGroup for SimulateSpeedPlugin<Num> 
	where Num:Copy+Send+Sync+'static
{
	fn build(self) -> PluginGroupBuilder {
		PluginGroupBuilder::start::<Self>()
			.add(move |app: &mut bevy::app::App|{app.insert_resource(self.global_speed.clone());})
			.add(set_simulate_speed_even_plugin::<Num>)
	}
}