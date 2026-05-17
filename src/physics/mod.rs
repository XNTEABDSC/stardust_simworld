pub mod bundle;
pub mod systems;

use std::marker::PhantomData;

use bevy::app::{PluginGroup, PluginGroupBuilder};
use nalgebra::{Const, DefaultAllocator, DimMin, DimName, RealField, allocator::Allocator};
use physics_basic::rotation::{DimNameToSoDimName, DimNameToSoDimNameType};

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
	Const<DIM>: DimNameToSoDimName + DimName + DimMin<Const<DIM>, Output = Const<DIM>>,
	DefaultAllocator: Allocator<DimNameToSoDimNameType<DIM>, DimNameToSoDimNameType<DIM>,Buffer<Num>:Sync+Send>+Allocator<DimNameToSoDimNameType<DIM>,Buffer<Num>:Sync+Send>,
    DimNameToSoDimNameType<DIM>:
        DimMin<DimNameToSoDimNameType<DIM>, Output = DimNameToSoDimNameType<DIM>>,
{
	fn build(self) -> PluginGroupBuilder {
		PluginGroupBuilder::start::<Self>()
			.add(apply_velocity::plugin::<Num,DIM>)
			.add(apply_rotation::plugin::<Num,DIM>)
			.add_group(CalculateSystemsPlugins::<Num,DIM>::default())
	}
}