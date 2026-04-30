
use bevy::ecs::resource::Resource;
use nalgebra::RealField;
// use physics_basic::num::Num;


#[derive(Resource,Clone)]
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