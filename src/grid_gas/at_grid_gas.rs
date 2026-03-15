use std::array;

use bevy::{app::{App, FixedPostUpdate, FixedPreUpdate}, ecs::{entity::Entity, schedule::IntoScheduleConfigs, system::{ParallelCommands, Query, Res}}, log::warn};
use bevy_ecs_macros::Component;
use frunk::{HList, HNil, Poly};
use statistic_physics::matters::MattersBasic;
use wacky_bag::{structures::n_dim_array::t_n_dim_array::TNDimArray, utils::h_list_helpers::{HMapP, HToMut, HToRef}};
use wacky_bag_bevy::{system::processing_system::ScheduleConfigsProcessing, utils::stat_for_hlist::{HChangeApplyChange, HChangeGetAndReset, MapTakeStatChange, MapToChange}};

use crate::{grid::at_grid::AtGridCell, grid_gas::resource::{GridGasDatas, GridGasResource}, schedule::{schedule_apply_change, schedule_pre_sim}};

/// we simply copy [GridGasDatas] from grid to entity when [FixedPreUpdate], and put changes back when [FixedPostUpdate]
#[derive(Component)]
pub struct AtGridCellGas<const DIM:usize>(pub GridGasDatas<DIM>);

pub fn set_at_grid_gas<const DIM:usize>(mut q:Query<(Entity,&AtGridCell<DIM>,Option<&mut AtGridCellGas<DIM>>)>,res:Res<GridGasResource<DIM>>, p_cmd:ParallelCommands ){
	q.par_iter_mut().for_each(|(e,pos,c_may)|{
		let new_c_may=res.0.get(&pos.0);
		match c_may {
			Some(mut c) => match new_c_may {
				Some(nc) => {c.0=nc.to_ref().map(Poly(MapTakeStatChange));},
				None => {p_cmd.command_scope(|mut c|{c.entity(e).remove::<AtGridCellGas<DIM>>();});},
			},
			None => match new_c_may {
				Some(nc) => {p_cmd.command_scope(|mut c|{c.entity(e).insert(AtGridCellGas(nc.to_ref().map(Poly(MapTakeStatChange))));});},
				None => {},
			},
		};
	});
}

pub fn apply_at_grid_gas_change<const DIM: usize>(mut q:Query<(Entity,&AtGridCell<DIM>,&mut AtGridCellGas<DIM>)>,res:Res<GridGasResource<DIM>>)
{
	q.par_iter_mut().for_each(|(e,pos,mut at_cell)|{
		let grid_c_may=res.0.get(&pos.0);
		match grid_c_may {
			Some(grid_cell) => {
				let grid_cell_changes: HToRef<HMapP<MattersBasic<DIM>,MapToChange>> =grid_cell.to_ref().sculpt().0;
				let at_cell_changes: HToMut<HMapP<MattersBasic<DIM>,MapToChange>> =at_cell.0.to_mut().sculpt().0;
				// let a_num=Num::default();
				// let dawdawd:Momentum<DIM>=Momentum();
				at_cell_changes.zip(grid_cell_changes).map(Poly(HChangeApplyChange));
				
			},
			None => {
				warn!("entity {} at grid idx {:?} is not in grid cell",e,pos.0);
				
				let at_cell_changes: HToMut<HMapP<MattersBasic<DIM>,MapToChange>> =at_cell.0.to_mut().sculpt().0;
				at_cell_changes.map(Poly(HChangeGetAndReset));
			},
		}
	});
}

pub fn plugin<const DIM:usize>(app:&mut App){
	app.add_systems(schedule_pre_sim(), 
		set_at_grid_gas::<DIM>.into_configs()
		.config_processing::<HList!(AtGridCell<DIM>),HNil,HList!(AtGridCellGas<DIM>)>()
	);
	app.add_systems(schedule_apply_change(), apply_at_grid_gas_change::<DIM>);
}