use wacky_bag_bevy::message::message_on_entity::{MessagesOnEntity, MessagesOnEntityRes};

use crate::subsystem::DynAny;


// pub type AnyMsg=Box<dyn Any>;

// pub fn put_messages<T>(mr:MessageReader<>)

// #[derive(Bundle)]
pub type AnyMsgC = MessagesOnEntity<DynAny>;

pub type AnyMsgRes = MessagesOnEntityRes<DynAny>;