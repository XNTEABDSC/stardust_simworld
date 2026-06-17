use std::{mem::swap, ops::{AddAssign, ControlFlow::{Break, Continue}, DerefMut}};

use bevy::{ecs::{entity::{Entity, EntityHash}, query::{QueryData, ROQueryItem, Without}, relationship::{Relationship, RelationshipTarget}, system::{Local, ParamSet, Query}}, tasks::{ComputeTaskPool, TaskPool}, utils::Parallel};
use dashmap::DashMap;
use num_traits::Zero;
use wacky_bag_bevy::{stat_component::change::Change, system::propagate_relationship::{PropagateChangeLeafToRoot, propagate_leaf_to_root}};

use crate::multi_body::attach::{AttachTo, AttachedEntities};

pub fn change_spread<T>(
	ps:ParamSet<(
		Query<(&Change<T>,&AttachTo),Without<AttachedEntities>>,
		Query<(&Change<T>,&AttachedEntities,Option<&AttachTo>)>,
	)>,
	update_sources_set:
	Local<DashMap<Entity,usize,EntityHash>>,
	//Local<Parallel<EntityHashSet>>, 
	//EntityHash 
	//DashSet<Entity,EntityHash>>,
	// mut update_sources:Local<Parallel<Vec<Entity>>>,
	update_tasks:Local<(Parallel<Vec<(Entity,PropagateChangeLeafToRoot<T>)>>,Parallel<Vec<(Entity,PropagateChangeLeafToRoot<T>)>>)>,
	// mut update_tasks_cache:Local<Parallel<Vec<(Entity,T)>>>,
)
	where T:Send+Sync+AddAssign+'static+Zero
{
	propagate_leaf_to_root::<PropagateChangeLeafToRoot<T>,AttachTo>(ps, update_sources_set, update_tasks);
}