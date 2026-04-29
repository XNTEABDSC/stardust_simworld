pub mod bundle;

use std::marker::PhantomData;

use bevy::app::{PluginGroup, PluginGroupBuilder};

pub mod apply_velocity;
pub mod apply_rotation;

pub struct TransformPlugins<Num,const DIM:usize>{
	pub p:PhantomData<[Num;DIM]>
}

impl<Num,const DIM:usize> PluginGroup for TransformPlugins<Num,DIM>
where 
	Num:nalgebra::RealField+Copy,
	nalgebra::Const<DIM>: nalgebra::DimMin<nalgebra::Const<DIM>,Output = nalgebra::Const<DIM>>
{
	fn build(self) -> PluginGroupBuilder {
		PluginGroupBuilder::start::<Self>()
			.add(apply_velocity::plugin::<Num,DIM>)
			.add(apply_rotation::plugin::<Num,DIM>)
	}
}