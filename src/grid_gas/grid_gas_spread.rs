use bevy_ecs::system::{Query, Res};
use statistic_physics::matters::MattersState;

use crate::{components::stat_component::StatComponent, grid::grid::{GridCell, GridResource}, transform::tramsform::Vel};

use super::grid_gas_edge_type::GridGasEdgeWall;

use physics_basic::stats::*;
use statistic_physics::stats::*;

pub fn grid_gas_spread_edge_wall<const XSIZE:usize,const YSIZE:usize>(query:Query<(&GridCell,&StatComponent< Mass>,&StatComponent<Momentum>,&StatComponent<Energy>,&StatComponent<Kinetic>,&StatComponent<Internal>,&StatComponent<Vel>,&StatComponent<VelVar1Dir>,&StatComponent<VelVarSq1Dir>)>,res_grid:Res<GridResource<XSIZE,YSIZE>>,_res_gas_grid_edge_wall:Res<GridGasEdgeWall>){
    query.into_iter().for_each(|(
        grid_cell,
        mass,
        momentum,
        energy,
        kinetic,
        internal,
        v_mean,
        v_var_1dir,
        v_var_sq_1dir
    )|{
        let matter_state=MattersState{
            mass: todo!(),
            momentum: todo!(),
            energy: todo!(),
            kinetic: todo!(),
            internal: todo!(),
            v_mean: todo!(),
            v_var_sq: todo!(),
            v_var: todo!(),
            v_var_sq_1dir: todo!(),
            v_var_1dir: todo!(),
        }
    });
}