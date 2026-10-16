use std::{ops::DerefMut, sync::Arc};

use bevy::{app::App, ecs::{component::Component, entity::{Entity, EntityHash}, query::{QueryItem, ROQueryItem}, schedule::{IntoScheduleConfigs, SystemSet}, system::{Query, SystemParamItem}, world::EntityMutExcept}};
use wacky_bag_bevy::{system::propagate_relationship::{PropagateLeafToRootMut, PropagateLeafToRootMutApplySysParam, PropagateLeafToRootMutFromSysParam, PropagateRootToLeafMut, PropagateRootToLeafMutBeginSysParam, PropagateRootToLeafMutProcessSysParam, propagate_leaf_to_root_mut, propagate_root_to_leaf_mut}, utils::system_param_with_query::{SystemParamWithQuery, SystemParamWithQueryMergeT, SystemParamWithQueryT}};

use crate::{multi_body::attach::AttachTo, schedule::schedule_sim, subsystem::dyn_control::{controller::DynController, dyn_object::DynObject, env::DynEnv}};


#[derive(Debug)]
pub struct PropagateControlLeafToRoot((Entity,Option<DynObject>));

#[derive(Debug)]
pub struct PropagateControlRootToLeaf((Entity,Option<DynObject>));

impl PropagateLeafToRootMut<AttachTo> for PropagateControlLeafToRoot {
	type FromSysParam=SystemParamWithQueryT<
		(
			&'static mut DynEnv,
			&'static mut DynController,
			Entity
		),(),()
	>;

	fn from_data(
			mut values: 
				QueryItem<
					<SystemParamWithQueryMergeT<PropagateLeafToRootMutFromSysParam<AttachTo>, Self::FromSysParam> as SystemParamWithQuery>::D
				>,
			_others: 
				&SystemParamItem<
					<SystemParamWithQueryMergeT<PropagateLeafToRootMutFromSysParam<AttachTo>, Self::FromSysParam> as SystemParamWithQuery>::P
				>
		)->Self {
		Self( (values.1.2,values.1.1.0.leaf_to_root_send(&mut values.1.0,values.1.2)) )
	}

	type ApplySysParam=SystemParamWithQueryT<
		(
			&'static mut DynEnv,
			&'static mut DynController
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
		values.1.1.0.leaf_to_root_receive(values.1.0, values.0.2, self.0);
		// values.1.1.0.leaf_receive(&mut values.1.0.0, values.0.2, self.0);
	}
}

impl PropagateRootToLeafMut<AttachTo> for PropagateControlRootToLeaf {
	type BeginSysParam=SystemParamWithQueryT<
		(
			&'static mut DynEnv,
			&'static mut DynController,
			Entity
		),(),()
	>;

	fn process_data_begin(
			mut values: 
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
		let mut f=values.1.1.0.root_to_leaf_send(values.1.0.deref_mut(), values.1.2);
		move|e| Self((values.1.2,f(e)))
	}

	type ProcessSysParam=SystemParamWithQueryT<
		(
			&'static mut DynEnv,
			&'static mut DynController,
			Entity
		),(),()
	>;

	fn process_data<'w,'s>(
			self,
			mut values: 
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
		let mut f=values.1.1.0.root_to_leaf_receive_send(values.1.0.deref_mut(), values.1.2,self.0);
		move|e| Self((values.1.2,f(e)))
	}
}

pub fn control_process(mut q:Query<(&mut DynEnv, &mut DynController, Entity, EntityMutExcept<(DynController,DynEnv)>)>){
	q.par_iter_mut().for_each(|(mut e,mut c,me,em)|{
		c.0.process(e.deref_mut(), me,em);
	});
}
#[derive(Debug,Default,Hash,PartialEq, Eq, Clone, Copy,SystemSet)]
pub struct ControlSystems;

pub fn plugin(app:&mut App) {
	app.add_systems(schedule_sim(), (
		propagate_leaf_to_root_mut::<PropagateControlLeafToRoot,AttachTo>,
		control_process,
		propagate_root_to_leaf_mut::<PropagateControlRootToLeaf,AttachTo>
	).chain().in_set(ControlSystems));
}