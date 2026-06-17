
use bevy::{app::{App, Update}, ecs::system::{Query, Res}, math::{Mat3, Quat, Vec3}, reflect::Reflect, time::{Fixed, Time}, transform::components::Transform};
use bevy_ecs_macros::Resource;
use nalgebra::{Const, Matrix, RealField, Storage};
use physics_basic::{rotation::{AngularVel, Rotation}, stats::TimePass};
pub use physics_basic::stats::{Pos,Vel,DirVec};
use simba::scalar::SupersetOf;
use wacky_bag_bevy::stat_component::stat::Stat;
#[derive(Resource,Debug,Clone, Copy,Reflect)]
pub struct WldLengthToScreenLength{
	pub wld_len_2_screen_len:f32,
	pub screen_len_2_wld_len:f32
}

impl Default for WldLengthToScreenLength {
	fn default() -> Self {
		Self { wld_len_2_screen_len: 1.0, screen_len_2_wld_len: 1.0 }
	}
}

impl WldLengthToScreenLength {
	pub fn from_wld_len_2_screen_len(wld_len_2_screen_len:f32)->Self{
		Self { wld_len_2_screen_len, screen_len_2_wld_len: 1.0/wld_len_2_screen_len }
	}
}

pub fn vec_3_num_to_f32<Num:RealField,S:Storage<Num,Const<3>,Const<1>>>(v:
	// &SVector<Num,3>
	&Matrix<Num,Const<3>,Const<1>,S>
)->Vec3{
	Vec3 { x: <Num as SupersetOf<f32>>::to_subset(&v[0]).unwrap(), y: <Num as SupersetOf<f32>>::to_subset(&v[1]).unwrap(), z: <Num as SupersetOf<f32>>::to_subset(&v[2]).unwrap() }
}

pub fn vec_2_num_to_vec_3_f32<Num:RealField,S:Storage<Num,Const<2>,Const<1>>>(v:&Matrix<Num,Const<2>,Const<1>,S>)->Vec3{
	Vec3 { x: <Num as SupersetOf<f32>>::to_subset(&v[0]).unwrap(), y: <Num as SupersetOf<f32>>::to_subset(&v[1]).unwrap(), z: 0.0 }
}

pub fn world_pos_to_transform_2d<Num:RealField+Copy>(mut q:Query<(&Stat<Pos<Num,2>>, &Stat<Vel<Num,2>>, &mut Transform, &Stat<TimePass<Num>>)>,fixed_time:Res<Time<Fixed>>,len_trans:Res<WldLengthToScreenLength>){
	let time_perc=
	fixed_time.overstep_fraction();
	//fixed_time.delta_secs()/fixed_time.timestep().as_secs_f32();
	q.par_iter_mut().for_each(|(q,v,mut trans,dt_per_frame)|{
		let dt=<Num as SupersetOf<f32>>::to_subset(&dt_per_frame.0.0).unwrap()*time_perc;
		let pos_v=vec_2_num_to_vec_3_f32(&q.0.0);
		let vec_v=vec_2_num_to_vec_3_f32(&v.0.0);
		trans.translation=(pos_v+vec_v*dt)*len_trans.wld_len_2_screen_len;
	});
}


pub fn world_pos_to_transform_3d<Num:RealField+Copy>(mut q:Query<(&Stat<Pos<Num,3>>, &Stat<Vel<Num,3>>, &mut Transform, &Stat<TimePass<Num>>)>,fixed_time:Res<Time<Fixed>>,len_trans:Res<WldLengthToScreenLength>){
	let time_perc=
	fixed_time.overstep_fraction();
	//fixed_time.delta_secs()/fixed_time.timestep().as_secs_f32();
	q.par_iter_mut().for_each(|(q,v,mut trans,dt_per_frame)|{
		let dt=<Num as SupersetOf<f32>>::to_subset(&dt_per_frame.0.0).unwrap()*time_perc;
		let pos_v=vec_3_num_to_f32(&q.0.0);
		let vec_v=vec_3_num_to_f32(&v.0.0);
		trans.translation=(pos_v+vec_v*dt)*len_trans.wld_len_2_screen_len;
	});
}

