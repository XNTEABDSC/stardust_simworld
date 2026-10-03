use std::{any::Any, sync::{Arc, Mutex}};

use bevy::ecs::{system::Query, world::{EntityMutExcept, EntityRef}};



pub mod thrust;
pub mod dyn_control;
pub mod momentum_whell;
pub mod msg_control;
pub mod propagate_control;

pub type DynAny=Box<dyn Any+Send+Sync>;