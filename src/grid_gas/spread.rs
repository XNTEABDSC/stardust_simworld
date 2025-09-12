use bevy_ecs::{system::{Query, Res, ResMut}, world::{EntityMut, World}};
use frunk::HList;
use wacky_bag::structures::n_dim_array::NDimArray;

use crate::{components::stat_component::StatComponent, grid::grid::{GridIndexer, GridResource}, grid_gas::resource::{GridGasDeltaResource, GridGasStatResource}, resources::simulate_speed::SimulateSpeed, transform::tramsform::Vel};

use wacky_bag_fixed::vec_fix::VecFix;
use super::edge_type::GridGasEdgeWall;

use physics_basic::{num::Num, stats::*};
use statistic_physics::stats::*;


pub fn grid_gas_spread_edge_wall<const DIM:usize>(grid_indexer:GridIndexer<DIM>,grid_gas_stat:Res<GridGasStatResource<DIM>>,grid_gas_delta:ResMut<GridGasDeltaResource<DIM>>,_res_gas_grid_edge_wall:Res<GridGasEdgeWall>,simulate_speed:Res<SimulateSpeed>){
    let volume=Num::ONE;
    let edge_len=Num::ONE;
    let spf=simulate_speed.second_per_frame;
    for grid_idx in grid_indexer.iter(){
        let stats=grid_gas_stat.0.get_with_neiborhoods_loop(grid_idx).unwrap();
        //let deltas=grid_gas_delta.get_mut_with_
        // if let Some(things)=things_may{
        //     let cur_gas=things.cur;
        //     //let cur_delta=things
        // }
    }

}

// pub fn grid_gas_spread_edge_wall_<const DIM:usize>(wld:&mut World,res_grid:Res<GridResource<DIM>>,_res_gas_grid_edge_wall:Res<GridGasEdgeWall>,simulate_speed:Res<SimulateSpeed>){
    
//     let volume=Num::ONE;
//     let edge_len=Num::ONE;
//     let spf=simulate_speed.second_per_frame;
    
//     let grids=wld.entity_mut(res_grid.grid_entities.values().as_slice());
//     let mut ndgrid=NDimArray::new(res_grid.grid_entities.n_dim_index(),grids);
//     for idx in res_grid.grid_entities.n_dim_index().iter(){
//         let things_may=ndgrid.get_mut_with_neiborhoods_mut_loop(idx);
//         if let Some(mut things)=things_may{
//             let cur=things.cur;
//             let cur_components=cur.components::<
//                     (&GridCell<DIM>,&StatComponent< Mass>,&StatComponent<Momentum<DIM>>,&StatComponent<Energy>,&StatComponent<Kinetic>,&StatComponent<Internal>,&StatComponent<Vel<DIM>>,&StatComponent<VelVar1Dir>,&StatComponent<VelVarSq1Dir>,&StatComponent<TimePass>)
//                 >();
//             let (
//                 grid_cell,
//                 mass,
//                 momentum,
//                 energy,
//                 kinetic,
//                 internal,
//                 v_mean,
//                 v_var_1dir,
//                 v_var_sq_1dir,
//                 time_pass
//             )=cur_components;
//             let dt=spf*time_pass.0.0;
//             things.neiborhoods.iter_mut().enumerate().for_each(
//                 |(dim,a)|
//                 {
//                 let helper=|obj:&mut EntityMut,dir_vec:VecFix<DIM>|{
//                     let delta=statistic_physics::formulas::gas_cell_spread_to_side()
//                 };
//             });
//         }

//     }
    
//     /*
//     for idx in res_grid.grid_entities.n_dim_index().iter(){
//         let things_may=res_grid.grid_entities.get_with_neiborhoods_loop(idx);
//         if let Some(things)=things_may{
//             let core_grid=wld.entity_mut(things.cur);
//             for dim in 0..DIM{

//             }
//         }
//     } */
// }

// pub fn grid_gas_spread_edge_wall<const DIM:usize>(query:Query<(&GridCell<DIM>,&StatComponent< Mass>,&StatComponent<Momentum<DIM>>,&StatComponent<Energy>,&StatComponent<Kinetic>,&StatComponent<Internal>,&StatComponent<Vel<DIM>>,&StatComponent<VelVar1Dir>,&StatComponent<VelVarSq1Dir>)>,res_grid:Res<GridResource<DIM>>,_res_gas_grid_edge_wall:Res<GridGasEdgeWall>){
    
    
    
    
//     query.into_iter().for_each(|(
//         grid_cell,
//         mass,
//         momentum,
//         energy,
//         kinetic,
//         internal,
//         v_mean,
//         v_var_1dir,
//         v_var_sq_1dir
//     )|{
//         //todo!();
//         let volume=Num::ONE;
//         let edge_len=Num::ONE;
//         let dt=Num::ONE;
//         let edge_dir_vec=VecFix::<DIM>::zeros();//Vec2Fix::vec2_new(Num::ONE, Num::ZERO);



        
//         /*
//         let matters_hl=hlist![&mass.0,&momentum.0,&energy.0,&kinetic.0,&internal.0,&v_mean.0,&v_var_1dir.0,&v_var_sq_1dir.0];
//         let (dwadawd,_):(HList!(&Mass,&Vel,&VelVarSq1Dir,&VelVar1Dir),_)=matters_hl.sculpt();
        
//         statistic_physics::formulas::gas_cell_spread_to_side(dwadawd.into(), volume, edge_dir_vec, edge_len, dt);
//          */
        
//         statistic_physics::formulas::gas_cell_spread_to_side((mass,v_mean,v_var_sq_1dir,v_var_1dir), volume, edge_dir_vec, edge_len, dt);
//         //statistic_physics::formulas::gas_cell_spread_to_side(select_to_tuple(matters_hl), volume, edge_dir_vec, edge_len, dt);
//         //statistic_physics::formulas::gas_cell_spread_to_side(matters_hl.sculpt().0.into(), volume, edge_dir_vec, edge_len, dt);
//     });
// }