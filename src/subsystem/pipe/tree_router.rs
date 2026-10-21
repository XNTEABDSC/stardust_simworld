use std::{collections::HashMap, hash::Hash, mem, ops::{Deref, DerefMut}, sync::{Arc, Mutex}};

use bevy::{app::App, ecs::{bundle::Bundle, entity::Entity, query::{QueryItem, ROQueryItem}, schedule::{IntoScheduleConfigs, ScheduleConfigs, SystemSet}, system::{ScheduleSystem, SystemParamItem}}, log::error, reflect::ty};
use wacky_bag_bevy::{component::{CAM, Comp}, system::propagate_relationship::{PropagateLeafToRootMut, PropagateLeafToRootMutApplySysParam, PropagateLeafToRootMutFromSysParam, PropagateRootToLeafMut, PropagateRootToLeafMutBeginSysParam, PropagateRootToLeafMutProcessSysParam, propagate_leaf_to_root_mut, propagate_root_to_leaf_mut}, utils::system_param_with_query::{SystemParamWithQuery, SystemParamWithQueryMergeT, SystemParamWithQueryT}};

use crate::{multi_body::attach::{AttachName, AttachTo}, schedule::schedule_sim, subsystem::{dyn_object::DynObject, pipe::dyn_key::DynKey as Key}};

pub trait MapLike {
    type Key;
    type Value;
    fn new_with_capacity(capacity:usize)->Self;
    fn insert(&mut self,k:Self::Key,v:Self::Value);
    fn reserves(&mut self,capacity:usize);
    fn drain(&mut self)->impl Iterator<Item = (Self::Key,Self::Value)>;
    fn insert_batch_self(&mut self,b:&mut Self);
    fn take(&mut self,idx:&Self::Key)->Option<Self::Value>;
    fn get(&self,idx:&Self::Key)->Option<&Self::Value>;
    fn get_mut(&mut self,idx:&Self::Key)->Option<&mut Self::Value>;
}

impl<T> MapLike for Vec<Option<T>> {
    type Key=usize;

    type Value=T;

    fn new_with_capacity(capacity:usize)->Self {
        (0..capacity).into_iter().map(|_|None).collect()
    }

    fn insert(&mut self,k:Self::Key,v:Self::Value) {
        self.reserve(k);
        if self.len()==k {
            self.push(Some(v));
        }else {
            self[k]=Some(v);
        }
    }

    fn reserves(&mut self,capacity:usize) {
        while self.len()<capacity {
            self.push(None);
        }
    }

    fn drain(&mut self)->impl Iterator<Item = (Self::Key,Self::Value)>{
        self.into_iter().filter_map(|a|a.take()).enumerate()
    }

    fn insert_batch_self(&mut self,b:&mut Self){
        // self.append(b);
        assert!(self.len()>=b.len());
        b.iter_mut().zip(self.iter_mut()).for_each(|(b,a)|*a=b.take());
    }
    
    fn take(&mut self,idx:&Self::Key)->Option<Self::Value> {
        if self.len()<=*idx {
            None
        }else {
            self[*idx].take()
        }
    }
    
    fn get(&self,idx:&Self::Key)->Option<&Self::Value> {
        <[_]>::get(&self, *idx).and_then(|v|v.as_ref())
    }
    
    fn get_mut(&mut self,idx:&Self::Key)->Option<&mut Self::Value> {
        <[_]>::get_mut(self, *idx).and_then(|v|v.as_mut())
    }
}

impl<K:Eq+Hash,V> MapLike for HashMap<K,V>{
    type Key=K;

    type Value=V;

    fn new_with_capacity(_capacity:usize)->Self {
        HashMap::new()
    }

    fn insert(&mut self,k:Self::Key,v:Self::Value) {
        // self.insert(k, v);
        HashMap::insert(self, k, v);
    }

    fn drain(&mut self)->impl Iterator<Item = (Self::Key,Self::Value)>{
        self.drain()
    }

