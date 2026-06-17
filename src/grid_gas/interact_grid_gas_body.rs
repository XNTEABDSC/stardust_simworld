use bevy::ecs::system::{Query, Res};
use frunk::Poly;
use log::info;
use nalgebra::{Const, DefaultAllocator, DimName, RealField, allocator::Allocator};
use physics_basic::rotation::{DimNameToSoDimName, DimNameToSoDimNameType};
use statistic_physics::formulas::{InteractGasCellBodyBodyChange, InteractGasCellBodyBodyMatters, interact_gas_cell_body_simple};
use wacky_bag::utils::{h_list_helpers::{HMapP, HToRef, MapRef, Sum}, output_func::HMappableFrom, select_zip::HSelectZippable, type_fn::ChainFunc};
use wacky_bag_bevy::utils::{h_list_query_data_old::HQueryData, stat_for_hlist::{HChangeAdd, MapFromStatRef, MapToChange, MapToStat}};

use crate::{grid_gas::at_grid_gas::AtGridCellGas, simulate_speed::simulate_speed::SimulateSpeed};


pub fn interact_grid_gas_body<Num,const DIM:usize>(q:Query<(&AtGridCellGas<Num,DIM>, HQueryData< 
	HToRef<
		Sum<
			HMapP<InteractGasCellBodyBodyMatters<Num,DIM>,MapToStat>,
			HMapP<InteractGasCellBodyBodyChange<Num,DIM>,MapToChange>
		>
	> 
	>)>,
	time:Res<SimulateSpeed<Num>>
)
where 
	Num:RealField+Copy,
    Const<DIM>: DimNameToSoDimName + DimName,
    DefaultAllocator: 
		Allocator<DimNameToSoDimNameType<DIM>>
        + Allocator<DimNameToSoDimNameType<DIM>, DimNameToSoDimNameType<DIM>, Buffer<Num>:Send+Sync >,
{

	let dt=time.second_per_frame;
	
	q.par_iter().for_each(|(gas_c,b_stats)|{

		let gas=gas_c.0.to_ref();

		let (b_stats_c,b_change_c)=b_stats.sculpt();

		let b_stats_o:HToRef<InteractGasCellBodyBodyMatters<Num,DIM>>= HMappableFrom::output_map(b_stats_c, Poly(MapFromStatRef));//b_stats.map(Poly(MapFromStatRef));

		let (gas_stats_c,gas_c_r):(_,_)=gas.sculpt();

		let stats=HMappableFrom::output_map(gas_stats_c, Poly(MapFromStatRef));

		let (gas_change,b_change)=interact_gas_cell_body_simple::<Num,DIM>(stats,b_stats_o,Num::from_f64(0.1).unwrap(),dt);
		
		let gas_changes_c=gas_c_r.sculpt().0;

		b_change.zip(b_change_c).map(Poly(HChangeAdd));

		// info!("{gas_change:.3?}");

		gas_change.select_zip(
			Poly(ChainFunc(MapToChange,MapRef::default())), 
			gas_changes_c
		).map(Poly(HChangeAdd));

	});
}