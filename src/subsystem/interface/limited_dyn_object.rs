use std::collections::HashMap;

use derive_more::From;

use crate::subsystem::interface::limited_dyn_object::LDynObject::{IMap, IVec, SMap};


#[derive(Debug,From)]
pub enum LDynObject{
	Int(i64),
	Float(f32),
	Str(String),
	// ArcMutex(Arc<Mutex<dyn Any+Send+Sync>>),
	// WeakMutex(Weak<Mutex<dyn Any>>),
	SMap(HashMap<String,LDynObject>),
	IMap(HashMap<i64,LDynObject>),
	IVec(Vec<LDynObject>)
}

impl LDynObject {
	pub fn table_get(&self,i:&LDynObject)->Option<&LDynObject> {
		match self {
			SMap(m)=>{
				match i {
					LDynObject::Str (s)=>{
						m.get(s)
					},
					_=>None
				}
			},
			IMap(m)=>{
				match i {
					LDynObject::Int (s)=>{
						m.get(s)
					},
					_=>None
				}
			},
			IVec(v)=>{
				match i {
					LDynObject::Int (s)=>{
						v.get(*s as usize)
					},
					_=>None
				}
			}
			_=>None
		}
	}

	pub fn table_get_mut(&mut self,i:&LDynObject)->Option<&mut LDynObject> {
		match self {
			SMap(m)=>{
				match i {
					LDynObject::Str (s)=>{
						m.get_mut(s)
					},
					_=>None
				}
			},
			IMap(m)=>{
				match i {
					LDynObject::Int (s)=>{
						m.get_mut(s)
					},
					_=>None
				}
			},
			IVec(v)=>{
				match i {
					LDynObject::Int (s)=>{
						v.get_mut(*s as usize)
					},
					_=>None
				}
			}
			_=>None
		}
	}

	pub fn table_insert(&mut self,k:LDynObject,v:LDynObject)->Option<LDynObject>{
		match self {
			SMap(m)=>{
				match k {
					LDynObject::Str (k)=>{
						m.insert(k,v)
					},
					_=>None
				}
			},
			IMap(m)=>{
				match k {
					LDynObject::Int (k)=>{
						m.insert(k,v)
					},
					_=>None
				}
			},
			IVec(m)=>{
				match k {
					LDynObject::Int (k)=>{
						m.insert(k as usize, v);
						None
					},
					_=>None
				}
			}
			_=>None
		}
	}

	pub fn table_remove(&mut self,k:&LDynObject)->Option<LDynObject>{
		match self {
			SMap(m)=>{
				match k {
					LDynObject::Str (k)=>{
						m.remove(k)
					},
					_=>None
				}
			},
			IMap(m)=>{
				match k {
					LDynObject::Int (k)=>{
						m.remove(k)
					},
					_=>None
				}
			},
			IVec(m)=>{
				match k {
					LDynObject::Int (k)=>{
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
	SMap(<&'a HashMap<String,LDynObject> as IntoIterator>::IntoIter),
	IMap(<&'a HashMap<i64,LDynObject> as IntoIterator>::IntoIter),
	IVec(std::iter::Enumerate< <&'a Vec<LDynObject> as IntoIterator>::IntoIter >),
}

impl<'a> Iterator for Iter<'a> {
	type Item=(LDynObject,&'a LDynObject);

	fn next(&mut self) -> Option<Self::Item> {
		match self {
			Iter::SMap(i) => i.next().map(|(k,v)|( k.clone().into(),v )),
			Iter::IMap(i) => i.next().map(|(k,v)|( (*k).into(),v )),
			Iter::IVec(i) => i.next().map(|(k,v)|( (k as i64).into(),v )),
		}
	}
}

pub enum IterMut<'a> {
	SMap(<&'a mut HashMap<String,LDynObject> as IntoIterator>::IntoIter),
	IMap(<&'a mut HashMap<i64,LDynObject> as IntoIterator>::IntoIter),
	IVec(std::iter::Enumerate< <&'a mut Vec<LDynObject> as IntoIterator>::IntoIter >),
}

impl<'a> Iterator for IterMut<'a> {
	type Item=(LDynObject,&'a mut LDynObject);

	fn next(&mut self) -> Option<Self::Item> {
		match self {
			IterMut::SMap(i) => i.next().map(|(k,v)|( k.clone().into(),v )),
			IterMut::IMap(i) => i.next().map(|(k,v)|( (*k).into(),v )),
			IterMut::IVec(i) => i.next().map(|(k,v)|( (k as i64).into(),v )),
		}
	}
}
