use bevy::{app::{App, FixedPreUpdate}, ecs::{query::With, schedule::IntoScheduleConfigs, system::Query}};
use frunk::{HList, HNil, hlist};
use statistic_physics::formulas::calculate_matters_state_inplace;



use physics_basic::stats::*;

use statistic_physics::stats::*;
use wacky_bag_bevy::{stat_component::{determining::Determining, stat::Stat}, system::processing_system::ScheduleConfigsProcessing};



pub fn calculate_states<const DIM:usize>(mut query:Query<
    (&Stat< Mass>,&Stat<Momentum<DIM>>,&Stat<Energy>,&mut Stat<Vel<DIM>>,&mut Stat<Kinetic>,&mut Stat<Internal>,&mut Stat<VelVarSq>,&mut Stat<VelVar>,&mut Stat<VelVarSq1Dir>,&mut Stat<VelVar1Dir>)
    ,(With<Determining<Mass>>,With<Determining<Momentum<DIM>>>,With<Determining<Energy>>)>){
    query.par_iter_mut().for_each(
        |(mass,momentum,energy,mut vel,mut kinetic,mut internal,mut vel_var_sq,mut vel_var,mut vel_var_sq_1dir,mut vel_var_1dir)|{
            calculate_matters_state_inplace(hlist!(&mass.0,&momentum.0,&energy.0,&mut vel.0,&mut kinetic.0,&mut internal.0,&mut vel_var_sq.0,&mut vel_var.0,&mut vel_var_sq_1dir.0,&mut vel_var_1dir.0));
    });
}

pub fn calculate_states_plugin<const DIM:usize>(app:&mut App){
	app.add_systems(FixedPreUpdate, calculate_states::<DIM>.into_configs()
		.config_processing::<
			HList!(Stat<Mass>,Stat<Momentum<DIM>>,Stat<Energy>),
			HNil,
			HList!(Stat<Vel<DIM>>,Stat<Kinetic>,Stat<Internal>,Stat<VelVarSq>,Stat<VelVar>,Stat<VelVarSq1Dir>,Stat<VelVar1Dir>)>()
	);
}