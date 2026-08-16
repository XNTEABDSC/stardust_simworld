use bevy::{ecs::entity::Entity, log::error} ;
use frunk::HList;
use nalgebra::{Const, DefaultAllocator, DimMin, DimName, RealField, allocator::Allocator};
use physics_basic::rotation::{AngularInertia, AngularVel, ConstDimToSoDimT, DimSquare, DimToSoDim, Rotation, so_mat_to_vec, so_vec_to_mat};
use wacky_bag_bevy::stat_component::{change::Change, stat::Stat};
use wacky_bag_hlist::h_list_helpers::HToRef;

pub struct JointRotationFixed<Num:RealField,const DIM:usize,Data>
where Const<DIM>:DimToSoDim,DefaultAllocator:Allocator<ConstDimToSoDimT<DIM>>
{
	pub a:Data,
	pub b:Data,
	pub rotation_offset:Rotation<Num,DIM>
}

// impl<Num:RealField,const DIM:usize,Data> JointRotationFixed<Num,DIM,Data>{
// 	pub fn map<F,Data2>(self,mut f:&mut F)->JointRotationFixed<Num,DIM,Data2>
// 	where F:FnMut(Data)->Data2
// 	{
// 		Self { a: f(self.a), b: f(self.b), rotation_offset: () }
// 	}
// }

pub type CalcJointRotationFixedParams<Num,const DIM:usize> = HList!(
	Stat<AngularInertia<Num,DIM>>, Stat<Rotation<Num,DIM>>, Stat<AngularVel<Num,DIM>>,
	Change<Rotation<Num,DIM>>, Change<AngularVel<Num,DIM>>
);
pub fn calc_joint_rotation_fixed<Num:RealField+Copy,const DIM:usize>(
	JointRotationFixed{a,b,rotation_offset}:
	JointRotationFixed<Num,DIM,HToRef<CalcJointRotationFixedParams<Num,DIM>>>
	// rotation_offset:Rotation<Num,DIM>,
	// a:HToRef<CalcJointRotationFixedParams<Num,DIM>>,
	// b:HToRef<CalcJointRotationFixedParams<Num,DIM>>
)
where
	Num:RealField+Copy,
	Const<DIM>: DimToSoDim + DimName + DimMin<Const<DIM>, Output = Const<DIM>>,
	DefaultAllocator: Allocator<ConstDimToSoDimT<DIM>, ConstDimToSoDimT<DIM>,Buffer<Num>:Sync+Send>+Allocator<ConstDimToSoDimT<DIM>,Buffer<Num>:Sync+Send>,
    ConstDimToSoDimT<DIM>:DimSquare
{
	let (a_agi,b_agi)=
	(&a.pluck::<&Stat<AngularInertia<Num,DIM>>,_>().0.0.0,&b.pluck::<&Stat<AngularInertia<Num,DIM>>,_>().0.0.0);
	
	// let (a_mass,b_mass)={
	// 	let (a_mass,b_mass)=(a.pluck::<&Stat<AngularInertia<Num,DIM>>,_>().0.0.0,b.pluck::<&Stat<AngularInertia<Num,DIM>>,_>().0.0.0);
	// 	if !a_mass.is_zero()||!b_mass.is_zero() {
	// 		(a_mass,b_mass)
	// 	}else {
	// 		(
	// 			nalgebra::OMatrix::<Num, DimNameToSoDimNameType<DIM>, DimNameToSoDimNameType<DIM>>::identity(),
	// 			nalgebra::OMatrix::<Num, DimNameToSoDimNameType<DIM>, DimNameToSoDimNameType<DIM>>::identity(),
	// 		)
	// 		// todo!()
	// 	}
	// };
	let mut inv_sum_agi=a_agi+b_agi;
	if !inv_sum_agi.try_inverse_mut(){
		error!("calc_joint_rotation_fixed solving sum_agi {:?} that cant be inversed",inv_sum_agi);
		return;
	}

	let cur_rot_ofs=&b.pluck::<&Stat<Rotation<Num,DIM>>,_>().0.0.0-&a.pluck::<&Stat<Rotation<Num,DIM>>,_>().0.0.0;
	let cur_2_ofs=rotation_offset.0-cur_rot_ofs;
	let rot_adj_factor=&inv_sum_agi*&cur_2_ofs;

	// a_agi * a_v_c + b_agi * b_v_c ==0
	// a_v_c + b_v_c = cur_2_ofs
	// a_v_c = -cur_2_ofs / (a_agi+b_agi) * b_agi 
	// b_v_c =  cur_2_ofs / (a_agi+b_agi) * a_agi 
	// let rot_adj_factor=cur_2_ofs.0/sum_agi;

	a.pluck::<&Change<Rotation<Num,DIM>>,_>().0.add_change(Rotation(-b_agi*&rot_adj_factor ));
	b.pluck::<&Change<Rotation<Num,DIM>>,_>().0.add_change(Rotation(a_agi*rot_adj_factor ));

	
	let cur_agv_ofs=&b.pluck::<&Stat<AngularVel<Num,DIM>>,_>().0.0.0-&a.pluck::<&Stat<AngularVel<Num,DIM>>,_>().0.0.0;
	let cur_agv_ofs_so=cur_agv_ofs;
	// let agv_adj_factor=inv_sum_agi*so_mat_to_vec(&-cur_agv_ofs.0);

	a.pluck::<&Change<AngularVel<Num,DIM>>,_>().0
	.add_change(AngularVel(&inv_sum_agi*b_agi*&cur_agv_ofs_so  ));
	b.pluck::<&Change<AngularVel<Num,DIM>>,_>().0
	.add_change(AngularVel(inv_sum_agi*a_agi*-cur_agv_ofs_so  ));


}

