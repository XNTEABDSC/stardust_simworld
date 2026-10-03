use std::{any::Any, collections::{BTreeMap, HashMap}, fmt::Display, mem, ops::{Deref, Index}, sync::{Arc, Mutex}};

use bevy::ecs::component::Component;
use derive_more::From;

use crate::subsystem::dyn_control::dyn_object::DynObject::{IMap, IVec, SMap};

// #[derive(Debug,Clone)]
// pub enum DynBasic {
// 	Int(i64),
// 	Float(f32),
// 	Str(String),
// }

// impl Display for DynObject {
// 	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// 		match self {
// 			DynObject::Int(a) => a.fmt(f),
// 			DynObject::Float(a) => a.fmt(f),
// 			DynObject::Str(a) => a.fmt(f),

// 		}
// 	}
// }

// #[derive(Debug,Clone)]
// pub enum DynMap {
// 	StrMap(HashMap<String,DynObject>),
// 	IntMap(HashMap<i64,DynObject>),
// 	FMap(BTreeMap<f32,DynObject>),
// 	Vec(Vec<DynObject>),
// }



#[derive(Debug,Clone,From)]
pub enum DynObject{
	Int(i64),
	Float(f32),
	Str(String),
	// Box(Box<dyn Any>),
	ArcMutex(Arc<Mutex<dyn Any+Send+Sync>>),
	// WeakMutex(Weak<Mutex<dyn Any>>),
	SMap(HashMap<String,DynObject>),
	IMap(HashMap<i64,DynObject>),
	IVec(Vec<DynObject>)
}

impl DynObject {
	pub fn table_get(&self,i:&DynObject)->Option<&DynObject> {
		match self {
			SMap(m)=>{
				match i {
					DynObject::Str (s)=>{
						m.get(s)
					},
					_=>None
				}
			},
			IMap(m)=>{
				match i {
					DynObject::Int (s)=>{
						m.get(s)
					},
					_=>None
				}
			},
			IVec(v)=>{
				match i {
					DynObject::Int (s)=>{
						v.get(*s as usize)
					},
					_=>None
				}
			}
			_=>None
		}
	}

	pub fn table_get_mut(&mut self,i:&DynObject)->Option<&mut DynObject> {
		match self {
			SMap(m)=>{
				match i {
					DynObject::Str (s)=>{
						m.get_mut(s)
					},
					_=>None
				}
			},
			IMap(m)=>{
				match i {
					DynObject::Int (s)=>{
						m.get_mut(s)
					},
					_=>None
				}
			},
			IVec(v)=>{
				match i {
					DynObject::Int (s)=>{
						v.get_mut(*s as usize)
					},
					_=>None
				}
			}
			_=>None
		}
	}

	pub fn table_insert(&mut self,k:DynObject,v:DynObject)->Option<DynObject>{
		match self {
			SMap(m)=>{
				match k {
					DynObject::Str (k)=>{
						m.insert(k,v)
					},
					_=>None
				}
			},
			IMap(m)=>{
				match k {
					DynObject::Int (k)=>{
						m.insert(k,v)
					},
					_=>None
				}
			},
			IVec(m)=>{
				match k {
					DynObject::Int (k)=>{
						m.insert(k as usize, v);
						None
					},
					_=>None
				}
			}
			_=>None
		}
	}

	pub fn table_remove(&mut self,k:&DynObject)->Option<DynObject>{
		match self {
			SMap(m)=>{
				match k {
					DynObject::Str (k)=>{
						m.remove(k)
					},
					_=>None
				}
			},
			IMap(m)=>{
				match k {
					DynObject::Int (k)=>{
						m.remove(k)
					},
					_=>None
				}
			},
			IVec(m)=>{
				match k {
					DynObject::Int (k)=>{
						m.remove(*k as usize);
						None
					},
					_=>None
				}
			}
			_=>None
		}
	}

	pub fn table_iter<'a>(&'a self)->Option<Iter<'a>>{
		match self {
			SMap(m) => Some(Iter::SMap(m.iter())),
			IMap(m) => Some(Iter::IMap(m.iter())),
			IVec(m) => Some(Iter::IVec(m.iter().enumerate())),
			_=>None
		}
	}

	pub fn table_iter_mut<'a>(&'a mut self)->Option<IterMut<'a>>{
		match self {
			SMap(m) => Some(IterMut::SMap(m.iter_mut())),
			IMap(m) => Some(IterMut::IMap(m.iter_mut())),
			IVec(m) => Some(IterMut::IVec(m.iter_mut().enumerate())),
			_=>None
		}
	}
}

pub enum Iter<'a> {
	SMap(<&'a HashMap<String,DynObject> as IntoIterator>::IntoIter),
	IMap(<&'a HashMap<i64,DynObject> as IntoIterator>::IntoIter),
	IVec(std::iter::Enumerate< <&'a Vec<DynObject> as IntoIterator>::IntoIter >),
}

impl<'a> Iterator for Iter<'a> {
	type Item=(DynObject,&'a DynObject);

	fn next(&mut self) -> Option<Self::Item> {
		match self {
			Iter::SMap(i) => i.next().map(|(k,v)|( k.clone().into(),v )),
			Iter::IMap(i) => i.next().map(|(k,v)|( (*k).into(),v )),
			Iter::IVec(i) => i.next().map(|(k,v)|( (k as i64).into(),v )),
		}
	}
}

pub enum IterMut<'a> {
	SMap(<&'a mut HashMap<String,DynObject> as IntoIterator>::IntoIter),
	IMap(<&'a mut HashMap<i64,DynObject> as IntoIterator>::IntoIter),
	IVec(std::iter::Enumerate< <&'a mut Vec<DynObject> as IntoIterator>::IntoIter >),
}

impl<'a> Iterator for IterMut<'a> {
	type Item=(DynObject,&'a mut DynObject);

	fn next(&mut self) -> Option<Self::Item> {
		match self {
			IterMut::SMap(i) => i.next().map(|(k,v)|( k.clone().into(),v )),
			IterMut::IMap(i) => i.next().map(|(k,v)|( (*k).into(),v )),
			IterMut::IVec(i) => i.next().map(|(k,v)|( (k as i64).into(),v )),
		}
	}
}
