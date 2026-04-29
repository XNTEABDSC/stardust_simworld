
use std::str::FromStr;

use bevy::ecs::system::Res;
use frunk::{Poly, hlist};
use nalgebra::{RealField, SVector};
use wacky_bag::math::normal_cdf::NormalCdfConsts;
use wacky_bag::utils::num_extend::NumExtend;
use wacky_bag::{structures::n_dim_array::{dim_dir::DimDir, t_n_dim_array::{TNDimArrayForEachEdgeParallel, TNDimArrayIterPairParallel}}, utils::{h_list_helpers::MapNeg, output_func::HMappableFrom, select_zip::HSelectZippable}};
use wacky_bag_bevy::utils::{stat_for_hlist::{HAddChange, HChangeTransfer, MapFromStatRef, Select2ChangeRef, SelectChangeRef}, thread_scope::ComputeTaskPoolScopeCreater};


use crate::grid_gas::edge_type::GridGasEdgeConst;
use crate::{grid::grid::GridData, grid_gas::{edge_type::GridGasEdgeWall, resource::GridGasResource}, simulate_speed::simulate_speed::SimulateSpeed};


use physics_basic::{stats::Momentum};
use statistic_physics::formulas;

/*
e^x -> 2^x = e^(x*log2)
e^log2=2
*/


pub fn grid_gas_spread<Num:RealField+Copy+Default+NormalCdfConsts<Marker>,const DIM:usize,Marker>(grid_gas:Res<GridGasResource<Num,DIM>>,simulate_speed:Res<SimulateSpeed<Num>>,grid_size:Res<GridData<Num,DIM>>) {
	let spf=simulate_speed.second_per_frame;
	let edge_len:Num=grid_size.grid_edge_len;
	// let volume:Num=grid_size.grid_volume;
	//edge_len.exp2();
	//cordic::exp(edge_len/Num::LOG2_E);

	grid_gas.0.iter_pair_2_side_parallel(&|a,b,dim_dir|{
		let edge_dir_vec=dim_dir.to_dir_vec().map(|a|Num::from_isize(a).unwrap()).into();
		let r=formulas::gas_cell_spread_to_side(
			HMappableFrom::output_map(
				a
				.to_ref()
				.sculpt().0,
				Poly(MapFromStatRef)
			)
			, edge_dir_vec, edge_len, spf);

		// r.clone().select_zip(Poly(SelectChangeRef::default()), b.to_ref().sculpt().0)
		// .map(Poly(HAddChange));

		// r.map(Poly(MapNeg))
		// .select_zip(Poly(SelectChangeRef::default()), a.to_ref().sculpt().0)
		// .map(Poly(HAddChange));
		r.select_zip(Poly(Select2ChangeRef::default()), a.to_ref().zip(b.to_ref()).sculpt().0).map(Poly(HChangeTransfer));
	}, &ComputeTaskPoolScopeCreater);
}

pub fn grid_gas_spread_edge_wall<Num:RealField+Copy+Default+NormalCdfConsts<Marker>,const DIM:usize,Marker:Send+Sync+'static>(grid_gas:Res<GridGasResource<Num,DIM>>,simulate_speed:Res<SimulateSpeed<Num>>,grid_size:Res<GridData<Num,DIM>>,_res_gas_grid_edge_wall:Res<GridGasEdgeWall<Num,DIM,Marker>>) {
	let dt=simulate_speed.second_per_frame;
	let edge_len:Num=grid_size.grid_edge_len;

	for dim in 0..DIM {
		for dir in 0..=1 {

			let dim_dir=DimDir{dim,dir_positive: (dir as i32).is_positive()};
			let dir_vec=dim_dir.to_dir_vec().map(|a|Num::from_isize(a).unwrap()).into();

			grid_gas.0.for_each_edge_parallel(dim_dir, &|a,_|{

				let r=formulas::gas_cell_spread_to_side(
					HMappableFrom::output_map(
						a
						.to_ref()
						.sculpt().0,
						Poly(MapFromStatRef)
					)
					, dir_vec, edge_len, dt);

				let mut m:Momentum<Num,DIM>=r.pluck().0;

				for i in 0..DIM {
					m.0[(0,dim)]=if i==dim {
						-m.0[(0,dim)]*Num::p2()
					}else {
						Num::zero()
					}
				}

				(hlist![m]).select_zip(Poly(SelectChangeRef::default()), a.to_ref().sculpt().0)
				.map(Poly(HAddChange));

			}
			, &ComputeTaskPoolScopeCreater);
		}
	}

	// grid_gas.0.iter_pair_parallel(&|a,b,i|{
	// 	let edge_dir_vec: VecFix<DIM>=array::from_fn(|z|if z==i {Num::from_num(1)} else {Num::from_num(0)}).into();
	// 	let r=formulas::gas_cell_spread_to_side(
	// 		HMappableFrom::output_map(
	// 			a
	// 			.to_ref()
	// 			.sculpt().0,
	// 			Poly(MapFromStatRef)
	// 		)
	// 		, volume, edge_dir_vec, edge_len, spf);

	// 	r.select_zip(Poly(SelectChangeRef::default()), b.to_ref().sculpt().0)
	// 	.map(Poly(HAddChange));
	// }, &ComputeTaskPoolScopeCreater);
}

