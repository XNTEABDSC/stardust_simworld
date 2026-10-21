use bevy::{app::App, ecs::{entity::Entity, query::QueryItem, schedule::{IntoScheduleConfigs, ScheduleConfigs}, system::{ScheduleSystem, SystemParamItem}}};
use bevy::ecs::component::Component;
use frunk::HList;
use nalgebra::{Const, DefaultAllocator, RealField, allocator::Allocator};
use physics_basic::{rotation::{AngularVel, ConstDimToSoDimT, DimSquare, DimToSoDim, Rotation, RotationMatrix, so_vec_to_mat}, stats::{Pos, Vel}};
use wacky_bag_bevy::{stat_component::{cache_set::{CacheSet, set_cache_set_system_cfg}, stat::Stat}, system::{processing_system::ScheduleConfigsProcessing, propagate_relationship::{PropagateRootToLeafMut, PropagateRootToLeafMutBeginSysParam, PropagateRootToLeafMutProcessSysParam, propagate_root_to_leaf, propagate_root_to_leaf_mut}}, utils::system_param_with_query::{SystemParamWithQuery, SystemParamWithQueryMerge, SystemParamWithQueryT}};

use crate::{multi_body::attach::{AttachTo, AttachToWith}, schedule::schedule_apply_change};

type SPQMerge<A,B>=<A as SystemParamWithQueryMerge<B>>::Merge;

pub type AttachToPos<Num,const DIM:usize>=AttachToWith<Pos<Num,DIM>>;
pub type AttachToRotation<Num,const DIM:usize> = AttachToWith<Rotation<Num,DIM>>;

#[derive(Debug, Clone)]
pub struct PropagatePositional<Num:RealField,const DIM:usize>
where Const<DIM>:DimToSoDim,DefaultAllocator:Allocator<ConstDimToSoDimT<DIM>>
{
	pos:Pos<Num,DIM>,
	rot:Rotation<Num,DIM>,
	rot_mat:RotationMatrix<Num,DIM>,
	vel:Vel<Num,DIM>,
	agv:AngularVel<Num,DIM>,
}

impl<Num: RealField + Copy, const DIM: usize> Copy for PropagatePositional<Num, DIM>
where Const<DIM>:DimToSoDim,DefaultAllocator:Allocator<ConstDimToSoDimT<DIM>,Buffer<Num>:Copy>
{
}

impl<Num:RealField+Copy,const DIM:usize> PropagateRootToLeafMut<AttachTo> for PropagatePositional<Num,DIM>
where Const<DIM>:DimToSoDim+DimSquare,DefaultAllocator:Allocator<ConstDimToSoDimT<DIM>,Buffer<Num> : Send+Sync+Copy>
{
	type BeginSysParam=SystemParamWithQueryT<
		(
			&'static Stat<Pos<Num,DIM>>,
			&'static Stat<Rotation<Num,DIM>>,
			&'static mut CacheSet<Stat<RotationMatrix<Num,DIM>>>,
			&'static Stat<Vel<Num,DIM>>,
			&'static Stat<AngularVel<Num,DIM>>
		),(),
		()
		>
		;

	fn send_begin(
		mut values: 
			QueryItem<
				<SPQMerge<PropagateRootToLeafMutBeginSysParam<AttachTo>, Self::BeginSysParam> as SystemParamWithQuery>::D
			>,
		_others: 
			&SystemParamItem<
				<SPQMerge<PropagateRootToLeafMutBeginSysParam<AttachTo>, Self::BeginSysParam> as SystemParamWithQuery>::P
			>
	)->impl FnMut(Entity)->Self
	{
		
		let s=Self{
			pos:values.1.0.0,
			rot_mat:values.1.2.get_or_else_set(||RotationMatrix::from_so(&values.1.1.0).into()).0,
			rot:values.1.1.0,
			vel:values.1.3.0,
			agv:values.1.4.0,
		};
		return move |_e|{
			s.clone()
		};
	}

	type ProcessSysParam=SystemParamWithQueryT<
		(
			Option<&'static AttachToPos<Num,DIM>>,
			Option<&'static AttachToRotation<Num,DIM>>,
			&'static mut Stat<Pos<Num,DIM>>,
			&'static mut Stat<Rotation<Num,DIM>>,
			&'static mut CacheSet<Stat<RotationMatrix<Num,DIM>>>,
			&'static mut Stat<Vel<Num,DIM>>,
			&'static mut Stat<AngularVel<Num,DIM>>
		),(),
		()
		>
		;

	fn send_process<'w,'s>(
		mut self,
		values: 
			QueryItem<
				<SPQMerge<PropagateRootToLeafMutProcessSysParam<AttachTo>, Self::ProcessSysParam> as SystemParamWithQuery>::D
			>,
		_others: 
			&SystemParamItem<
				<SPQMerge<PropagateRootToLeafMutProcessSysParam<AttachTo>, Self::ProcessSysParam> as SystemParamWithQuery>::P
			>)
	->impl FnMut(Entity)->Self
	{
		let (dp,dr,mut sp,mut sr,mut rm, mut sv,mut sagv) = values.1;
		if let Some(dp) = dp {
			let g_dp=self.rot_mat.0 * dp.0.0;
			self.pos += Pos( g_dp );
			self.vel.0 += so_vec_to_mat(&self.agv.0) * g_dp;
		}
		if let Some(dr) = dr {
			self.rot += dr.0;
		}
		sp.0=self.pos;
		sr.0=self.rot;
		sv.0=self.vel;
		sagv.0=self.agv;
		self.rot_mat = rm.set_and_get(RotationMatrix::from_so(&self.rot).into()).0;
		return move |_e|{
			self.clone()
		};
	}
	
	fn send_end<'w,'s>(
		mut self,
		values: 
			QueryItem<
				<wacky_bag_bevy::utils::system_param_with_query::SystemParamWithQueryMergeT<PropagateRootToLeafMutProcessSysParam<AttachTo>, Self::ProcessSysParam> as SystemParamWithQuery>::D
			>,
		_others: 
			&SystemParamItem<
				<wacky_bag_bevy::utils::system_param_with_query::SystemParamWithQueryMergeT<PropagateRootToLeafMutProcessSysParam<AttachTo>, Self::ProcessSysParam> as SystemParamWithQuery>::P
			>)
	->() {
		let (dp,dr,mut sp,mut sr,mut rm, mut sv,mut sagv) = values.1;
		if let Some(dp) = dp {
			let g_dp=self.rot_mat.0 * dp.0.0;
			self.pos += Pos( g_dp );
			self.vel.0 += so_vec_to_mat(&self.agv.0) * g_dp;
		}
		if let Some(dr) = dr {
			self.rot += dr.0;
		}
		sp.0=self.pos;
		sr.0=self.rot;
		sv.0=self.vel;
		sagv.0=self.agv;
		self.rot_mat = rm.set_and_get(RotationMatrix::from_so(&self.rot).into()).0;
		
	}
}