    fn reserves(&mut self,_capacity:usize) {
        
    }

    fn insert_batch_self(&mut self,b:&mut Self) {
        self.extend(b.drain());
    }
    
    fn take(&mut self,idx:&Self::Key)->Option<Self::Value> {
        self.remove(idx)
    }
    
    fn get(&self,idx:&Self::Key)->Option<&Self::Value> {
        self.get(idx)
    }
    
    fn get_mut(&mut self,idx:&Self::Key)->Option<&mut Self::Value> {
        self.get_mut(idx)
    }
}

#[derive(Debug,Default)]
pub struct TreeRouterKeys{
    
    leaf_map:HashMap<Entity,Vec<usize>>,
    key_map:HashMap<Key,usize>,
    new_keys:Vec<(usize,Key)>,
}


impl TreeRouterKeys
{
    pub fn len(&self)->usize{self.key_map.len()}

    pub fn recv_leaf_keys(&mut self,leaf_entity:Entity,leaf_name:Key,keys:impl IntoIterator<Item = (usize,Key)>){
        let lm=self.leaf_map.entry(leaf_entity).or_default();
        let key_map=&mut self.key_map;
        let new_keys=&mut self.new_keys;
        keys.into_iter().for_each(move |(leaf_idx,leaf_key)|{
            let idx=key_map.len();
            let new_key=leaf_name.clone().append(leaf_key);
            if key_map.contains_key(&new_key) {
                error!("key {new_key} already exist");
            }
            key_map.insert(new_key.clone(),idx);
            new_keys.push((idx,new_key));

            if lm.len()!=leaf_idx {
                error!("received {} should be eq to newly incoming {} but not, means that the key order from leaf is not sequential", lm.len(),leaf_idx);
            }
            while lm.len()<leaf_idx {
                lm.push(usize::MAX);
            }
            if lm.len()>leaf_idx {
                lm[leaf_idx]=idx;
            }else {
                lm.push(idx);
            }
            
        });
    }

    pub fn key(&mut self, key:Key)->usize{
        let idx=self.key_map.len();
        * self.key_map.entry(key.clone()).or_insert_with(||{
            self.new_keys.push((idx,key));
            return idx;
        })
    }

    pub fn drain_new_keys(&mut self)->Vec<(usize,Key)>{
        mem::replace(&mut self.new_keys, Vec::new())
    }

    pub fn leaf_map(&self)->&HashMap<Entity,Vec<usize>>{&self.leaf_map}

    pub fn leaf_map_at(&mut self,e:Entity)->&Vec<usize>{
        self.leaf_map.entry(e).or_default()
    }
    
    pub fn key_map(&self) -> &HashMap<Key,usize> {
        &self.key_map
    }

}

#[derive(Debug,Default)]
pub struct TreeRouter<
    // Data:Send+Sync+'static,
    C:MapLike<Key = usize>
>{
    container:Option<C>,
    keys:TreeRouterKeys,
    buffer_leafs:HashMap<Entity,Option<C>>,
    // leaf_map:HashMap<Entity,Vec<usize>>,
    // key_map:HashMap<Key,usize>,
    // new_keys:Vec<(usize,Key)>,
}

struct TreeRouterPropagateLeafToRootData<
    C:MapLike<Key = usize>
>
{
    new_keys:Vec<(usize,Key)>,
    data:Option<C>,
}

struct TreeRouterPropagateRootToLeafData<
    C:MapLike<Key = usize>
>
{
    data:C,
}


