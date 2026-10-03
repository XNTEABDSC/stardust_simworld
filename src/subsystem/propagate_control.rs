use std::{fmt::Debug, marker::PhantomData, sync::{Arc, Mutex}};

use bevy::{app::App, ecs::{component::Component, entity::Entity, query::{QueryItem, ROQueryItem}, schedule::{IntoScheduleConfigs, SystemSet}, system::{Query, SystemParamItem}, world::{EntityMutExcept, EntityRefExcept}}};
use wacky_bag_bevy::{system::propagate_relationship::{PropagateLeafToRootMut, PropagateLeafToRootMutApplySysParam, PropagateLeafToRootMutFromSysParam, PropagateRootToLeafMut, PropagateRootToLeafMutBeginSysParam, PropagateRootToLeafMutProcessSysParam, propagate_leaf_to_root_mut, propagate_root_to_leaf_mut}, utils::system_param_with_query::{SystemParamWithQuery, SystemParamWithQueryMergeT, SystemParamWithQueryT}};
use wacky_bag_hlist::impl_phantom;

use crate::{multi_body::attach::AttachTo, schedule::schedule_sim};
#[derive(Debug,Component)]
pub struct DynController<DynAny>(pub Arc<Mutex<dyn PropagateControl<DynAny>+Send+Sync>>);
pub trait PropagateControl<DynAny>:Debug {
	fn leaf_to_root_send   (&mut self,entity:EntityMutExcept<()>)->DynAny;
	fn leaf_to_root_receive(&mut self,entity:EntityRefExcept<()>,leaf_data:(Entity,DynAny));
	fn process             (&mut self,entity:EntityMutExcept<()>);
	fn root_to_leaf_send   (&mut self,entity:EntityMutExcept<()>)->Box<dyn FnMut(Entity)->DynAny>;
	fn root_to_leaf_receive_send(&mut self,entity:EntityMutExcept<()>,root_data:(Entity,DynAny))->Box<dyn FnMut(Entity)->DynAny>;
}

#[derive(Debug)]
pub struct PropagateControlLeafToRoot<DynAny>((Entity,DynAny));

#[derive(Debug)]
pub struct PropagateControlRootToLeaf<DynAny>((Entity,DynAny));

impl<DynAny:'static> PropagateLeafToRootMut<AttachTo> for PropagateControlLeafToRoot<DynAny> {
	type FromSysParam=SystemParamWithQueryT<
		(
			EntityMutExcept<'static,'static,()>,
			&'static mut DynController<DynAny>,
			Entity
		),(),()
	>;

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
		Self( (values.1.2,values.1.1.0.lock().unwrap().leaf_to_root_send(values.1.0)) )
	}

	type ApplySysParam=SystemParamWithQueryT<
		(
			EntityRefExcept<'static,'static,()>,
			&'static mut DynController<DynAny>,
		),(),()
	>;

	fn apply_to_data(
			self,
			values:
				ROQueryItem<
					<SystemParamWithQueryMergeT<PropagateLeafToRootMutApplySysParam<AttachTo>, Self::ApplySysParam> as SystemParamWithQuery>::D
				>,
			//SystemParamWithQueryROItem<'w,'s,SPQMerge<PropagateLeafToRootApplySysParam<R>, Self::ApplySysParam>>,
			_others:
				&SystemParamItem<
					<SystemParamWithQueryMergeT<PropagateLeafToRootMutApplySysParam<AttachTo>, Self::ApplySysParam> as SystemParamWithQuery>::P
				>
		) {
		values.1.1.0.lock().unwrap().leaf_to_root_receive(values.1.0, self.0);
		// values.1.1.0.leaf_receive(&mut values.1.0.0, values.0.2, self.0);
	}
}

impl<DynAny:'static> PropagateRootToLeafMut<AttachTo> for PropagateControlRootToLeaf<DynAny> {
	type BeginSysParam=SystemParamWithQueryT<
		(
			EntityMutExcept<'static,'static,()>,
			&'static mut DynController<DynAny>,
			Entity
		),(),()
	>;

	fn process_data_begin(
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
		let mut f=values.1.1.0.lock().unwrap().root_to_leaf_send(values.1.0);
		move|e| Self((values.1.2,f(e)))
	}

	type ProcessSysParam=SystemParamWithQueryT<
		(
			EntityMutExcept<'static,'static,()>,
			&'static mut DynController<DynAny>,
			Entity
		),(),()
	>;

	fn process_data<'w,'s>(
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
		let mut f=values.1.1.0.lock().unwrap().root_to_leaf_receive_send(values.1.0,self.0);
		move|e| Self((values.1.2,f(e)))
	}
}

pub fn control_process<DynAny:'static>(mut q:Query<(EntityMutExcept<'static,'static,()>,&'static mut DynController<DynAny>)>){
	q.par_iter_mut().for_each(|(e,c)|{
		c.0.lock().unwrap().process(e);
	});
}
#[derive(SystemSet)]
pub struct ControlSystems<DynAny>(PhantomData<DynAny>);

impl_phantom!(ControlSystems<DynAny>);

pub fn plugin<DynAny:'static+Send+Sync>(app:&mut App) {
	app.add_systems(schedule_sim(), (
		propagate_leaf_to_root_mut::<PropagateControlLeafToRoot<DynAny>,AttachTo>,
		control_process::<DynAny>,
		propagate_root_to_leaf_mut::<PropagateControlRootToLeaf<DynAny>,AttachTo>
	).chain().in_set(ControlSystems::<DynAny>::default()));
}