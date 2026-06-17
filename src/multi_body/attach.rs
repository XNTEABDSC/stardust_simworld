use bevy::ecs::{entity::Entity, lifecycle::HookContext, relationship::{Relationship, RelationshipTarget}, world::DeferredWorld};
use bevy_ecs_macros::Component;


#[derive(Debug,Component)]
pub struct AttachTo{
	target:Entity,
	// pub(super) level:Option<usize>,
}

pub struct AttachLevel{
	pub level:usize
}

#[derive(Debug,Component)]
pub struct AttachedEntities{
	attached_entities:Vec<Entity>,
}

impl AttachTo {
	pub fn from_entity(entity: Entity)->Self{
		AttachTo{
			target:entity,
			// level:None
		}
	}
	// pub fn get_level(&self)->&Option<usize>{
	// 	&self.level
	// }
}

impl Relationship for AttachTo 
{
	type RelationshipTarget=AttachedEntities;

	fn get(&self) -> Entity {
		self.target
	}

	fn from(entity: Entity) -> Self {
		AttachTo::from_entity(entity)
	}

	fn set_risky(&mut self, entity: Entity) {
		self.target=entity;
	}

	// fn on_insert(
	// 	mut world: DeferredWorld,
	// 	hook_context: HookContext,
	// )
	// {
	// 	Relationship::on_insert(world, hook_context);
	// }
}

impl RelationshipTarget for AttachedEntities {
	const LINKED_SPAWN: bool=true;

	type Relationship=AttachTo;

	type Collection=Vec<Entity>;

	fn collection(&self) -> &Self::Collection {
		&self.attached_entities
	}

	fn collection_mut_risky(&mut self) -> &mut Self::Collection {
		&mut self.attached_entities
	}

	fn from_collection_risky(collection: Self::Collection) -> Self {
		AttachedEntities{attached_entities:collection}
	}
}

