use std::{borrow::Cow, collections::VecDeque, fmt::{Debug, Display}};

use derive_more::{From};


#[derive(Debug,Hash,PartialEq, Eq,From,Clone)]
pub enum DynKey {
    Int(isize),
    Str(Cow<'static,str>),
    Vec(VecDeque<DynKey>)
}

impl DynKey {
    /// append 2 DynKey, merge as 1 VecDeque if possible
    pub fn append(self,suffix:DynKey)->DynKey{
        if let DynKey::Vec(mut items)=self{
            if let DynKey::Vec(mut items2)=suffix {
                items.append(&mut items2);
                DynKey::Vec(items)
            }else {
                items.push_back(suffix);
                DynKey::Vec(items)
            }
        }else {
            if let DynKey::Vec(mut items)=suffix {
                items.push_front(self);
                DynKey::Vec(items)
            }else {
                DynKey::Vec(VecDeque::from_iter([self,suffix]))
            }
        }
    }
}

impl Display for DynKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(i)=>{
                <_ as Display>::fmt(i,f)
            },
            Self::Str(s)=>{
                <_ as Display>::fmt(s,f)
            },
            Self::Vec(v)=>{
                <_ as Debug>::fmt(v,f)
            }
        }
    }
}