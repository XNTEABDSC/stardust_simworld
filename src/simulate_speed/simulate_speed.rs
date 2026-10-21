
use bevy::{ecs::resource::Resource, reflect::Reflect};
use nalgebra::RealField;
use bevy::prelude::ReflectResource;
// use physics_basic::num::Num;


#[derive(Resource,Clone,Reflect)]
#[reflect(Resource)]
pub struct SimulateSpeed<Num>{
    pub second_per_frame:Num,
    pub frame_per_second:Num,
}

impl<Num:RealField+Copy> SimulateSpeed<Num> {
	pub fn from_fps(fps:Num)->Self{
		Self{
			frame_per_second:fps,
			second_per_frame:Num::one()/fps
		}
	}
}