use std::sync::{Arc, Mutex};

use bevy::ecs::component::Component;

use crate::subsystem::dyn_object::DynObject;


#[derive(Debug,Default,Component)]
pub struct DynEnv(pub Arc<Mutex<Option<DynObject>>>);