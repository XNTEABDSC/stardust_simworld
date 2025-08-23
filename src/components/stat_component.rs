

use std::ops::Deref;

use bevy::prelude::Component;

#[derive( Default,Component)]
pub struct StatComponent<T>(pub T);

impl<T> Deref for StatComponent<T> {
    type Target=T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}