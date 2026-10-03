use std::{any::Any, fmt::Debug};

use bevy::ecs::{entity::Entity, system::Query, world::{EntityMutExcept, EntityRef}};

use crate::subsystem::DynAny;

// pub trait ControllerMut{
// 	fn process_mut(&mut self, entity:EntityMutExcept<()>, data:DynAny);
// }

pub trait MessageController {
	fn receive_msg(&self, entity:EntityRef, data:DynAny);
}

pub fn process_message_control(q:Query<>){
	
}