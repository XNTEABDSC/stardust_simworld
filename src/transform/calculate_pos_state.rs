use bevy_ecs::{query::With, system::Query};

use crate::components::{change_component::ChangeComponent, determining_components::DeterminingComponent, stat_component::StatComponent};

use super::tramsform::{Pos, Vel};


/*
pub fn determining_pos_apply_vel(mut query:Query<(&mut StatComponent< Pos >,& StatComponent<Vel>),With<DeterminingComponent<Pos>>>){
    (&mut query).par_iter_mut().for_each(|(mut value,delta)|{
        value.0.0+=delta.0.0;

    });
}
 */

pub fn add_vel_to_pos_change(mut query:Query<(&mut ChangeComponent<Pos>,&StatComponent<Vel>)>){
    (&mut query).par_iter_mut().for_each(|(mut cpos,vel)|{
        //(*cpos).add_change()
        //(*cpos).add_change()
    })
}