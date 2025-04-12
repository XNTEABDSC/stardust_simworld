use bevy::prelude::Component;

use statistic_physics::{num::Num,vec2_fix::Vec2Fix};

#[derive(Component,Debug,Default)]
pub struct Pos(pub Vec2Fix);
#[derive(Component,Debug,Default)]
pub struct Vel(pub Vec2Fix);

#[derive(Component,Debug,Default)]
pub struct Dir(pub Num);
#[derive(Component,Debug,Default)]
pub struct Agv(pub Num);


wacky_bag::derive_add_traits!(Pos);
wacky_bag::derive_add_traits!(Vel);
wacky_bag::derive_add_traits!(Dir);
wacky_bag::derive_add_traits!(Agv);