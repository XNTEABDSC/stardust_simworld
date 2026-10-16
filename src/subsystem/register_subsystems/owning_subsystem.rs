use std::{any::Any, ops::DerefMut, sync::{Arc, Mutex}};

use bevy::{app::App, ecs::{component::Component, entity::Entity, schedule::{IntoScheduleConfigs, SystemSet}, system::Query}, log::error, };
use dashmap::DashMap;
use wacky_bag_bevy::message::{message_on_entity::{MessageOnEntity, MessagesOnEntity, MessagesOnEntityRes, MessagesOnEntityWriter}, owned_message::apply_owned_message_parallel};

use crate::schedule::schedule_apply_change;



pub struct OwningSubsystem{
    pub entity:Entity,
    pub subsystem:Arc<Mutex<dyn Any+Send+Sync>>,
}
#[derive(Component)]
pub struct OwningSubsystems{
    subsystems:DashMap<String,OwningSubsystem>,
}

pub struct RegisterSubsystem{
    pub sys:Option<OwningSubsystem>,
    pub name:String,
}

pub type RegisterSubsystemMessage=MessageOnEntity<RegisterSubsystem>;
pub type RegisterSubsystemMessages=MessagesOnEntity<RegisterSubsystem>;
pub type RegisterSubsystemMessagesRes=MessagesOnEntityRes<RegisterSubsystem>;
pub type RegisterSubsystemMessagesWriter<'a>=MessagesOnEntityWriter<'a,RegisterSubsystem>;

pub fn register_subsystem(mut q:Query<(&mut RegisterSubsystemMessages,&mut OwningSubsystems,Entity)>){
    q.par_iter_mut().for_each(|(mut ms,s,e)|{
        ms.deref_mut().par_for_each(&|m|{
            match m.sys {
                Some(sys) => {
                    if s.subsystems.contains_key(&m.name){
                        error!("{}'s subsystem {} should not be registered multiple time",e,m.name);
                    }
                    s.subsystems.insert(m.name,sys);
                },
                None => {
                    if !s.subsystems.contains_key(&m.name){
                        error!("{}'s subsystem {} should not exist when tried to remove",e,m.name);
                    }
                    s.subsystems.remove(&m.name);
                },
            }
        });
    });
}

#[derive(Debug,Default,Hash,PartialEq,Eq,SystemSet,Clone, Copy)]
pub struct RegisterSubsysteSystemSet;

pub fn plugin(app:&mut App) {
    app.init_resource::<RegisterSubsystemMessagesRes>();
    app.add_systems(schedule_apply_change(), (apply_owned_message_parallel::<RegisterSubsystemMessage>,register_subsystem).chain().in_set(RegisterSubsysteSystemSet));
}