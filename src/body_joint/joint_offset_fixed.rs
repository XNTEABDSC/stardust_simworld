use bevy::ecs::entity::Entity ;
use frunk::HList;
use nalgebra::RealField;
use physics_basic::stats::{Mass, Momentum, Pos};
use wacky_bag_bevy::stat_component::{change::Change, stat::Stat};
use wacky_bag_hlist::h_list_helpers::HToRef;

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
	a.pluck::<&Change<Pos<Num,DIM>>,_>().0.add_change(Pos(-pos_adj_factor*b_mass));
	b.pluck::<&Change<Pos<Num,DIM>>,_>().0.add_change(Pos(pos_adj_factor*a_mass));

	let a_mom=a.pluck::<&Stat<Momentum<Num,DIM>>,_>().0.0;
	let b_mom=b.pluck::<&Stat<Momentum<Num,DIM>>,_>().0.0;
	let sum_mom=a_mom+b_mom;
	let new_v=sum_mom.0/mass_sum;
	let new_a_mom = new_v*a_mass;
	let new_b_mom = new_v*b_mass;

	a.pluck::<&Change<Momentum<Num,DIM>>,_>().0.add_change(Momentum(new_a_mom)-a_mom);
	b.pluck::<&Change<Momentum<Num,DIM>>,_>().0.add_change(Momentum(new_b_mom)-b_mom);
}