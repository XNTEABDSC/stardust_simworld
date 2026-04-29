
use bevy::{app::App, ecs::system::Query};
use nalgebra::RealField;
use physics_basic::stats::{ Pos, Vel};
use wacky_bag_bevy::stat_component::{change::Change, stat::Stat};

use crate::schedule::schedule_sim;




pub fn apply_velocity<Num:RealField+Copy,const DIM:usize>(query:Query<(&Change<Pos<Num,DIM>>,&Stat<Vel<Num,DIM>>)>)
{
    query.par_iter().for_each(|(value,delta)|{
        value.add_change(Pos(delta.0.0));
    });
}

pub fn plugin<Num:RealField+Copy,const DIM:usize>(app:&mut App){
	app.add_systems(schedule_sim(), apply_velocity::<Num,DIM>);
}