pub fn mat_3_num_to_f32<Num:RealField,S:Storage<Num,Const<3>,Const<3>>>(mat:Matrix<Num,Const<3>,Const<3>,S>)->Mat3{
	Mat3 { 
		x_axis: vec_3_num_to_f32(&mat.column(0)), 
		y_axis: vec_3_num_to_f32(&mat.column(1)), 
		z_axis: vec_3_num_to_f32(&mat.column(2)) 
	}
}

pub fn world_rot_to_transform_3d<Num:RealField+Copy>(mut q:Query<(&Stat<Rotation<Num,3>>, &Stat<AngularVel<Num,3>>, &mut Transform, &Stat<TimePass<Num>>)>, fixed_time:Res<Time<Fixed>>){
	let time_perc=
	fixed_time.overstep_fraction();
	//fixed_time.delta_secs()/fixed_time.timestep().as_secs_f32();
	q.par_iter_mut().for_each(|(r,agv,mut trans,dt_per_frame)|{
		let dt=<Num as SupersetOf<f32>>::to_subset(&dt_per_frame.0.0).unwrap()*time_perc;
		let final_rotation_mat_num=r.0.0*physics_basic::rotation::angular_vel_to_rotation(agv,Num::from_subset(&dt));
		let rot_mat=mat_3_num_to_f32(final_rotation_mat_num);
		trans.rotation=Quat::from_mat3(&rot_mat);
	});
}

pub fn world_rot_to_transform_2d<Num:RealField+Copy>(mut q:Query<(&Stat<Rotation<Num,2>>, &Stat<AngularVel<Num,2>>, &mut Transform, &Stat<TimePass<Num>>)>, fixed_time:Res<Time<Fixed>>){
	let time_perc=
	fixed_time.overstep_fraction();
	//fixed_time.delta_secs()/fixed_time.timestep().as_secs_f32();
	q.par_iter_mut().for_each(|(r,agv,mut trans,dt_per_frame)|{
		let dt=<Num as SupersetOf<f32>>::to_subset(&dt_per_frame.0.0).unwrap()*time_perc;
		let final_rotation_mat_num=r.0.0*physics_basic::rotation::angular_vel_to_rotation(agv,Num::from_subset(&dt));
		// let angle_num=Num::atan2(final_rotation_mat_num[(0,0)], final_rotation_mat_num[(1,0)]);
		// let angle:f32=Num::to_subset(&angle_num).unwrap();
		// trans.rotation=Quat::from_rotation_z(angle);Quat::from
		trans.rotation=Quat::from_mat3(&Mat3 { 
			x_axis: Vec3 { 
				x: Num::to_subset(&final_rotation_mat_num[(0,0)]).unwrap(), 
				y: Num::to_subset(&final_rotation_mat_num[(1,0)]).unwrap(), 
				z: 0.0 }, 
			y_axis: Vec3 { 
				x: Num::to_subset(&final_rotation_mat_num[(0,1)]).unwrap(), 
				y: Num::to_subset(&final_rotation_mat_num[(1,1)]).unwrap(), 
				z: 0.0 }, 
			z_axis: Vec3 { x: 0.0, y: 0.0, z: 1.0 } })
	});
}

pub fn plugin_3d<Num:RealField+Copy>(app:&mut App){
	app.add_systems(Update, (world_pos_to_transform_3d::<Num>,world_rot_to_transform_3d::<Num>));
}
pub fn plugin_2d<Num:RealField+Copy>(app:&mut App){
	app.add_systems(Update, (world_pos_to_transform_2d::<Num>,world_rot_to_transform_2d::<Num>));
}

