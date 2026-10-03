use std::any::Any;

use bevy::ecs::{bundle::Bundle, entity::Entity, message::MessageReader};
use wacky_bag_bevy::message::{message_on_entity::{MessageOnEntity, MessagesOnEntity, MessagesOnEntityRes}, owned_message::ParOwnedMessages};

use crate::subsystem::DynAny;


// pub type AnyMsg=Box<dyn Any>;

// pub fn put_messages<T>(mr:MessageReader<>)

// #[derive(Bundle)]
pub type AnyMsgC = MessagesOnEntity<DynAny>;

pub type AnyMsgRes = MessagesOnEntityRes<DynAny>;