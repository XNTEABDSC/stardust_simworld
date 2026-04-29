use nalgebra::{Matrix, RealField, SMatrix};
use physics_basic::rotation::Rotation;
use wacky_bag_bevy::stat_component::change_generic::AlgebraicSystem;



pub struct RotationAlgebra;

impl<Num,const DIM:usize> AlgebraicSystem<Rotation<Num,DIM>> for RotationAlgebra 
	where Num:RealField+Copy,
		
{
	fn unit()->Rotation<Num,DIM> {
		Rotation(SMatrix::<Num,DIM,DIM>::identity())
	}

	fn apply_assign(a:&mut Rotation<Num,DIM>,b:Rotation<Num,DIM>) {
		a.0*=b.0;
	}

	fn apply(a:T,b:T)->T {
		todo!()
	}

	fn inverse(a:T)->T {
		todo!()
	}
}