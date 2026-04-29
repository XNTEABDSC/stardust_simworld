use bevy::{app::App, ecs::{schedule::IntoScheduleConfigs, system::{Query, Res}}};
use frunk::{HList, HNil};
use physics_basic::stats::TimePass;
use wacky_bag_bevy::{stat_component::stat::Stat, system::processing_system::ScheduleConfigsProcessing};

use crate::{schedule::schedule_pre_sim, simulate_speed::simulate_speed::SimulateSpeed};



pub fn set_simulate_speed_even<Num>(mut query:Query<(&mut Stat<TimePass<Num>>,)>, global_speed:Res<SimulateSpeed<Num>>)
	where Num:Sync+Send+'static+Copy
{
	query.par_iter_mut().for_each(|(mut a,)|{
		a.0.0=global_speed.second_per_frame;
	});
} 

pub fn set_simulate_speed_even_plugin<Num>(app:&mut App)
	where Num:Sync+Send+'static+Copy
{
	app.add_systems(schedule_pre_sim(), set_simulate_speed_even::<Num>.into_configs()
		.config_processing::<
			HList!(SimulateSpeed<Num>),
			HNil,
			HList!(Stat<TimePass<Num>>)
		>());
}