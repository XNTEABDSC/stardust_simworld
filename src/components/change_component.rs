

use std::{mem, sync::Mutex};

use bevy::prelude::Component;

#[derive(Component,Debug,Default)]
pub struct ChangeComponent<T>(pub Mutex<T>);

impl<T> ChangeComponent<T>{
    pub fn add_change<T2>(&self,change:T2)
        where T:std::ops::AddAssign<T2>
    {
        *self.0.lock().unwrap()+=change;
        
    }
    pub fn get_and_reset(&mut self)->T
        where T:Default
    {
        let mut b=self.0.lock().unwrap();
        mem::replace(&mut b,Default::default())
    }
}

