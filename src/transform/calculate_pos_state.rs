use bevy_ecs::{query::With, system::Query};

use crate::components::determining_components::DeterminingComponent;

use super::tramsform::{Pos, Vel};



pub fn determining_pos_apply_vel(mut query:Query<(&mut Pos,&Vel),With<DeterminingComponent<Pos>>>){
    for (mut value,delta) in &mut query {
        value.0+=delta.0;
    }
}
