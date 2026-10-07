use std::{fmt::Debug, marker::PhantomData, ops::{Deref, DerefMut}, sync::{Arc, Mutex}};

use bevy::{app::App, ecs::{component::{Component, Mutable}, entity::Entity, query::{QueryItem, ROQueryItem}, schedule::{IntoScheduleConfigs, SystemSet}, system::{Query, SystemParamItem}, world::{EntityMutExcept, EntityRefExcept}}, utils::default};
use wacky_bag_bevy::{system::propagate_relationship::{PropagateLeafToRootMut, PropagateLeafToRootMutApplySysParam, PropagateLeafToRootMutFromSysParam, PropagateRootToLeafMut, PropagateRootToLeafMutBeginSysParam, PropagateRootToLeafMutProcessSysParam, propagate_leaf_to_root_mut, propagate_root_to_leaf_mut}, utils::system_param_with_query::{SystemParamWithQuery, SystemParamWithQueryMergeT, SystemParamWithQueryT}};
use wacky_bag_hlist::impl_phantom;

use crate::{multi_body::attach::AttachTo, schedule::schedule_sim};
/// performs leaf_to_root , process, root_to_leaf control in 1 frame
/// 
/// driven by [`plugin`]
pub trait PropagateControl<C,DynAny> {
	fn leaf_to_root_send   (&mut self,entity:EntityMutExcept<()>)->DynAny;
	fn leaf_to_root_receive(&self,    entity:EntityRefExcept<()>,leaf_data:(Entity,DynAny));
	fn root_to_leaf_send   (&mut self,entity:EntityMutExcept<()>)->Box<dyn FnMut(Entity)->DynAny>;
	fn root_to_leaf_receive_send(&mut self,entity:EntityMutExcept<()>,root_data:(Entity,DynAny))->Box<dyn FnMut(Entity)->DynAny>;
}

pub trait ControlProcess<C,DynAny>
where C:Component
{
	fn process(&mut self,entity:EntityMutExcept<(C,)>);
}

#[derive(Debug)]
pub struct PropagateControlLeafToRoot<Controller,DynAny>((Entity,DynAny),PhantomData<Controller>);

#[derive(Debug)]
pub struct PropagateControlRootToLeaf<Controller,DynAny>((Entity,DynAny),PhantomData<Controller>);

impl<Controller,DynAny:'static> PropagateLeafToRootMut<AttachTo> for PropagateControlLeafToRoot<Controller,DynAny>
where Controller:PropagateControl<Controller,DynAny>+Component<Mutability = Mutable>
{
	type FromSysParam=SystemParamWithQueryT<
		(
			EntityMutExcept<'static,'static,(
				Controller,AttachTo
			)>,
			&'static mut Controller,
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
		Self( (values.1.2,values.1.1.leaf_to_root_send(values.1.0)) ,default())
	}

	type ApplySysParam=SystemParamWithQueryT<
		(
			EntityRefExcept<'static,'static,()>,
			&'static mut Controller,
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
		values.1.1.leaf_to_root_receive(values.1.0, self.0);
		// values.1.1.0.leaf_receive(&mut values.1.0.0, values.0.2, self.0);
	}
}

impl<Controller,DynAny:'static> PropagateRootToLeafMut<AttachTo> for PropagateControlRootToLeaf<Controller,DynAny>
where Controller:PropagateControl<DynAny>+Component<Mutability = Mutable>
{
	type BeginSysParam=SystemParamWithQueryT<
		(
			EntityMutExcept<'static,'static,()>,
			&'static mut Controller,
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
		let mut f=values.1.1.root_to_leaf_send(values.1.0);
		move|e| Self((values.1.2,f(e)),default())
	}

	type ProcessSysParam=SystemParamWithQueryT<
		(
			EntityMutExcept<'static,'static,()>,
			&'static mut Controller,
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
		let mut f=values.1.1.root_to_leaf_receive_send(values.1.0,self.0);
		move|e| Self((values.1.2,f(e)),default())
	}
}

pub fn control_process<Controller,DynAny:'static>(mut q:Query<(EntityMutExcept<'static,'static,()>,&'static mut Controller)>)
where Controller:PropagateControl<DynAny>+Component<Mutability = Mutable>
{
	q.par_iter_mut().for_each(|(e,mut c)|{
		c.process(e);
	});
}
#[derive(SystemSet)]
pub struct ControlSystems<DynAny>(PhantomData<DynAny>);

impl_phantom!(ControlSystems<DynAny>);

pub fn plugin<Controller,DynAny:'static+Send+Sync>(app:&mut App)
where Controller:PropagateControl<DynAny>+Component<Mutability = Mutable>
{
	app.add_systems(schedule_sim(), (
		propagate_leaf_to_root_mut::<PropagateControlLeafToRoot<Controller,DynAny>,AttachTo>,
		control_process::<Controller,DynAny>,
		propagate_root_to_leaf_mut::<PropagateControlRootToLeaf<Controller,DynAny>,AttachTo>
	).chain().in_set(ControlSystems::<DynAny>::default()));
}


// #[derive(Component)]
// pub struct DynController<DynAny>(pub Box<dyn PropagateControl<DynAny>+Send+Sync>);

impl<DynAny> PropagateControl<DynAny> for Box<dyn PropagateControl<DynAny>+Send+Sync> {
	fn leaf_to_root_send   (&mut self,entity:EntityMutExcept<()>)->DynAny {
		self.deref_mut().leaf_to_root_send(entity)
	}

	fn leaf_to_root_receive(&self,entity:EntityRefExcept<()>,leaf_data:(Entity,DynAny)) {
		self.deref().leaf_to_root_receive(entity,leaf_data)
	}

	fn process             (&mut self,entity:EntityMutExcept<()>) {
		self.deref_mut().process(entity)
	}

	fn root_to_leaf_send   (&mut self,entity:EntityMutExcept<()>)->Box<dyn FnMut(Entity)->DynAny> {
		self.deref_mut().root_to_leaf_send(entity)
	}

	fn root_to_leaf_receive_send(&mut self,entity:EntityMutExcept<()>,root_data:(Entity,DynAny))->Box<dyn FnMut(Entity)->DynAny> {
		self.deref_mut().root_to_leaf_receive_send(entity,root_data)
	}
}