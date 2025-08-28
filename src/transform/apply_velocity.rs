use bevy_ecs::system::Query;
use physics_basic::stats::{Agv, DirVec, Pos, Vel};

use crate::components::{change_component::ChangeComponent, stat_component::StatComponent};


pub fn apply_velocity<const DIM:usize>(mut query:Query<(&ChangeComponent<Pos<DIM>>,&StatComponent<Vel<DIM>>)>)
{
    (&mut query).par_iter_mut().for_each(|(value,delta)|{
        value.add_change(Pos(delta.0.0));
    });
}

pub fn apply_angular_velocity(mut query:Query<(&ChangeComponent<Dir>,&StatComponent<Agv>)>)
{
    (&mut query).par_iter_mut().for_each(|(value,delta)|{
        value.add_change(Dir(delta.0.0));
    });
}
