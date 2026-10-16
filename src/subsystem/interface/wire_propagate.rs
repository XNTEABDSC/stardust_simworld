use std::collections::HashMap;

use bevy::{ecs::{entity::Entity, query::{QueryItem, ROQueryItem}, system::SystemParamItem}};
use wacky_bag_bevy::{component::CAM, system::propagate_relationship::{PropagateLeafToRootMut, PropagateLeafToRootMutApplySysParam, PropagateLeafToRootMutFromSysParam}, utils::system_param_with_query::{SystemParamWithQuery, SystemParamWithQueryMergeT, SystemParamWithQueryT}};

use crate::{multi_body::attach::{AttachName, AttachTo}, subsystem::interface::{dyn_key::DynKey as Key, num_key_map::NumKeyMap}};

#[derive(Debug,Default)]
pub struct WirePropagateLeafToRootStore<
    Data:Send+Sync+'static,
>
{
    data_vec:Vec<Option<Data>>,
    vec_num_num_map:NumKeyMap<Option<(Entity,usize)>>,
    vec_num_key_map:NumKeyMap<Key>,
    vec_last_key_update_index:usize,

    data_map:HashMap<usize,Data>,
    map_num_num_map:NumKeyMap<Option<(Entity,usize)>>,
    map_num_key_map:NumKeyMap<Key>,
    map_last_key_update_index:usize,
}

pub struct WirePropagateData<
    Data:Send+Sync+'static,
>
{
    new_keys:Vec<(usize,Key,bool)>,
    vec_data:Vec<Option<Data>>,
    map_data:Vec<(usize,Data)>,
}

impl<
    Data:Send+Sync+'static,
>   WirePropagateLeafToRootStore<Data>
{
    pub fn recv_propagate_new_keys(&mut self,src_entity:Entity,src_name:Key,keys:impl IntoIterator<Item = (usize,Key,bool)>){
        keys.into_iter().for_each(|k|{
            if k.2 {
                self.vec_num_num_map.push(Some((src_entity,k.0)));
                self.vec_num_key_map.push(src_name.clone().append(k.1));
                self.data_vec.push(None);
            }else {
                self.map_num_num_map.push(Some((src_entity,k.0)));
                self.map_num_key_map.push(src_name.clone().append(k.1));
            }
        });
    }

    pub fn recv_key(&mut self, key:Key, frequent_update:bool){
        if frequent_update {
            self.vec_num_num_map.push_no_map(None);
            self.vec_num_key_map.push(key);
            self.data_vec.push(None);
        }else {
            self.map_num_num_map.push_no_map(None);
            self.map_num_key_map.push(key);
        }
    }
    
    pub fn recv_propagate_data(&mut self,src_entity:Entity,data_vec:impl IntoIterator<Item = Option<Data>>,data_map:impl IntoIterator<Item = (usize,Data)>){
        data_vec.into_iter().enumerate().for_each(|(i,d)|{
            let Some(idx)=self.vec_num_num_map.key_to_num().get(&Some((src_entity,i))) else{return;};
            self.data_vec[*idx]=d;
        });
        data_map.into_iter().for_each(|(i,d)|{
            let Some(idx)=self.map_num_num_map.key_to_num().get(&Some((src_entity,i))) else{return;};
            self.data_map.insert(*idx, d);
        });
    }

    pub fn recv_data(&mut self,key:&Key,frequent_update:bool,data:Data){
        if frequent_update{
            let Some(idx)=self.vec_num_key_map.key_to_num().get(key) else{return;};
            self.data_vec[*idx]=Some(data);
        }
        else {
            let Some(idx)=self.map_num_key_map.key_to_num().get(key) else{return;};
            self.data_map.insert(*idx, data);
        }
        
    }
    
    pub fn send(&mut self)->WirePropagateData<Data>{
        let vec_new_keys=
        ( self.vec_last_key_update_index..self.vec_num_key_map.len()).into_iter().zip(self.vec_num_key_map.num_to_key().iter().skip(self.vec_last_key_update_index).map(|a|a.clone()))
        ;

        self.vec_last_key_update_index=self.vec_num_key_map.len();

        
        let map_new_keys=
        ( self.map_last_key_update_index..self.map_num_key_map.len()).into_iter().zip(self.map_num_key_map.num_to_key().iter().skip(self.map_last_key_update_index).map(|a|a.clone()))
        ;
        self.map_last_key_update_index=self.map_num_key_map.len();

        WirePropagateData { 
            // vec_new_keys, 
            vec_data: self.data_vec.iter_mut().map(|v|v.take()).collect(), 
            // map_new_keys: map_new_keys, 
            new_keys: vec_new_keys.map(|(a,b)|(a,b,true)).chain(map_new_keys.map(|(a,b)|(a,b,false))).collect(),
            map_data: self.data_map.drain().collect::<Vec<_>>()
        }
    }
}

pub struct WirePropagateLeafToRoot
<Data:Send+Sync+'static>
{
    data:WirePropagateData<Data>,
    name:Key,
    entity:Entity,
}

impl<
    Data:Send+Sync+'static
> PropagateLeafToRootMut<AttachTo> for WirePropagateLeafToRoot<Data> {
    type FromSysParam=SystemParamWithQueryT<
        (&'static CAM<WirePropagateLeafToRootStore<Data>>,&'static AttachName,Entity),
        (),
        ()
    >
        ;

    fn from_data(
            values: 
                QueryItem<
                    <SystemParamWithQueryMergeT<PropagateLeafToRootMutFromSysParam<AttachTo>, Self::FromSysParam> as SystemParamWithQuery>::D
                >,
            _others: 
                &SystemParamItem<
                    <SystemParamWithQueryMergeT<PropagateLeafToRootMutFromSysParam<AttachTo>, Self::FromSysParam> as SystemParamWithQuery>::P
                >
        )->Self {
        let s=values.1.0.lock().unwrap().send();
        Self { data:s, name: Key::Str(values.1.1.0.as_str().to_owned().into()),entity:values.1.2 }
    }

    type ApplySysParam=SystemParamWithQueryT<
        (&'static CAM<WirePropagateLeafToRootStore<Data>>,),
        (),
        ()
    >;

    fn apply_to_data(
            self,
            values:
                ROQueryItem<
                    <SystemParamWithQueryMergeT<PropagateLeafToRootMutApplySysParam<AttachTo>, Self::ApplySysParam> as SystemParamWithQuery>::D
                >,
            _others:
                &SystemParamItem<
                    <SystemParamWithQueryMergeT<PropagateLeafToRootMutApplySysParam<AttachTo>, Self::ApplySysParam> as SystemParamWithQuery>::P
                >
        ) {
        let mut d=values.1.0.lock().unwrap();
        d.recv_propagate_new_keys(self.entity,self.name, self.data.new_keys);
        d.recv_propagate_data(self.entity, self.data.vec_data,self.data.map_data);
    }
}