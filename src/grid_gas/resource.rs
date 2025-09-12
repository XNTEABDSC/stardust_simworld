use bevy_ecs::system::Resource;
use frunk::HList;
use wacky_bag::structures::n_dim_array::NDimArray;

use crate::grid::grid::GridResource;

use statistic_physics::matters::Matters;

pub type GridGasStat<const DIM:usize>=HList!(Matters<DIM>);


#[derive(Resource)]
pub struct GridGasStatResource<const DIM:usize>(pub GridResource<DIM,GridGasStat<DIM>>);

pub type GridGasDelta<const DIM:usize>=HList!(Matters<DIM>);

#[derive(Resource)]
pub struct GridGasDeltaResource<const DIM:usize>(pub GridResource<DIM,GridGasDelta<DIM>>);