impl<T,C> TreeRouter<C>
where C:MapLike<Key = usize,Value = T>,
{
    pub fn len(&self)->usize{self.keys.len()}

    pub fn container(&self)->&Option<C>{&self.container}
    pub fn keys(&self)->&TreeRouterKeys{&self.keys}

    pub fn container_mut(&mut self)->&mut C{
        let len=self.len();
        Self::_container_mut(&mut self.container,len)
    }

    fn _container_mut(data:&mut Option<C>,len:usize)->&mut C{
        let c=data.get_or_insert(C::new_with_capacity(len));
        c.reserves(len);
        c
    }

    fn reserve_container(&mut self){
        let len=self.len();
        let c=self.container.get_or_insert(C::new_with_capacity(len));
        c.reserves(len);
    }

    fn recv_leaf_keys(&mut self,src_entity:Entity,src_name:Key,keys:impl IntoIterator<Item = (usize,Key)>){
        self.keys.recv_leaf_keys(src_entity, src_name, keys);
        self.reserve_container();
    }

    pub fn key(&mut self, key:Key)->usize{
        let idx=self.keys.key(key);
        self.reserve_container();
        idx
    }

    fn recv_leaf_data(&mut self,src_entity:Entity,mut data:C){
        
        let len=self.keys.len();
        let lm=self.keys.leaf_map_at(src_entity);
        let container=Self::_container_mut(&mut self.container,len);

        data.drain().for_each(|(leaf_idx,d)|{
            let Some(idx)=lm.get(leaf_idx) else{
                error!("received unregisterred idx {leaf_idx}");
                return;
            };
            container.insert(*idx, d);
        });
        
        self.buffer_leafs.insert(src_entity, Some(data));
    }
    
    fn recv_data(&mut self,mut data:C){
        if self.container.is_none(){
            self.container=Some(data);
        }else {
            self.container_mut().insert_batch_self(&mut data);
        }
    }

    pub fn insert(&mut self,idx:usize,d:T){
        self.container_mut().insert(idx, d);
    }

    pub fn insert_by_key(&mut self,key:&Key,d:T)->usize{
        let idx=if let Some(idx)=self.keys.key_map().get(key){
            *idx
        }else{
            self.key(key.clone())
        };
        self.container_mut().insert(idx, d);
        return idx;
    }

    fn send_root(&mut self)->TreeRouterPropagateLeafToRootData<C>{
        TreeRouterPropagateLeafToRootData { new_keys: self.keys.drain_new_keys(), data: self.container.take() }
    }
    
    fn send_leaf(&mut self, e:Entity)->TreeRouterPropagateRootToLeafData<C>{
        let len=self.keys.len();
        let mut buf=self.buffer_leafs.get_mut(&e).map_or_else(||C::new_with_capacity(len),|v|v.take().unwrap_or_else(||C::new_with_capacity(len)));
        let container=Self::_container_mut(&mut self.container,len);
        self.keys.leaf_map_at(e).iter().enumerate().for_each(|(lidx,idx)|{
            if let Some(v)=container.take(idx){
                buf.insert(lidx, v);
            }
        });

        TreeRouterPropagateRootToLeafData 
        { 
            data: buf
        }
    }

    pub fn get(&self,idx:usize)->Option<&T>{
        self.container.as_ref().and_then(|c|c.get(&idx))
    }
    pub fn get_mut(&mut self,idx:usize)->Option<&mut T>{
        self.container.as_mut().and_then(|c|c.get_mut(&idx))
    }

    pub fn take(&mut self,idx:usize)->Option<T>{
        self.container.as_mut().and_then(|c|c.take(&idx))
    }

}

struct TreeRouterPropagateLeafToRoot
<
    C:MapLike<Key = usize>
>
{
    data:TreeRouterPropagateLeafToRootData<C>,
    name:Key,
    entity:Entity,
}

impl<
    C:MapLike<Key = usize,Value = T>+Send+Sync+'static,
    T
    // T:Send+Sync+'static
