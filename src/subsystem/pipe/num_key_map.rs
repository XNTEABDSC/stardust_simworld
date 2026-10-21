use std::hash::Hash;

use bevy::platform::collections::HashMap;


#[derive(Debug)]
pub struct NumKeyMap<Key:Hash+PartialEq+Eq+Send+Sync+'static+Clone>{
    // count:usize,
    num_to_key:Vec<Key>,
    key_to_num:HashMap<Key,usize>,
}

impl<Key: Hash + PartialEq + Eq + Send + Sync + 'static+Clone> Default for NumKeyMap<Key> {
    fn default() -> Self {
        Self { num_to_key: Default::default(), key_to_num: Default::default() /*, new_keys: Default::default() */ }
    }
}

impl<Key:Hash+PartialEq+Eq+Send+Sync+'static+Clone> NumKeyMap<Key> {
    pub fn new()->Self{
        Self::default()
    }

    pub fn push(&mut self,key:Key)->usize{
        let id=self.num_to_key.len();
        self.num_to_key.push(key.clone());
        self.key_to_num.insert(key, id);
        return id;
    }

    pub fn push_no_map(&mut self,key:Key)->usize{
        let id=self.num_to_key.len();
        self.num_to_key.push(key.clone());
        return id;
    }

    pub fn num_to_key(&self)->&Vec<Key>{
        // self.num_to_key.get(num)
        &self.num_to_key
    }

    pub fn key_to_num(&self)->&HashMap<Key,usize>{
        // self.key_to_num.get(key)
        &self.key_to_num
    }

    // pub fn drain_new_keys(&mut self)->impl Iterator<Item = (usize,Key)>{
    //     ( (self.num_to_key.len()-self.new_keys.len()) .. self.num_to_key.len()).into_iter().zip(
    //         self.new_keys.drain(..)
    //     )
    // }

    pub fn len(&self)->usize{self.num_to_key.len()}
}

