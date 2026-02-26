use std::{ops::Add, str};

use bevy::ecs::resource::Resource;
use frunk::{Func, HList, Poly, hlist::{self, HMappable, HZippable},};
use statistic_physics::matters::{MattersBasic, MattersFull};
use wacky_bag_bevy::{stat_component::{change::Change, stat::Stat}, utils::stat_for_hlist::{MapToChange, MapToStat}};


use crate::grid::grid::GridResource;

// use statistic_physics::matters::Matters;

// pub type GridGasStat<const DIM:usize>=HList!(Matters<DIM>);


#[derive(Resource)]
pub struct GridGasResource<const DIM:usize>(pub GridResource<DIM,
	
	<
		<MattersFull::<DIM> as HMappable<Poly<MapToStat>>>::Output
		as Add<
		<MattersBasic::<DIM> as HMappable<Poly<MapToChange>>>::Output
	>
	>::Output
>);
