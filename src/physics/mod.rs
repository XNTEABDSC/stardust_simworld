pub mod bundle;
pub mod systems;

use std::marker::PhantomData;

use bevy::app::{PluginGroup, PluginGroupBuilder};
use nalgebra::{Const, DefaultAllocator, DimMin, DimName, RealField, allocator::Allocator};
use physics_basic::rotation::{ConstDimToSoDimT, DimSquare, DimSquareSo, DimToSoDimT};


use crate::physics::systems::CalculateSystemsPlugins;

pub mod apply_velocity;
pub mod apply_rotation;
#[derive(Debug, Clone, Copy)]
pub struct Plugins<Num,const DIM:usize>{
	pub p:PhantomData<[Num;DIM]>
}

impl<Num, const DIM: usize> Default for Plugins<Num, DIM> {
    fn default() -> Self {
		Self { p: Default::default() }
	}
}

impl<Num,const DIM:usize> PluginGroup for Plugins<Num,DIM>
where
	Num:RealField+Copy,
	Const<DIM>: DimSquareSo,
	DefaultAllocator: Allocator<ConstDimToSoDimT<DIM>, ConstDimToSoDimT<DIM>,Buffer<Num>:Sync+Send>+Allocator<ConstDimToSoDimT<DIM>,Buffer<Num>:Sync+Send>,
    ConstDimToSoDimT<DIM>:DimSquare,
{
	fn build(self) -> PluginGroupBuilder {
		PluginGroupBuilder::start::<Self>()
			.add(apply_velocity::plugin::<Num,DIM>)
			.add(apply_rotation::plugin::<Num,DIM>)
			.add_group(CalculateSystemsPlugins::<Num,DIM>::default())
	}
}