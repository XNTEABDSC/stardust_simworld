use std::{any::Any, collections::HashMap, ops::{Deref, DerefMut}, sync::Arc};


// pub type AnyOnbject=Arc<>

pub trait ControllerInterface {
    fn get(&self,req:&str)->Option<&dyn Any>;

    fn get_t<T:'static>(&self,req:&str)->Option<&T>{
        self.get(req).and_then(|a|a.downcast_ref())
    }

    fn get_mut(&mut self,req:&str)->Option<&mut dyn Any>;

    fn get_t_mut<T:'static>(&mut self,req:&str)->Option<&mut T>{
        self.get_mut(req).and_then(|a|a.downcast_mut())
    }

    fn set(&mut self,req:&str,v:Box<dyn Any>);

}

pub struct Controller{
    pub v:HashMap<String,Box<dyn Any>>
}

impl ControllerInterface for Controller {
    fn get(&self,req:&str)->Option<&dyn Any> {
        return self.v.get(req).map(|v|v.deref());
    }

    fn get_mut(&mut self,req:&str)->Option<&mut dyn Any> {
        return self.v.get_mut(req).map(|v|v.deref_mut());
    }

    fn set(&mut self,req:&str,v:Box<dyn Any>) {
        self.v.insert(req.to_owned(), v);
    }
}