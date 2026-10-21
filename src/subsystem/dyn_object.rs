use std::{any::Any, borrow::Cow, collections::HashMap, };

use derive_more::From;

use crate::subsystem::dyn_object::DynObject::{IMap, IVec, SMap};


#[derive(Debug,From)]
pub enum DynObject{
	Int(i64),
	Float(f64),
	Str(Cow<'static,str>),
	Box(Box<dyn Any+Send+Sync>),
	// ArcMutex(Arc<Mutex<dyn Any+Send+Sync>>),
	// WeakMutex(Weak<Mutex<dyn Any>>),
	SMap(HashMap<Cow<'static,str>,DynObject>),
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

	/// [`downcast_ref`] for [`Box`]
	/// 
	/// Does not consider other enums that matches T. If you want, use [`Self::flat_box`] then try_into()
	pub fn downcast_ref<T:'static>(&self)->Option<&T>{
		match self {
			Self::Box(v)=>v.downcast_ref::<T>(),
			_=>None
		}
	}
	/// [`downcast_mut`] for [`Box`]
	/// 
	/// Does not consider other enums that matches T. If you want, use [`Self::flat_box`] then try_into()
	pub fn downcast_mut<T:'static>(&mut self)->Option<&mut T>{
		match self {
			Self::Box(v)=>v.downcast_mut::<T>(),
			_=>None
		}
	}

	/// try to flatten self if is Box and can be represented by other basic enums
	pub fn flat_box(self)->Self{
		if let Self::Box(b)=self {
			if let Some(v) = b.downcast_ref::<i64>() {
				return Self::Int(*v);
			}else if let Some(v) = b.downcast_ref::<f64>() {
				return Self::Float(*v);
			}

			return 
			b.downcast::<Cow<'static,str>>()
			.map(|v|Self::Str(*v))
			.or_else(|b|b.downcast::<HashMap<Cow<'static,str>,DynObject>>()
				.map(|v|Self::SMap(*v))
			).or_else(|b|b.downcast::<HashMap<i64,DynObject>>()
				.map(|v|Self::IMap(*v))
			).or_else(|b|b.downcast::<Vec<DynObject>>()
				.map(|v|Self::IVec(*v))
			)
			.unwrap_or_else(|v|Self::Box(v));
		} else {
			self
		}
	}
}
impl From<String> for DynObject {
	fn from(value: String) -> Self {
		Self::Str(value.into())
	}
}
impl From<&'static str> for DynObject {
	fn from(value: &'static str) -> Self {
		Self::Str(value.into())
	}
}
impl TryFrom<DynObject> for i64 {
	type Error=DynObject;

	fn try_from(value: DynObject) -> Result<Self, Self::Error> {
		match value {
			DynObject::Int(v) => Ok(v),
			_=>Err(value)
		}
	}
}
impl TryFrom<DynObject> for f64 {
	type Error=DynObject;

	fn try_from(value: DynObject) -> Result<Self, Self::Error> {
		match value {
			DynObject::Float(v) => Ok(v),
			_=>Err(value)
		}
	}
}
impl TryFrom<DynObject> for Cow<'static,str> {
	type Error=DynObject;

	fn try_from(value: DynObject) -> Result<Self, Self::Error> {
		match value {
			DynObject::Str(v) => Ok(v),
			_=>Err(value)
		}
	}
}
impl TryFrom<DynObject> for Box<dyn Any+Send+Sync> {
	type Error=DynObject;

	fn try_from(value: DynObject) -> Result<Self, Self::Error> {
		match value {
			DynObject::Box(v) => Ok(v),
			_=>Err(value)
		}
	}
}
impl TryFrom<DynObject> for HashMap<Cow<'static,str>,DynObject> {
	type Error=DynObject;

	fn try_from(value: DynObject) -> Result<Self, Self::Error> {
		match value {
			DynObject::SMap(v) => Ok(v),
			_=>Err(value)
		}
	}
}
impl TryFrom<DynObject> for HashMap<i64,DynObject> {
	type Error=DynObject;

	fn try_from(value: DynObject) -> Result<Self, Self::Error> {
		match value {
			DynObject::IMap(v) => Ok(v),
			_=>Err(value)
		}
	}
}
impl TryFrom<DynObject> for Vec<DynObject> {
	type Error=DynObject;

	fn try_from(value: DynObject) -> Result<Self, Self::Error> {
		match value {
			DynObject::IVec(v) => Ok(v),
			_=>Err(value)
		}
	}
}

pub enum Iter<'a> {
	SMap(<&'a HashMap<Cow<'static,str>,DynObject> as IntoIterator>::IntoIter),
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
	SMap(<&'a mut HashMap<Cow<'static,str>,DynObject> as IntoIterator>::IntoIter),
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
