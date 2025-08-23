use bevy_ecs::system::{Query, Res};
use frunk::HList;

use crate::{components::stat_component::StatComponent, grid::grid::{GridCell, GridResource}, transform::tramsform::Vel};

use super::grid_gas_edge_type::GridGasEdgeWall;

use physics_basic::{num::Num, stats::*, vec2_fix::Vec2Fix};
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
        //todo!();
        let volume=Num::ONE;
        let edge_len=Num::ONE;
        let dt=Num::ONE;
        let edge_dir_vec=Vec2Fix::vec2_new(Num::ONE, Num::ZERO);



        
        /*
        let matters_hl=hlist![&mass.0,&momentum.0,&energy.0,&kinetic.0,&internal.0,&v_mean.0,&v_var_1dir.0,&v_var_sq_1dir.0];
        let (dwadawd,_):(HList!(&Mass,&Vel,&VelVarSq1Dir,&VelVar1Dir),_)=matters_hl.sculpt();
        
        statistic_physics::formulas::gas_cell_spread_to_side(dwadawd.into(), volume, edge_dir_vec, edge_len, dt);
         */
        
        statistic_physics::formulas::gas_cell_spread_to_side((mass,v_mean,v_var_sq_1dir,v_var_1dir), volume, edge_dir_vec, edge_len, dt);
        //statistic_physics::formulas::gas_cell_spread_to_side(select_to_tuple(matters_hl), volume, edge_dir_vec, edge_len, dt);
        //statistic_physics::formulas::gas_cell_spread_to_side(matters_hl.sculpt().0.into(), volume, edge_dir_vec, edge_len, dt);
    });
}