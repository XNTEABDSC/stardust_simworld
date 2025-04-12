
use std::marker::PhantomData;

use bevy::prelude::Component;

#[derive( Default,Component)]
pub struct DeterminingComponent<T>(PhantomData<T>);