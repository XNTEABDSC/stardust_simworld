use bevy::prelude::Component;

use statistic_physics::{num::Num,vec2_fix::Vec2Fix};


#[derive(Component,Debug,Default)]
pub struct Momentum(pub Vec2Fix);


#[derive(Component,Debug,Default)]
pub struct Energy(pub Num);

#[derive(Component,Debug,Default)]
pub struct Kinetic(pub Num);
#[derive(Component,Debug,Default)]
pub struct Internal(pub Num);
#[derive(Component,Debug,Default)]
pub struct VelVarSq(pub Num);
#[derive(Component,Debug,Default)]
pub struct VelVar(pub Num);
#[derive(Component,Debug,Default)]
pub struct VelVarSq1Dir(pub Num);
#[derive(Component,Debug,Default)]
pub struct VelVar1Dir(pub Num);

wacky_bag::derive_add_traits!(Momentum);
wacky_bag::derive_add_traits!(Energy);
wacky_bag::derive_add_traits!(Kinetic);
wacky_bag::derive_add_traits!(Internal);
/*
wacky_bag::derive_add_traits!(VelVarSq);
wacky_bag::derive_add_traits!(VelVar);
wacky_bag::derive_add_traits!(VelVarSq1Dir);
wacky_bag::derive_add_traits!(VelVar1Dir);
 */