use bevy::ecs::entity::Entity;
use frunk::HList;
use nalgebra::RealField;
use num_traits::Zero;
use physics_basic::{collide::collide_inelastic, rotation::Rotation, stats::{Mass, Momentum, Pos, Vel}};
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
	
	let a_vel_on_dir=cur_pos_dif_norm.dot(&a.pluck::<&Stat<Vel<Num,DIM>>,_>().0.0.0);
	let b_vel_on_dir=cur_pos_dif_norm.dot(&b.pluck::<&Stat<Vel<Num,DIM>>,_>().0.0.0);
	let new_vel=collide_inelastic(a_mass, a_vel_on_dir, b_mass, b_vel_on_dir);

	a.pluck::<&Change<Vel<Num,DIM>>,_>().0.add_change(Vel(cur_pos_dif_norm*(new_vel-a_vel_on_dir)));
	b.pluck::<&Change<Vel<Num,DIM>>,_>().0.add_change(Vel(cur_pos_dif_norm*(new_vel-b_vel_on_dir)));
	
}
pub struct JointOffsetFixed<Num,const DIM:usize>{
	pub entity:Entity,
	pub offset:Pos<Num,DIM>
}
pub type CalcJointOffsetFixedParams<Num,const DIM:usize>=HList!(Stat<Pos<Num,DIM>>,Stat<Momentum<Num,DIM>>,Stat<Mass<Num>>,Change<Pos<Num,DIM>>,Change<Momentum<Num,DIM>>);
pub fn calc_joint_offset_fixed<Num:RealField+Copy,const DIM:usize>(
	offset:Pos<Num,DIM>,
	a:HToRef<CalcJointOffsetFixedParams<Num,DIM>>,
	b:HToRef<CalcJointOffsetFixedParams<Num,DIM>>,
){
	let (a_mass,b_mass)={
		let (a_mass,b_mass)=(a.pluck::<&Stat<Mass<Num>>,_>().0.0.0,b.pluck::<&Stat<Mass<Num>>,_>().0.0.0);
		if a_mass.is_zero()&&b_mass.is_zero() {
			(Num::one(),Num::one())
		}else {
			(a_mass,b_mass)
		}
	};
	let mass_sum=a_mass+b_mass;
	let cur_pos_dif=b.pluck::<&Stat<Pos<Num,DIM>>,_>().0.0-a.pluck::<&Stat<Pos<Num,DIM>>,_>().0.0;
	let cur_2_ofs=offset-cur_pos_dif;
	let pos_adj_factor=cur_2_ofs.0/(mass_sum);
	a.pluck::<&Change<Pos<Num,DIM>>,_>().0.add_change(Pos(-pos_adj_factor*a_mass));
	b.pluck::<&Change<Pos<Num,DIM>>,_>().0.add_change(Pos(pos_adj_factor*b_mass));

	let a_mom=a.pluck::<&Stat<Momentum<Num,DIM>>,_>().0.0;
	let b_mom=b.pluck::<&Stat<Momentum<Num,DIM>>,_>().0.0;
	let sum_mom=a_mom+b_mom;
	let new_v=sum_mom.0/mass_sum;
	let new_a_mom = new_v*a_mass;
	let new_b_mom = new_v*b_mass;

	a.pluck::<&Change<Momentum<Num,DIM>>,_>().0.add_change(Momentum(new_a_mom)-a_mom);
	b.pluck::<&Change<Momentum<Num,DIM>>,_>().0.add_change(Momentum(new_b_mom)-b_mom);
}

pub struct JointRotationFixed<Num:RealField,const DIM:usize>{
	pub entity:Entity,
	pub offset:Rotation<Num,DIM>
}

pub fn calc_joint_rotation_fixed<Num:RealField,const DIM:usize>(){

}

// pub struct JointDistanceSoft<Num>{
// 	pub entity:Entity,
// 	pub dist:Num,
// 	pub k:Num
// }

