

use bevy::prelude::Component;

#[derive( Default,Component)]
pub struct StatComponent<T>(pub T);