pub fn grid_gas_spread_edge_const<Num:RealField+Copy+Default+NormalCdfConsts<Marker>,const DIM:usize,Marker:Send+Sync+'static>(grid_gas:Res<GridGasResource<Num,DIM>>,simulate_speed:Res<SimulateSpeed<Num>>,grid_size:Res<GridData<Num,DIM>>,res_gas_grid_edge_const:Res<GridGasEdgeConst<Num,DIM,Marker>>) {
	let dt=simulate_speed.second_per_frame;
	let edge_len:Num=grid_size.grid_edge_len;

	let edge_stat=res_gas_grid_edge_const.const_matters;

	for dim in 0..DIM {
		for dir in 0..=1 {

			let dim_dir_rev=DimDir{dim,dir_positive: !(dir as i32).is_positive()};
			let dir_vec_rev=dim_dir_rev.to_dir_vec().map(|a|Num::from_isize(a).unwrap()).into();
			
			let edge_spread=formulas::gas_cell_spread_to_side(edge_stat.to_ref(), dir_vec_rev, edge_len, dt);

			let dim_dir=DimDir{dim,dir_positive: (dir as i32).is_positive()};
			let dir_vec=dim_dir.to_dir_vec().map(|a|Num::from_isize(a).unwrap()).into();

			grid_gas.0.for_each_edge_parallel(dim_dir, &|a,_|{

				let r=formulas::gas_cell_spread_to_side(
					HMappableFrom::output_map(
						a
						.to_ref()
						.sculpt().0,
						Poly(MapFromStatRef)
					)
					, dir_vec, edge_len, dt);
				
				r.map(Poly(MapNeg))
				.select_zip(Poly(SelectChangeRef::default()), a.to_ref().sculpt().0)
				.map(Poly(HAddChange));

				edge_spread
				.select_zip(Poly(SelectChangeRef::default()), a.to_ref().sculpt().0)
				.map(Poly(HAddChange));
			}
			, &ComputeTaskPoolScopeCreater);
		}
	}

	// grid_gas.0.iter_pair_parallel(&|a,b,i|{
	// 	let edge_dir_vec: VecFix<DIM>=array::from_fn(|z|if z==i {Num::from_num(1)} else {Num::from_num(0)}).into();
	// 	let r=formulas::gas_cell_spread_to_side(
	// 		HMappableFrom::output_map(
	// 			a
	// 			.to_ref()
	// 			.sculpt().0,
	// 			Poly(MapFromStatRef)
	// 		)
	// 		, volume, edge_dir_vec, edge_len, spf);

	// 	r.select_zip(Poly(SelectChangeRef::default()), b.to_ref().sculpt().0)
	// 	.map(Poly(HAddChange));
	// }, &ComputeTaskPoolScopeCreater);
}

// pub fn grid_gas_spread_edge_wall<const DIM:usize>(grid_indexer:GridIndexer<DIM>,grid_gas_stat:Res<GridGasStatResource<DIM>>,grid_gas_delta:ResMut<GridGasDeltaResource<DIM>>,_res_gas_grid_edge_wall:Res<GridGasEdgeWall>,simulate_speed:Res<SimulateSpeed>){
//     let volume=Num::ONE;
//     let edge_len=Num::ONE;
//     let spf=simulate_speed.second_per_frame;
//     // for grid_idx in grid_indexer.iter(){
//     //     let stats=grid_gas_stat.0.get_with_neiborhoods_loop(grid_idx).unwrap();
//     //     //let deltas=grid_gas_delta.get_mut_with_
//     //     // if let Some(things)=things_may{
//     //     //     let cur_gas=things.cur;
//     //     //     //let cur_delta=things
//     //     // }
//     // }

// }

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