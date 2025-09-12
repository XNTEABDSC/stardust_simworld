use bevy_ecs::bundle::Bundle;

use crate::statistic_physics::matters_bundle::MattersBundle;



#[derive(Bundle,Default)]
pub struct GridGasBundle<const DIM:usize>{
    pub matters:MattersBundle<DIM>,
}