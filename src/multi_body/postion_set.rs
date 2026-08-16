use bevy::{app::App, ecs::{query::ROQueryItem, schedule::{IntoScheduleConfigs, ScheduleConfigs}, system::{Query, ScheduleSystem, System}}};
use bevy_ecs_macros::Component;
use frunk::HList;
use nalgebra::{Const, DefaultAllocator, RealField, allocator::Allocator};
use physics_basic::{rotation::{ConstDimToSoDimT, DimToSoDim, Rotation, RotationMatrix}, stats::Pos};
use wacky_bag_bevy::{stat_component::{cache_set::{CacheSet, set_cache_set_system, set_cache_set_system_cfg}, determining::Determining, stat::Stat}, system::{processing_system::{ProcessingSystemSet, ScheduleConfigsProcessing}, propagate_relationship::{PropagateRootToLeaf, PropagateRootToLeafMut, propagate_root_to_leaf}}};

use crate::{multi_body::attach::AttachTo, schedule::schedule_apply_change};

#[derive(Debug,Component)]
pub struct AttachToPos<Num,const DIM:usize>(pub Pos<Num,DIM>);
#[derive(Debug,Component)]
pub struct AttachToRotation<Num:RealField,const DIM:usize>(pub Rotation<Num,DIM>)
where Const<DIM>:DimToSoDim,DefaultAllocator:Allocator<ConstDimToSoDimT<DIM>>;

#[derive(Debug, Clone)]
pub struct PropagatePositionRotation<Num:RealField,const DIM:usize>
where Const<DIM>:DimToSoDim,DefaultAllocator:Allocator<ConstDimToSoDimT<DIM>>
{
	pos:Pos<Num,DIM>,
	rot:Rotation<Num,DIM>,
	rot_mat:RotationMatrix<Num,DIM>
}

impl<Num: RealField + Copy, const DIM: usize> Copy for PropagatePositionRotation<Num, DIM>
where Const<DIM>:DimToSoDim,DefaultAllocator:Allocator<ConstDimToSoDimT<DIM>,Buffer<Num>:Copy>
{
}
 
impl<Num:RealField+Copy,const DIM:usize> PropagateRootToLeafMut for PropagatePositionRotation<Num,DIM>
where Const<DIM>:DimToSoDim,DefaultAllocator:Allocator<ConstDimToSoDimT<DIM>,Buffer<Num> : Send+Sync+Copy>
{

	type DataBegin =(
		&'static Stat<Pos<Num,DIM>>,
		&'static Stat<Rotation<Num,DIM>>,
		&'static mut Stat<RotationMatrix<Num,DIM>>
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
			rot_mat:values.2.0
		}
	}

	fn process_data<'w,'s>(&mut self,values:&ROQueryItem<'w,'s,Self::Data>) {
		let (p,r,apm,arm)=values;
		if let Some(ap) = apm {
			self.pos = Pos( self.rot_mat.0 * ap.0.0 );
		}
		if let Some(ar) = arm {
			self.rot = Rotation( ar.0.0 + self.rot.0);
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

pub fn propagate_position_rotation_plugin<Num:RealField+Copy,const DIM:usize>(app:&mut App) {
	app.add_systems(schedule_apply_change(), propagate_position_rotation_system::<Num,DIM>());
	app.add_systems(schedule_apply_change(), 
		(
			set_cache_set_system_cfg::<Stat<Pos<Num,DIM>>,()>(),
			set_cache_set_system_cfg::<Stat<Rotation<Num,DIM>>,()>()
		)
	);
}