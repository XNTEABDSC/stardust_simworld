use std::ops::Deref;

use bevy_ecs_macros::Component;
use physics_basic::num::Num;



/// How much time does this entity passes this frame
/// 
/// Why not
#[derive(Component)]
pub struct TimePerFrame(pub Num);

// impl TimePerFrame {
// 	/// How much time does this entity passes this frame
// 	/// 
// 	/// By this function we can decide whether const or special
// 	pub fn time(&self)->impl Deref<Target = Num>{&self.0}
// }