use bevy_ecs::{query::With, system::Query};
use statistic_physics::{constants::NUMINV2, formulas::calculate_matters_state, num::Num};

use crate::{components::{determining_components::DeterminingComponent, stat_component::StatComponent}, transform::tramsform::Vel};

use physics_basic::stats::*;

use statistic_physics::stats::*;



pub fn calculate_states(mut query:Query<
    (&StatComponent< Mass>,&StatComponent<Momentum>,&StatComponent<Energy>,&mut StatComponent<Vel>,&mut StatComponent<Kinetic>,&mut StatComponent<Internal>,&mut StatComponent<VelVarSq>,&mut StatComponent<VelVar>,&mut StatComponent<VelVarSq1Dir>,&mut StatComponent<VelVar1Dir>)
    ,(With<DeterminingComponent<Mass>>,With<DeterminingComponent<Momentum>>,With<DeterminingComponent<Energy>>)>){
    query.iter_mut().for_each(
        |(mass,momentum,energy,mut vel,mut kinetic,mut internal,mut vel_var_sq,mut vel_var,mut vel_var_sq_1dir,mut vel_var_1dir)|{
            calculate_matters_state((&mass.0,&momentum.0,&energy.0,&mut vel.0,&mut kinetic.0,&mut internal.0,&mut vel_var_sq.0,&mut vel_var.0,&mut vel_var_sq_1dir.0,&mut vel_var_1dir.0))
    });
}