use bevy::{ecs::bundle::Bundle, utils::default};
use frunk::{Poly, hlist::{HMappable, HZippable}};
use nalgebra::{Const, DefaultAllocator, DimMin, DimName, RealField, allocator::Allocator};
use physics_basic::{body::{PhyBodyBasic, PhyBodyBasicStat, calculate_body_state, calculate_body_state_full}, rotation::{DimNameToSoDimName, DimNameToSoDimNameType}, stat_to_change_type::MapStatToChangeTypeZ};
use wacky_bag::utils::{h_list_helpers::{HMapP, HZip, MapToPhantom}, type_fn::MapPhantomUnwrap};
use wacky_bag_bevy::utils::stat_for_hlist::{MapToChange, MapToDetermining, MapToStat};



pub fn bundle<Num,const DIM:usize>(basics:PhyBodyBasic<Num,DIM>)
	->impl Bundle
where 
	Num:RealField+Copy+Default,
	Const<DIM>: DimNameToSoDimName + DimName,
	DefaultAllocator: Allocator<DimNameToSoDimNameType<DIM>, DimNameToSoDimNameType<DIM>, Buffer<Num> :Send+Sync >+Allocator<DimNameToSoDimNameType<DIM>>,
    DimNameToSoDimNameType<DIM>:
        DimMin<DimNameToSoDimNameType<DIM>, Output = DimNameToSoDimNameType<DIM>>,
{
	let stat_matters_full=calculate_body_state_full(basics).map(Poly(MapToStat));
	let matters_determing:HMapP<PhyBodyBasicStat<Num,DIM>,MapToDetermining>=default();
	let matters_change:
		HMapP< HMapP< HMapP<
			HZip<
				HMapP<
					PhyBodyBasicStat<Num,DIM>
					,MapToPhantom> 
				,_
			>,MapStatToChangeTypeZ
		>,MapPhantomUnwrap>,MapToChange>=
		default();
	(stat_matters_full+matters_determing+matters_change).into_tuple2()
}