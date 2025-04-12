

use std::{mem, sync::Mutex};

use bevy::prelude::Component;

#[derive(Component,Debug,Default)]
pub struct ChangeComponent<T>(Mutex<T>)
    where T:Default+std::ops::AddAssign<T>;

impl<T> ChangeComponent<T>
where T:Default+std::ops::AddAssign<T>{
    pub fn add_change(&self,change:T){
        loop {
            let b=self.0.try_lock();
            match b {
                Ok(mut v) => {*v+=change;drop(v);break;},
                Err(_) => {},
            }
        }
    }
    pub fn get_and_reset(&mut self)->T{
        let mut b=self.0.lock().unwrap();
        mem::replace(&mut b,Default::default())
    }
}