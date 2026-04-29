use bevy::{app::App, ecs::system::Query};
use nalgebra::{Const, DimMin, RealField};
use physics_basic::{rotation::{AngularVel, Rotation, RotationDelta, angular_vel_to_rotation}, stats::TimePass};
use wacky_bag_bevy::stat_component::{change::Change, change_generic::{AlgebraicSystemMul, ChangeGeneric}, determining_apply_changes::determining_apply_changes_2, stat::Stat};

use crate::schedule::{schedule_apply_change, schedule_sim};





// pub fn apply_angular_velocity<Num:RealField+Copy,const DIM:usize>(mut q:Query<(
// 	&ChangeGeneric<Rotation<Num,DIM>,AlgebraicSystemMul>,
// 	&Stat<AngularVel<Num,DIM>>,&Stat<TimePass<Num>>)>
// )
// 	where Const<DIM>: DimMin<Const<DIM>,Output = Const<DIM>>
// {
// 	q.par_iter().for_each(|(r,agv,t)|{
// 		let to_mul=angular_vel_to_rotation(agv,t.0.0);
// 		r.add_change(to_mul);
// 	});
// }

pub fn apply_angular_velocity<Num:RealField+Copy,const DIM:usize>(q:Query<
	(&Change<RotationDelta<Num,DIM>>,&Stat<AngularVel<Num,DIM>>,&Stat<TimePass<Num>>)
	>
)
where Const<DIM>: DimMin<Const<DIM>,Output = Const<DIM>>
{
	q.par_iter().for_each(|(r,agv,t)|{
		let to_mul=angular_vel_to_rotation(agv,t.0.0);
		r.add_change(RotationDelta(to_mul));
	});
}

pub fn plugin<Num:RealField+Copy,const DIM:usize>(app:&mut App)
	where Const<DIM>: DimMin<Const<DIM>,Output = Const<DIM>>
{
	app.add_systems(schedule_sim(), apply_angular_velocity::<Num,DIM>);
	// app.add_systems(schedule_apply_change(), determining_apply_changes_2::<
	// 	Rotation<Num,DIM>,
	// 	RotationDelta<Num,DIM>
	// 	>);
}