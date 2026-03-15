use bevy::{app::{App, FixedPreUpdate}, ecs::{schedule::IntoScheduleConfigs, system::{Query, Res}}, utils::default};
use bevy_ecs_macros::Component;
use physics_basic::num::Num;
use wacky_bag::structures::n_dim_array::n_dim_index::NDimIndex;
use wacky_bag_bevy::{physics::position::Position, stat_component::stat::Stat, system::processing_system::ProcessingSystemSet};

use crate::{grid::grid::GridData, schedule::schedule_pre_sim};

#[derive(Component)]
pub struct AtGridCell<const DIM:usize>(pub NDimIndex<DIM>);

pub fn entity_at_grid<const DIM:usize>(mut q:Query<(&Stat<Position<DIM>>,&mut AtGridCell<DIM>)>,grid_data:Res<GridData<DIM>>) {
	let grid_edge_len=grid_data.grid_edge_len;
	q.par_iter_mut().for_each(|(p,mut at)|{
		let a:[Num;DIM]=p.0.0.into();
		at.0=a.map(|i|(i/grid_edge_len).floor().to_num())
	});
}

pub fn entity_at_grid_plugin<const DIM:usize>(app:&mut App){
	app.add_systems(schedule_pre_sim(), 
		entity_at_grid::<DIM>.into_configs()
		.after(ProcessingSystemSet::<Stat<Position<DIM>>>::default())
		.before(ProcessingSystemSet::<AtGridCell<DIM>>::default())
	);
}