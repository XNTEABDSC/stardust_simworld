use bevy::{app::App, ecs::{entity::{Entity, EntityHashSet}, lifecycle::{Insert, Remove}, observer::On, query::{Changed, Without}, system::{Commands, ParamSet, Query, ResMut}}, log::warn, reflect::Reflect};
use bevy_ecs_macros::{Component, Resource};
use wacky_bag::utils::default_of::default;

use crate::multi_body::attach::{AttachTo, AttachedEntities};

#[derive(Resource)]
pub struct AttachLevels{
	pub entities:Vec<EntityHashSet>,
}
#[derive(Debug,Component,Reflect)]
pub struct AttachLevel(pub usize);

pub fn on_attach_inserted(
	event:On<Insert, AttachTo>,
	mut commands:Commands,
	mut ps:ParamSet<
		(Query<&AttachTo>,
		Query<&AttachLevel>)
	>,
	mut attach_levels:ResMut<AttachLevels>)
{
	let Some(target)=ps.p0().related::<AttachTo>(event.entity) else {
		warn!("On<Insert, AttachTo> event.entity dont have AttachTo");
		return;
	};
	
	let binding = ps.p1();

	let level=if let Ok(AttachLevel (prev_level))=binding.get(target){
		*prev_level+1
	}else {
		1
	};
	attach_levels.entities.resize_with(level+1, default);
	attach_levels.entities[level].insert(event.entity);
	commands.entity(event.entity).entry::<AttachLevel>().and_modify(move|mut v|v.0=level).or_insert(AttachLevel(level));
}

pub fn on_attach_removed(
	event:On<Remove, AttachTo>,
	mut commands:Commands,
	mut ps:ParamSet<
		(
		Query<&AttachLevel>,
		// Query<&AttachLevel>
		)
	>,
	mut attach_levels:ResMut<AttachLevels>
){
	let binding = ps.p0();
	let Ok(target)=binding.get(event.entity) else {
		warn!("On<Insert, AttachTo> event.entity dont have AttachTo");
		return;
	};
	let level=target.0;
	
	// let binding = ps.p1();

	// let level=if let Ok(AttachLevel (prev_level))=binding.get(target){
	// 	*prev_level+1
	// }else {
	// 	1
	// };
	// attach_levels.entities.resize_with(level+1, default);
	attach_levels.entities[level].remove(&event.entity);
	commands.entity(event.entity).remove::<AttachLevel>();
	
}

pub fn plugin_observer(app:&mut App){
	app.add_observer(on_attach_inserted);
}

pub fn update_attach_levels(
	query_root:Query<(&mut AttachedEntities,), (Without<AttachTo>,Changed<AttachedEntities>)>,
	query_node:Query<(&AttachTo,&mut AttachedEntities,), >,
	query_leaf:Query<(&mut AttachTo,),(Without<AttachedEntities>,)>,
	attach_levels:ResMut<AttachLevels>
)
{
	
}