> PropagateLeafToRootMut<AttachTo> for TreeRouterPropagateLeafToRoot<C> {
    type FromSysParam=SystemParamWithQueryT<
        (&'static CAM<TreeRouter<C>>,&'static AttachName,Entity),
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
        let s=values.1.0.lock().unwrap().send_root();
        Self { data:s, name: Key::Str(values.1.1.0.as_str().to_owned().into()),entity:values.1.2 }
    }

    type ApplySysParam=SystemParamWithQueryT<
        (&'static CAM<TreeRouter<C>>,),
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
        d.recv_leaf_keys(self.entity,self.name, self.data.new_keys);
        if let Some(data)=self.data.data{
            d.recv_leaf_data(self.entity, data);
        }
    }
}

struct TreeRouterPropagateRootToLeaf
<
    C:MapLike<Key = usize>
>
{
    data:TreeRouterPropagateRootToLeafData<C>,
}

impl<C> PropagateRootToLeafMut<AttachTo> for TreeRouterPropagateRootToLeaf<C> 
where C:MapLike<Key = usize>+Send+Sync+'static
{
    type BeginSysParam=SystemParamWithQueryT<
        (&'static CAM<TreeRouter<C>>,),
        (),
        ()
    >;

    fn send_begin(
            values: 
                QueryItem<
                    <SystemParamWithQueryMergeT<PropagateRootToLeafMutBeginSysParam<AttachTo>, Self::BeginSysParam> as SystemParamWithQuery>::D
                >,
            _others: 
                &SystemParamItem<
                    <SystemParamWithQueryMergeT<PropagateRootToLeafMutBeginSysParam<AttachTo>, Self::BeginSysParam> as SystemParamWithQuery>::P
                >
        )->
        // Vec<(Entity,Self)>
        impl FnMut(Entity)->Self
         {
        return |e|Self { data: values.1.0.0.lock().unwrap().send_leaf(e) };
    }

    type ProcessSysParam=SystemParamWithQueryT<
        (&'static CAM<TreeRouter<C>>,),
        (),
        ()
    >;

    fn send_process<'w,'s>(
            self,
            values: 
                QueryItem<
                    <SystemParamWithQueryMergeT<PropagateRootToLeafMutProcessSysParam<AttachTo>, Self::ProcessSysParam> as SystemParamWithQuery>::D
                >,
            _others: 
                &SystemParamItem<
                    <SystemParamWithQueryMergeT<PropagateRootToLeafMutProcessSysParam<AttachTo>, Self::ProcessSysParam> as SystemParamWithQuery>::P
                >)
        ->
        // Vec<(Entity,Self)>
        impl FnMut(Entity)->Self
        {
        values.1.0.lock().unwrap().recv_data(self.data.data);
        return |e|Self { data: values.1.0.0.lock().unwrap().send_leaf(e) };
    }
    
    fn send_end<'w,'s>(
            self,
            values: 
                QueryItem<
                    <SystemParamWithQueryMergeT<PropagateRootToLeafMutProcessSysParam<AttachTo>, Self::ProcessSysParam> as SystemParamWithQuery>::D
                >,
            others: 
                &SystemParamItem<
                    <SystemParamWithQueryMergeT<PropagateRootToLeafMutProcessSysParam<AttachTo>, Self::ProcessSysParam> as SystemParamWithQuery>::P
                >)
        ->() {
        values.1.0.lock().unwrap().recv_data(self.data.data);
    }
    
}

// pub type TreeRouterVec<T>=TreeRouter<Vec<Option<T>>>;

// #[derive(QueryData)]
// pub struct TreeRouterVec<'w,T>
// where T:Send+Sync+'static
// {
//     tree_router: &'w CAM::<TreeRouter<Vec<Option<T>>>>
// }

// impl<'w,T:Send+Sync+'static> TreeRouterVec<'w,T> {
//     pub fn len(&self)->usize{self.tree_router.lock().unwrap().len()}
//     pub fn get_idx_by_key(&self)->&HashMap<Key,usize>{self.tree_router.lock().unwrap()}
//     pub fn insert_by_key(&self,key:&Key,d:T)->usize{self.tree_router.lock().unwrap().insert_by_key(key, d)}
//     pub fn insert_by_idx(&self,idx:usize,d:T){self.tree_router.lock().unwrap().insert_by_idx(idx,d)}
//     pub fn get_mut_by_idx(&self,idx:usize)->Option<impl Deref<Target = Option<T>>>{
//         let guard=self.tree_router.lock().unwrap();
//         if guard.container.as_deref().is_some_and(|v|v.len()>idx) {
//             Some(DerefMutDelegate::<_,_,_,Option<T>>{
//                 guard,
//                 fr:move |guard: &std::sync::MutexGuard<'_, TreeRouter<Vec<Option<T>>>>|{
//                     let g=&*(guard.deref());
//                     let c=&g.container;
//                     &c.as_ref().unwrap()[idx]
//                 },
//                 fm:move |guard: &mut std::sync::MutexGuard<'_, TreeRouter<Vec<Option<T>>>>|{
//                     let g=&mut *(guard.deref_mut());
//                     let c=&mut g.container;
//                     &mut c.as_mut().unwrap()[idx]
//                 },
//             })
//         }else {
//             None
//         }
//     }
//     pub fn len(&self)->usize{self.tree_router.lock().unwrap().len()}
//     pub fn len(&self)->usize{self.tree_router.lock().unwrap().len()}
// }

// pub fn bundle<C>()->impl Bundle
// where C:MapLike<Key = usize>+Send+Sync+'static+Default
// {
//     (
//         CAM::<TreeRouter<C>>::default(),
//     )
// }
#[derive(SystemSet,Debug,Default,Hash,PartialEq, Eq, Clone, Copy)]
pub struct TreeRouterPropagateLeafToRootSystemSet;
#[derive(SystemSet,Debug,Default,Hash,PartialEq, Eq, Clone, Copy)]
pub struct TreeRouterPropagateRootToLeafSystemSet;

pub type TreeRouterVec=TreeRouter<Vec<Option<DynObject>>>;
pub type TreeRouterMap=TreeRouter<HashMap<usize,DynObject>>;

pub type TreeRouterVecComp=CAM<TreeRouterVec>;
pub type TreeRouterMapComp=CAM<TreeRouterMap>;

pub fn propagate_systems<C>()->ScheduleConfigs<ScheduleSystem>
where C:MapLike<Key = usize>+Send+Sync+'static
{
    let lr_v=propagate_leaf_to_root_mut::<TreeRouterPropagateLeafToRoot<C>,_>.into_configs();
    // let lr_m=propagate_leaf_to_root_mut::<TreeRouterPropagateLeafToRoot<HashMap<usize,T> >,_>.into_configs();
    
    let rl_v=propagate_root_to_leaf_mut::<TreeRouterPropagateRootToLeaf<C>,_>.into_configs();
    // let rl_m=propagate_root_to_leaf_mut::<TreeRouterPropagateRootToLeaf<HashMap<usize,T> >,_>.into_configs();

    return (
        (lr_v).in_set(TreeRouterPropagateLeafToRootSystemSet),
        (rl_v).in_set(TreeRouterPropagateRootToLeafSystemSet)
    ).into_configs();
}

pub fn plugin_c<C>(app:&mut App)
where C:MapLike<Key = usize>+Send+Sync+'static
{
    app.add_systems(schedule_sim(), propagate_systems::<C>());
}

pub fn plugin_t<T>(app:&mut App)
where T:Send+Sync+'static
{
    app.add_plugins((
        plugin_c::<Vec<Option<T>>>,
        plugin_c::<HashMap<usize,T>>
    ));
}

pub fn plugin(app:&mut App){
    app.add_plugins(plugin_t::<DynObject>);
}

pub fn bundle_t<T:Send+Sync+'static>()->impl Bundle{
    (
        Comp(Arc::new(Mutex::new(TreeRouterVec::default()))),
        Comp(Arc::new(Mutex::new(TreeRouterMap::default()))),
    )
}

pub fn bundle()->impl Bundle{
    bundle_t::<DynObject>()
}