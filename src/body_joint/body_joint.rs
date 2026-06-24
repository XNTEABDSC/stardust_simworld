use bevy::ecs::entity::Entity;
use frunk::HList;
use nalgebra::RealField;
use num_traits::Zero;
use physics_basic::stats::{Mass, Momentum, Pos, Vel};
use wacky_bag_bevy::stat_component::{change::Change, stat::Stat};
use wacky_bag_hlist::h_list_helpers::HToRef;



pub struct JointDistanceFixed<Num>{
	pub entity:Entity,
	pub dist:Num
}
pub type CalcJointDistanceFixedParams<Num,const DIM:usize>=HList!(Stat<Pos<Num,DIM>>,Stat<Vel<Num,DIM>>,Stat<Mass<Num>>,Change<Pos<Num,DIM>>,Change<Vel<Num,DIM>>);
pub fn calc_joint_distance_fixed<Num,const DIM:usize>(
	dist:Num,
	a:HToRef<CalcJointDistanceFixedParams<Num,DIM>>,
	b:HToRef<CalcJointDistanceFixedParams<Num,DIM>>,
) 
where Num:RealField+Copy
{
	let cur_pos_dif=b.pluck::<&Stat<Pos<Num,DIM>>,_>().0.0-a.pluck::<&Stat<Pos<Num,DIM>>,_>().0.0;
	if cur_pos_dif.is_zero() {
		return;
	}
	let cur_pos_dif_norm=cur_pos_dif.0.normalize();
	let (a_mass,b_mass)={
		let (a_mass,b_mass)=(a.pluck::<&Stat<Mass<Num>>,_>().0.0.0,b.pluck::<&Stat<Mass<Num>>,_>().0.0.0);
		if a_mass.is_zero()&&b_mass.is_zero() {
			(Num::one(),Num::one())
		}else {
			(a_mass,b_mass)
		}
	};
	
	let mass_sum=a_mass+b_mass;
	let cur_pos_dif_dist=cur_pos_dif.0.magnitude();//len
	let cur_pos_dist_adj=dist-cur_pos_dif_dist;
	let pos_adj_factor=cur_pos_dif_norm*(cur_pos_dist_adj/mass_sum);
	a.pluck::<&Change<Pos<Num,DIM>>,_>().0.add_change(Pos(-pos_adj_factor*a_mass));
	b.pluck::<&Change<Pos<Num,DIM>>,_>().0.add_change(Pos(pos_adj_factor*b_mass));

	let cur_vel_dif=b.pluck::<&Stat<Vel<Num,DIM>>,_>().0.0-a.pluck::<&Stat<Vel<Num,DIM>>,_>().0.0;
	let cur_vel_dif_on_dir=cur_vel_dif.0.dot(&cur_pos_dif_norm);


}

pub struct JointDistanceSoft<Num>{
	pub entity:Entity,
	pub dist:Num,
	pub k:Num
}

