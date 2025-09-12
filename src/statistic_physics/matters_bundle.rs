use bevy_ecs::bundle::Bundle;

use crate::components::{change_component::ChangeComponent, determining_components::DeterminingComponent, stat_component::StatComponent};

use physics_basic::stats::*;

use statistic_physics::stats::*;



#[derive(Bundle,Default)]
pub struct MattersBundle<const DIM:usize>{
    pub mass:StatComponent<Mass>,
    pub momentum:StatComponent<Momentum<DIM>>,
    pub energy:StatComponent<Energy>,
    pub kinetic:StatComponent<Kinetic>,
    pub internal:StatComponent<Internal>,
    pub vel_var_sq:StatComponent<VelVarSq>,
    pub vel_var:StatComponent<VelVar>,
    pub vel_var_sq1_dir:StatComponent<VelVarSq1Dir>,
    pub vel_var1_dir:StatComponent<VelVar1Dir>,
    pub mass_determining:DeterminingComponent<Mass>,
    pub energy_determining:DeterminingComponent<Energy>,
    pub momentum_determining:DeterminingComponent<Momentum<DIM>>,
    pub mass_change:ChangeComponent<Mass>,
    pub momentum_change:ChangeComponent<Momentum<DIM>>,
    pub energy_change:ChangeComponent<Energy>
}