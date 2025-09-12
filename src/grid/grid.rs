use std::sync::Arc;

use bevy_ecs::{component::Component, entity::Entity, system::Resource};
use derive_more::Deref;
use statistic_physics::num::Num;
use wacky_bag::structures::{grid::Grid2D, just::Just, n_dim_array::NDimArray, n_dim_index::{NDimIndex, NDimIndexer}};




pub type GridResource<const DIM:usize,T>=NDimArray<Arc<NDimIndexer<DIM>>,DIM,T,Vec<T>>;

#[derive(Resource,Deref)]
pub struct GridIndexer<const DIM:usize>(pub Arc<NDimIndexer<DIM>>);

/*

#[derive(Resource)]
pub struct GridResource<const XSIZE:usize,const YSIZE:usize>{
    pub grid_entities:Grid2D<[[Entity;XSIZE];YSIZE]>//Grid2DVec<Entity>
}

impl<const XSIZE:usize,const YSIZE:usize> GridResource<XSIZE,YSIZE> {
    const GRID_SIZE:Num=Num::ONE;

    pub const fn grid_size(&self)->Num{Self::GRID_SIZE}
    pub const fn grid_len(&self)->(usize,usize){(XSIZE,YSIZE)}
    pub const fn grid_entities(&self)->&Grid2D<[[Entity;XSIZE];YSIZE]>{&self.grid_entities}
    pub const fn grid_entities_mut(&mut self)->&mut Grid2D<[[Entity;XSIZE];YSIZE]>{&mut self.grid_entities}
}

 */