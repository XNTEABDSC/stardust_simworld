use bevy::app::App;
use nalgebra::{Const, DefaultAllocator, DimName, RealField, allocator::Allocator};
use physics_basic::rotation::{AllocatorSyncVMSq, ConstDimToSoDimT, DimSquare, DimToSoDim};

pub mod attach;
// pub mod attach_level;
pub mod propagate_change;
pub mod propagate_position;
pub mod attach_root;

pub fn plugin<Num,const DIM:usize>(app:&mut App)
where 
	Num:RealField+Copy,
	Const<DIM>: DimToSoDim + DimName + DimSquare,
	DefaultAllocator: 
		AllocatorSyncVMSq<ConstDimToSoDimT<DIM>,Num>+
        Allocator<ConstDimToSoDimT<DIM>,Buffer<Num> : Send+Sync+Copy>
{
    app.add_plugins((propagate_change::plugin::<Num,DIM>,propagate_position::propagate_position_rotation_plugin::<Num,DIM>));
}