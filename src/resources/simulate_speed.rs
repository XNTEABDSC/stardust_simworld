use bevy_ecs::system::Resource;
use physics_basic::num::Num;


#[derive(Resource)]
pub struct SimulateSpeed{
    pub second_per_frame:Num,
    pub frame_per_second:Num,

}