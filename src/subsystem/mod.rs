use std::{any::Any, sync::{Arc, Mutex}};



pub mod controller;
pub mod thrust;
pub mod control_systems;
pub mod dyn_object;
pub mod env;

pub type BA=Box<dyn Any>;
pub type AMA=Arc<Mutex<dyn Any>>;

pub trait ControlInterface {
    
}