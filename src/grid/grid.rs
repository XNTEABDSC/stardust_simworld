use std::{ops::Range, sync::Arc};

use bevy::ecs::resource::Resource;
use derive_more::Deref;
use wacky_bag::structures::n_dim_array::{n_dim_chunk_array::NDimChunkArray, n_dim_indexer::NDimIndexer};





pub type GridResource<const DIM:usize,T>=NDimChunkArray<DIM,T>;

#[derive(Resource,Deref)]
pub struct GridIndexer<const DIM:usize>(pub Arc<NDimIndexer<DIM>>);

#[derive(Resource)]
pub struct GridData<Num,const DIM:usize>{
	pub ranges:[Range<isize>;DIM],
	pub grid_edge_len:Num,
	pub grid_volume:Num
}

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