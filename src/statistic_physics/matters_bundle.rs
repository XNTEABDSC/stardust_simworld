
use bevy::ecs::bundle::Bundle;

// use crate::components::{change_component::ChangeComponent, determining_components::DeterminingComponent, stat_component::StatComponent};


use physics_basic::stats::*;

use statistic_physics::stats::*;
use wacky_bag_bevy::stat_component::{determining::Determining, stat::Stat, change::Change};



#[derive(Bundle,Default)]
pub struct MattersBundle<const DIM:usize>{
    pub mass:Stat<Mass>,
    pub momentum:Stat<Momentum<DIM>>,
    pub energy:Stat<Energy>,
    pub kinetic:Stat<Kinetic>,
    pub internal:Stat<Internal>,
    pub vel_var_sq:Stat<VelVarSq>,
    pub vel_var:Stat<VelVar>,
    pub vel_var_sq1_dir:Stat<VelVarSq1Dir>,
    pub vel_var1_dir:Stat<VelVar1Dir>,
    pub mass_determining:Determining<Mass>,
    pub energy_determining:Determining<Energy>,
    pub momentum_determining:Determining<Momentum<DIM>>,
    pub mass_change:Change<Mass>,
    pub momentum_change:Change<Momentum<DIM>>,
    pub energy_change:Change<Energy>
}