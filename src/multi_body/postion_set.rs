use bevy::ecs::{query::ROQueryItem, schedule::{IntoScheduleConfigs, ScheduleConfigs}, system::{ScheduleSystem, System}};
use bevy_ecs_macros::Component;
use frunk::HList;
use nalgebra::RealField;
use physics_basic::{rotation::Rotation, stats::Pos};
use wacky_bag_bevy::{stat_component::{cache_set::CacheSet, determining::Determining, stat::Stat}, system::{processing_system::ScheduleConfigsProcessing, propagate_relationship::{PropagateRootToLeaf, propagate_root_to_leaf}}};

use crate::multi_body::attach::AttachTo;

#[derive(Debug,Component)]
pub struct AttachToPos<Num,const DIM:usize>(pub Pos<Num,DIM>);
#[derive(Debug,Component)]
pub struct AttachToRotation<Num:RealField,const DIM:usize>(pub Rotation<Num,DIM>);

#[derive(Debug, Clone, Copy)]
pub struct PropagatePositionRotation<Num:RealField,const DIM:usize>{
	pos:Pos<Num,DIM>,
	rot:Rotation<Num,DIM>
}

impl<Num:RealField+Copy,const DIM:usize> PropagateRootToLeaf for PropagatePositionRotation<Num,DIM> {

	type DataBegin =(
		&'static Stat<Pos<Num,DIM>>,
		&'static Stat<Rotation<Num,DIM>>
	);

	// type DataBegin =(
	// 	&'static CacheSet<Stat<Pos<Num,DIM>>>,
	// 	&'static CacheSet<Stat<Rotation<Num,DIM>>>
	// );

	type Data =(
		&'static CacheSet<Stat<Pos<Num,DIM>>>,
		&'static CacheSet<Stat<Rotation<Num,DIM>>>,
		Option<&'static AttachToPos<Num,DIM>>,
		Option<&'static AttachToRotation<Num,DIM>>,
	);

	fn from_data<'w,'s>(values:&ROQueryItem<'w,'s,Self::DataBegin>)->Self {
		Self{
			pos:values.0.0,
			rot:values.1.0,
		}
	}

	fn process_data<'w,'s>(&mut self,values:&ROQueryItem<'w,'s,Self::Data>) {
		let (p,r,apm,arm)=values;
		if let Some(ap) = apm {
			self.pos = Pos( self.rot.0 * ap.0.0 );
		}
		if let Some(ar) = arm {
			self.rot = Rotation( ar.0.0 * self.rot.0);
		}
		*p.0.lock().unwrap()
		=Some(Stat(self.pos));
		*r.0.lock().unwrap()
		=Some(Stat(self.rot));
	}
}

pub fn propagate_position_rotation_system<Num:RealField+Copy,const DIM:usize>()->ScheduleConfigs<ScheduleSystem>{
	propagate_root_to_leaf::<PropagatePositionRotation<Num,DIM>,AttachTo>.into_configs()
	.config_processing::<
		HList!(Determining<Pos<Num,DIM>>,Determining<Rotation<Num,DIM>>,AttachToPos<Num,DIM>,AttachToRotation<Num,DIM>),
		HList!(PropagatePositionRotation<Num,DIM>),
		HList!(CacheSet<Stat<Pos<Num,DIM>>>,CacheSet<Stat<Rotation<Num,DIM>>>)
	>()
}