pub fn propagate_position_rotation_system<Num:RealField+Copy,const DIM:usize>()->ScheduleConfigs<ScheduleSystem>
where Const<DIM>:DimToSoDim+DimSquare,DefaultAllocator:Allocator<ConstDimToSoDimT<DIM>,Buffer<Num> : Send+Sync+Copy>
{
	propagate_root_to_leaf_mut::<PropagatePositional<Num,DIM>,AttachTo>.into_configs()
	.config_processing::<
		HList!(AttachToPos<Num,DIM>,AttachToRotation<Num,DIM>),
		HList!(PropagatePositional<Num,DIM>, Stat<Pos<Num,DIM>>, Stat<Rotation<Num,DIM>>, CacheSet<Stat<RotationMatrix<Num,DIM>>>),
		HList!()
	>()
}

pub fn propagate_position_rotation_plugin<Num:RealField+Copy,const DIM:usize>(app:&mut App)
where Const<DIM>:DimToSoDim+DimSquare,DefaultAllocator:Allocator<ConstDimToSoDimT<DIM>,Buffer<Num> : Send+Sync+Copy>
{
	app.add_systems(schedule_apply_change(), propagate_position_rotation_system::<Num,DIM>());
	app.add_systems(schedule_apply_change(), 
		(
			set_cache_set_system_cfg::<Stat<RotationMatrix<Num,DIM>>,()>(),
		)
	);
}
#[cfg(test)]
mod test{
    use bevy::{ecs::{related, relationship::RelationshipTarget, system::Query, world::World}, tasks::{ComputeTaskPool, TaskPool}};
	use frunk::{Poly, hlist};
	use nalgebra::SVector;
	use wacky_bag_bevy::utils::stat_for_hlist::MapToStat;
	use wacky_bag_hlist::variadics_tuple::ToVariadicsTuple;
	use crate::multi_body::attach::AttachedEntities;

	use super::*;
	const DIM:usize=2;
	type Num=f32;

	pub fn log_pos(q:Query<(Entity,&Stat<Pos<Num,DIM>>)>){
		q.par_iter().for_each(|a|{
			println!("{:?}",a);
		});
	}

	#[test]
	fn test_error(){
		bevy::log::error!("a error");
		bevy::log::debug!("a error");
		bevy::log::info!("a error");
	}
	
	#[test]
	fn test(){
		ComputeTaskPool::get_or_init(TaskPool::default);
		let mut wld=World::new();
		let aid=wld.spawn((
			hlist![
				Pos::<Num,DIM>( SVector::from([1.0,0.0]) ),
				Vel::<Num,DIM>( SVector::from([0.0,1.0]) ),
				Rotation::<Num,DIM>::default(),
				AngularVel::<Num,DIM>::default(),
				RotationMatrix::<Num,DIM>::default(),
			].map(Poly(MapToStat)).to_variadics_tuple(),
			CacheSet::<Stat<RotationMatrix<Num,DIM>>>::default(),

			related!(
				AttachedEntities[
					(
						hlist![
							Pos::<Num,DIM>::default(),
							Vel::<Num,DIM>::default(),
							Rotation::<Num,DIM>::default(),
							AngularVel::<Num,DIM>::default(),
							RotationMatrix::<Num,DIM>::default(),
						].map(Poly(MapToStat)).to_variadics_tuple(),
						CacheSet::<Stat<RotationMatrix<Num,DIM>>>::default(),
						AttachToWith(Pos::<Num,DIM>(SVector::from([-0.5,0.5])))
					),(
						hlist![
							Pos::<Num,DIM>::default(),
							Vel::<Num,DIM>::default(),
							Rotation::<Num,DIM>::default(),
							AngularVel::<Num,DIM>::default(),
							RotationMatrix::<Num,DIM>::default(),
						].map(Poly(MapToStat)).to_variadics_tuple(),
						CacheSet::<Stat<RotationMatrix<Num,DIM>>>::default(),
						AttachToWith(Pos::<Num,DIM>(SVector::from([-0.5,-0.5])))
					)
				]
			)
		));
		println!("{:?}",aid.get::<AttachedEntities>().unwrap().collection());
		
		let log_pos=wld.register_system(log_pos);
		let _=wld.run_system(log_pos);
		
		let propagate=wld.register_system(propagate_root_to_leaf_mut::<PropagatePositional<Num,DIM>,AttachTo>);
		let _=wld.run_system(propagate);

		let _=wld.run_system(log_pos);
	}
}