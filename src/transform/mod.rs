use std::marker::PhantomData;

use bevy::app::{PluginGroup, PluginGroupBuilder};
use nalgebra::RealField;
use wacky_bag_bevy::utils::plugin_add_systems::plugin_insert_resource;

use crate::transform::tramsform::WldLengthToScreenLength;


pub mod tramsform;

pub struct TransformPlugins<Num,const DIM:usize>{
	pub wld_length_to_screen_length:WldLengthToScreenLength,
	pub _p:PhantomData<[Num;DIM]>
}

impl<Num> PluginGroup for TransformPlugins<Num,2>
	where Num:RealField+Copy
{
	fn build(self) -> PluginGroupBuilder {
		PluginGroupBuilder::start::<Self>()
			.add(plugin_insert_resource(self.wld_length_to_screen_length))
			.add(tramsform::plugin_2d::<Num>)
	}
}

impl<Num> PluginGroup for TransformPlugins<Num,3>
	where Num:RealField+Copy
{
	fn build(self) -> PluginGroupBuilder {
		PluginGroupBuilder::start::<Self>()
			.add(plugin_insert_resource(self.wld_length_to_screen_length))
			.add(tramsform::plugin_3d::<Num>)
	}
}
