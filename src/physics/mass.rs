use bevy::prelude::Component;

use statistic_physics::num::Num;

#[derive(Component,Debug,Default)]
pub struct Mass(pub Num);



wacky_bag::derive_add_traits!(Mass);
/*
wacky_bag::derive_add_traits!(VelVarSq);
wacky_bag::derive_add_traits!(VelVar);
wacky_bag::derive_add_traits!(VelVarSq1Dir);
wacky_bag::derive_add_traits!(VelVar1Dir);
 */