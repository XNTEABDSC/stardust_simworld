
use bevy::ecs::system::Query;
use physics_basic::stats::{ Pos, Vel};
use wacky_bag_bevy::stat_component::{change::Change, stat::Stat};




pub fn apply_velocity<const DIM:usize>(mut query:Query<(&Change<Pos<DIM>>,&Stat<Vel<DIM>>)>)
{
    (&mut query).par_iter_mut().for_each(|(value,delta)|{
        value.add_change(Pos(delta.0.0));
    });
}

