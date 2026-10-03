use std::{any::Any, fmt::Debug};

use bevy::ecs::{bundle::Bundle, component::{Component, Mutable}, entity::Entity, system::Query, world::{EntityMutExcept, EntityRef}};
use wacky_bag_bevy::message::message_on_entity::MessagesOnEntity;

use crate::subsystem::DynAny;

// pub trait ControllerMut{
// 	fn process_mut(&mut self, entity:EntityMutExcept<()>, data:DynAny);
// }

// pub trait MessageController<DynAny> {
// 	fn receive_msg(&self, entity:EntityRef, data:DynAny);
// }

// pub fn process_message_control<MsgCtrl:MessageController<DynAny>+Component,DynAny:Send+Sync+'static>(mut q:Query<(&MsgCtrl,&MessagesOnEntity<DynAny>,EntityRef)>){
// 	q.par_iter_mut().for_each(|(c,m,er)|{
// 		// m.par_for_each(||);
// 	});
// }


pub trait MessageController<C,DynAny:Send+Sync+'static>
where (C, MessagesOnEntity<DynAny>): Bundle
// where Self:Component+Sized+'static
{
	fn receive_msg(&mut self, entity:&mut EntityMutExcept<(C,MessagesOnEntity<DynAny>)>, data:DynAny);
	
}

// impl<DynAny> MessageController<DynAny> for Box<dy>  {
	
// }

pub fn process_message_control<MsgCtrl,DynAny>(mut q:Query<(&mut MsgCtrl,&mut MessagesOnEntity<DynAny>,EntityMutExcept<(MsgCtrl,MessagesOnEntity<DynAny>)>)>)
where MsgCtrl:MessageController<MsgCtrl,DynAny>+Component<Mutability = Mutable>,
	DynAny:Send+Sync+'static
{
	q.par_iter_mut().for_each(|(mut c,mut ms,mut em)|{
		ms.drain().for_each(move |m|{
			c.receive_msg(&mut em, m);
		});
		// c.receive_msg(entity, data);
		// m.par_for_each(||);
	});
}

#[cfg(test)]
mod test{
	use bevy::{prelude::*, tasks::{ComputeTaskPool, TaskPool}};
	// use wacky_bag_bevy::{component::Comp, stat_component::stat::Stat};
	use super::*;

	use wacky_bag_bevy::{message::{message_on_entity::{MessageOnEntity, MessagesOnEntity, MessagesOnEntityRes, MessagesOnEntityWriter}, owned_message::{ParOwnedMessageWriter, apply_owned_message_parallel}}, utils::query_debug::query_debug};
	#[derive(Debug,Default,Component)]
	struct Strings{
		pub strs:Vec<String>
	}
	
	#[derive(Component)]
	struct DynCtrl(pub Box<dyn MessageController<DynCtrl,DynAny>+Send+Sync+'static>);

	impl MessageController<DynCtrl,DynAny> for DynCtrl {
		fn receive_msg(&mut self, entity:&mut EntityMutExcept<(DynCtrl,MessagesOnEntity<DynAny>)>, data:DynAny) {
			self.0.receive_msg(entity, data);
		}
	}
	#[derive(Debug,Default,Clone, Copy)]
	struct CollectStrs;

	impl<C> MessageController<C,DynAny> for CollectStrs
	where (C, MessagesOnEntity<DynAny>): Bundle
	{
		fn receive_msg(&mut self, entity:&mut EntityMutExcept<(C, MessagesOnEntity<DynAny>)>, data:DynAny) {
			if let Ok(data)=data.downcast::<String>(){
				entity.get_mut::<Strings>().unwrap().strs.push(*data);
			}
		}
	}

	// struct MessageRecvLog;
	#[derive(Debug,Component)]
	struct GenStringAndSend{
		recv_entity:Entity,
		count:i32
	}

	fn gen_string_and_send(mut q:Query<(&Name,&mut GenStringAndSend,)>,mw:MessagesOnEntityWriter<DynAny>){
		q.par_iter_mut().for_each(|(n,mut g)|{
			let str=format!("{} send {}",n,g.count);
			g.count+=1;
			mw.write(MessageOnEntity { entity: g.recv_entity, msg: Box::new(str) });
		});
	}

	// fn log_all_strings(q:Query<(Entity,&Name,&Strings)>){
		
	// }

	#[test]
	fn test(){
		ComputeTaskPool::get_or_init(TaskPool::default);
		let mut world = World::default();


		world.init_resource::<MessagesOnEntityRes<DynAny>>();

		let recv_id=world.spawn((
			Name::new("recv 1"),
			Strings::default(),
			DynCtrl(Box::new(CollectStrs)),
			MessagesOnEntity::<DynAny>::default()
		)).id();

		let mut schedule=Schedule::default();
		schedule.add_systems((gen_string_and_send,apply_owned_message_parallel::<MessageOnEntity<DynAny>> ,process_message_control::<DynCtrl,DynAny>).chain());

		let mut dbg_schedule = Schedule::default();
		dbg_schedule.add_systems(query_debug::<(&Name,&Strings),()>);
		
		world.resource_scope::<MessagesOnEntityRes<DynAny>,_>(|_w,ms|{
			ms.write(MessageOnEntity { entity: recv_id, msg: Box::new(String::from("wld write 1")) });
			ms.write(MessageOnEntity { entity: recv_id, msg: Box::new(String::from("wld write 2")) });
		});

		let sender=world.spawn((
			Name::new("send 1"),
			GenStringAndSend{
				recv_entity:recv_id,
				count:0
			}
		)).id();

		dbg_schedule.run(&mut world);
		println!();

		schedule.run(&mut world);
		println!("run 1");

		dbg_schedule.run(&mut world);
		println!();
		
		schedule.run(&mut world);
		println!("run 2");

		dbg_schedule.run(&mut world);
		println!();
		
		schedule.run(&mut world);
		println!("run 3");

		dbg_schedule.run(&mut world);
		println!();

	}
}