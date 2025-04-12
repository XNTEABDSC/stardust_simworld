use bevy_ecs::{query::With, system::Query};
use statistic_physics::{constants::NUMINV2, num::Num};

use crate::{components::determining_components::DeterminingComponent, physics::mass::Mass, transform::tramsform::Vel};

use super::matters::{Energy, Internal, Kinetic, Momentum, VelVar, VelVar1Dir, VelVarSq, VelVarSq1Dir};



pub fn calculate_states(mut query:Query<
    (&Mass,&Momentum,&Energy,&mut Vel,&mut Kinetic,&mut Internal,&mut VelVarSq,&mut VelVar,&mut VelVarSq1Dir,&mut VelVar1Dir)
    ,(With<DeterminingComponent<Mass>>,With<DeterminingComponent<Momentum>>,With<DeterminingComponent<Energy>>)>){
    query.iter_mut().for_each(
        |(mass,momentum,energy,mut vel,mut kinetic,mut internal,mut vel_var_sq,mut vel_var,mut vel_var_sq_1dir,mut vel_var_1dir)|{
            vel.0=momentum.0/mass.0;
            kinetic.0=(momentum.0*momentum.0/mass.0) *NUMINV2;
            internal.0=energy.0-kinetic.0;
            vel_var_sq.0=2*internal.0/mass.0;
            vel_var.0=Num::sqrt(vel_var_sq.0);
            vel_var_sq_1dir.0=vel_var_sq.0*NUMINV2;
            vel_var_1dir.0=vel_var.0*Num::FRAC_1_SQRT_2;
    });
}