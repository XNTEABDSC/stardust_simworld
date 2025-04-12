use bevy_ecs::bundle::Bundle;

use crate::{components::{change::ChangeComponent, determining_components::DeterminingComponent}, physics::mass::Mass, statistic_physics::matters::{Energy, Momentum}};

use super::matters::{Internal, Kinetic, VelVar, VelVar1Dir, VelVarSq, VelVarSq1Dir};



#[derive(Bundle,Default)]
pub struct MattersBundle{
    pub mass:Mass,
    pub momentum:Momentum,
    pub energy:Energy,
    pub kinetic:Kinetic,
    pub internal:Internal,
    pub vel_var_sq:VelVarSq,
    pub vel_var:VelVar,
    pub vel_var_sq1_dir:VelVarSq1Dir,
    pub vel_var1_dir:VelVar1Dir,
    pub mass_determining:DeterminingComponent<Mass>,
    pub energy_determining:DeterminingComponent<Energy>,
    pub momentum_determining:DeterminingComponent<Momentum>,
    pub mass_change:ChangeComponent<Mass>,
    pub momentum_change:ChangeComponent<Momentum>,
    pub energy_change:ChangeComponent<Energy>
}