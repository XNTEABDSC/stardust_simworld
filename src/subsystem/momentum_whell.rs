use std::array;

use bevy::ecs::{component::Component, message::MessageReader, system::Query};
use nalgebra::{Const, DefaultAllocator, Matrix, OVector, RealField, allocator::Allocator};
use physics_basic::{rotation::{AngularMomentum, ConstDimToSoDimT, DimToSoDim, RotationMatrix, rotation_transform, rotation_transform_so, so_mat_to_vec, so_vec_to_mat}, stats::TimePass};
use wacky_bag_bevy::stat_component::{change::Change, stat::Stat};


#[derive(Debug,Default,Clone,Component)]
pub struct MomentumWhellDef<Num>{
	rotor_inertia:Num,//only xy
	max_power:Num
}

#[derive(Debug,Default,Clone,Component)]
pub struct MomentumWhellState<Num>{
	agv:Num,
	torque:Num,
	aga:Num,
}

#[derive(Debug,Default,Clone,Component)]
pub struct MomentumWhellControl<Num>{
	pub power:Num
}

pub fn calc_momentum_whell<Num:RealField+Copy>(def:&MomentumWhellDef<Num>,ctrl:&MomentumWhellControl<Num>,state:&mut MomentumWhellState<Num>){
	let mut power=ctrl.power;
	if power<Num::zero(){power=Num::zero();}
	if power>def.max_power {
		power=def.max_power;
	}
	let torque=power/state.agv;
	let aga=torque/def.rotor_inertia;
	state.aga=aga;
	state.torque=torque;
}

// pub struct MomentumWhellControlMsg

pub fn momentum_whell_control_apply<Num:RealField+Copy>(mut q:Query<(&MomentumWhellDef<Num>,&MomentumWhellControl<Num>,&mut MomentumWhellState<Num>)>){
	q.par_iter_mut().for_each(|(d,c,mut s)|{
		calc_momentum_whell(d, c, &mut s);
	});
}

pub fn momentum_whell_active<Num:RealField+Copy,const DIM:usize>(mut q:Query<(
	&mut MomentumWhellState<Num>,
	&Stat<RotationMatrix<Num,DIM>>,
	&Change<AngularMomentum<Num,DIM>>,
	&Stat<TimePass<Num>>
)>)
where Const<DIM>:DimToSoDim,DefaultAllocator:Allocator<ConstDimToSoDimT<DIM>,Buffer<Num> : Send+Sync>
{
	q.par_iter_mut().for_each(|(mut s,rm,cagm,dt)|{
		let dt=dt.0.0;
		let s = &mut *s;
		cagm.add_change(AngularMomentum(
			rotation_transform_so(rm, &OVector::from_fn(|i,_|if i==0{s.torque*dt}else{Num::zero()}))
			// so_mat_to_vec(&rotation_transform(rm, &so_vec_to_mat(&OVector::from_fn(|i,_|if i==0{Num::one()}else{Num::zero()}) )))
		));
		s.agv+=s.aga*dt;
	});
}