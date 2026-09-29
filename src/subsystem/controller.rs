use std::{fmt::Debug, sync::{Arc, Mutex}};

use bevy::ecs::{component::Component, entity::{Entity, EntityHash}};
use dashmap::DashMap;

use crate::subsystem::dyn_object::DynObject;

// pub type EntityObject=(Entity,Option<DynObject>);

pub trait Controller:Debug {
	fn leaf_to_root_send   (&mut self,env:&mut Option<DynObject>,me:Entity)->Option<DynObject>;
	fn leaf_to_root_receive(&mut self,env:&mut Option<DynObject>,me:Entity,leaf_data:(Entity,Option<DynObject>));
	fn process     (&mut self,env:&mut Option<DynObject>,me:Entity);

	fn root_to_leaf_send   (&mut self,env:&mut Option<DynObject>,me:Entity)->Box<dyn FnMut(Entity)->Option<DynObject>>;
	fn root_to_leaf_receive_send(&mut self,env:&mut Option<DynObject>,me:Entity,root_data:(Entity,Option<DynObject>))->Box<dyn FnMut(Entity)->Option<DynObject>>;
}
#[derive(Debug,Component)]
pub struct DynController(pub Arc<Mutex<dyn Controller+Send+Sync>>);

