use bevy::{app::{App, FixedPreUpdate}, ecs::{query::With, schedule::IntoScheduleConfigs, system::Query}};
use frunk::{HList, HNil, hlist};
use nalgebra::RealField;
use statistic_physics::formulas::calculate_matters_state_inplace;



use physics_basic::stats::*;

use statistic_physics::stats::*;
use wacky_bag_bevy::{stat_component::{determining::Determining, stat::Stat}, system::processing_system::ScheduleConfigsProcessing};



pub fn calculate_states<Num:RealField+Copy,const DIM:usize>(mut query:Query<
    (&Stat< Mass<Num>>,&Stat<Momentum<Num,DIM>>,&Stat<Energy<Num>>,&mut Stat<Vel<Num,DIM>>,&mut Stat<Kinetic<Num>>,&mut Stat<Internal<Num>>,&mut Stat<VelVarSq<Num>>,&mut Stat<VelVar<Num>>,&mut Stat<VelVarSq1Dir<Num>>,&mut Stat<VelVar1Dir<Num>>)
    ,(With<Determining<Mass<Num>>>,With<Determining<Momentum<Num,DIM>>>,With<Determining<Energy<Num>>>)>){
    query.par_iter_mut().for_each(
        |(mass,momentum,energy,mut vel,mut kinetic,mut internal,mut vel_var_sq,mut vel_var,mut vel_var_sq_1dir,mut vel_var_1dir)|{
            calculate_matters_state_inplace(hlist!(&mass.0,&momentum.0,&energy.0,&mut vel.0,&mut kinetic.0,&mut internal.0,&mut vel_var_sq.0,&mut vel_var.0,&mut vel_var_sq_1dir.0,&mut vel_var_1dir.0));
    });
}

pub fn calculate_states_plugin<Num:RealField+Copy,const DIM:usize>(app:&mut App){
	app.add_systems(FixedPreUpdate, calculate_states::<Num,DIM>.into_configs()
		.config_processing::<
			HList!(Stat<Mass<Num>>,Stat<Momentum<Num,DIM>>,Stat<Energy<Num>>),
			HNil,
			HList!(Stat<Vel<Num,DIM>>,Stat<Kinetic<Num>>,Stat<Internal<Num>>,Stat<VelVarSq<Num>>,Stat<VelVar<Num>>,Stat<VelVarSq1Dir<Num>>,Stat<VelVar1Dir<Num>>)>()
	);
}