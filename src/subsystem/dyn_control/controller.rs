use std::{fmt::Debug, sync::{Arc, Mutex}};

use bevy::ecs::{component::Component, entity::{Entity, EntityHash}, world::EntityMutExcept};
use dashmap::DashMap;

use crate::subsystem::dyn_control::{dyn_object::DynObject, env::DynEnv};

// pub type EntityObject=(Entity,Option<DynObject>);

pub trait ControllerLeafToRoot {
	fn leaf_to_root_send   (&mut self,env:&mut DynEnv,me:Entity)->Option<DynObject>;
	fn leaf_to_root_receive(&self,env:&DynEnv,me:Entity,leaf_data:(Entity,Option<DynObject>));
}

pub trait ControllerProcess {
	fn process     (&mut self,env:&mut DynEnv,me:Entity,others:EntityMutExcept<(DynController,DynEnv)>);
}

pub trait ControllerRootToLeaf {
	fn root_to_leaf_send   (&mut self,env:&mut DynEnv,me:Entity)->Box<dyn FnMut(Entity)->Option<DynObject>>;
	fn root_to_leaf_receive_send(&mut self,env:&mut DynEnv,me:Entity,root_data:(Entity,Option<DynObject>))->Box<dyn FnMut(Entity)->Option<DynObject>>;
}

pub trait Controller:Debug +ControllerLeafToRoot+ControllerProcess+ControllerRootToLeaf {

}
#[derive(Component)]
// pub struct DynController(pub Arc<Mutex<dyn Controller+Send+Sync>>);

pub struct DynController(pub Box<dyn Controller+Send+Sync>);

