use bevy::{ecs::bundle::Bundle, utils::default};
use frunk::{HList, Poly, hlist,hlist::Plucker, hlist_pat};
use nalgebra::{Const, DefaultAllocator, DimMin, DimName, RealField, allocator::Allocator};
use physics_basic::{body::{PhyBodyBasic, PhyBodyBasicStat, ShapeSphere, calculate_angular_state, calculate_body_state_full, calculate_position_state}, rotation::{DimNameToSoDimName, DimNameToSoDimNameType}, stat_to_change_type::{HMapStatToChangeTypeZ, MapStatToChangeTypeZ}};
use wacky_bag::utils::{h_list_helpers::{HMapP, HZip, MapToPhantom}, type_fn::MapPhantomType};
use wacky_bag_bevy::utils::stat_for_hlist::{MapToChange, MapToDetermining, MapToStat};
use physics_basic::stats::*;
use physics_basic::rotation::*;
use statistic_physics::{formulas::{calculate_density, calculate_vel_var}, stats::*};


pub fn phy_body_bundle<Num,const DIM:usize>(basics:PhyBodyBasic<Num,DIM>)
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
		>,MapPhantomType>,MapToChange>=
		default();
		
	(stat_matters_full+matters_determing+matters_change).into_tuple2()
}


pub type PhyBodyStatisticBundleStats<Num,const DIM:usize>=HList!(
	TimePass<Num>,
	ShapeSphere<Num,DIM>,
	Mass<Num>,
	Pos<Num,DIM>,
	Momentum<Num,DIM>,
	AngularInertia<Num,DIM>,
	AngularMomentum<Num,DIM>,
	Rotation<Num,DIM>,
	Internal<Num>,
);

pub type PhyBodyStatisticBundleDetermining<Num,const DIM:usize>=HList!(
	Mass<Num>,
	Pos<Num,DIM>,
	Momentum<Num,DIM>,
	AngularInertia<Num,DIM>,
	AngularMomentum<Num,DIM>,
	Rotation<Num,DIM>,
	Internal<Num>,
);
pub fn phy_body_statistic_bundle<Num,const DIM:usize>(stats:PhyBodyStatisticBundleStats<Num,DIM>)
->impl Bundle
where 
	Num:RealField+Copy+Default,
	Const<DIM>: DimNameToSoDimName + DimName,
	DefaultAllocator: Allocator<DimNameToSoDimNameType<DIM>, DimNameToSoDimNameType<DIM>, Buffer<Num> :Send+Sync >+Allocator<DimNameToSoDimNameType<DIM>>,
    DimNameToSoDimNameType<DIM>:
        DimMin<DimNameToSoDimNameType<DIM>, Output = DimNameToSoDimNameType<DIM>>,
{
	// let stats=h_extend_by_fn_ref(stats, calculate_position_state);
	let stats=calculate_position_state(stats.to_ref().sculpt().0)+stats;
	let stats=calculate_angular_state(stats.to_ref().sculpt().0)+stats;
	let stats=calculate_vel_var::<Num,DIM>(stats.to_ref().sculpt().0)+stats;

	let hlist_pat![k,agk,i]:HList!(&Kinetic<Num>,&AngularKinetic<Num>,&Internal<Num>)=stats.to_ref().sculpt().0;
	let energy=Energy(k.0+agk.0+i.0);
	let stats=hlist![energy]+stats;

	let shape:&ShapeSphere<Num,DIM>=stats.to_ref().pluck().0;
	let stats=hlist![ shape.volume() ]+stats;

	let stats = calculate_density::<Num>(stats.to_ref().sculpt().0)+stats;

	let stats=stats.map(Poly(MapToStat));

	let changes: HMapP<HMapStatToChangeTypeZ<PhyBodyStatisticBundleDetermining<Num,DIM>,_>,MapToChange>=default();

	let determinings: HMapP<PhyBodyStatisticBundleDetermining<Num,DIM>,MapToDetermining>=default();

	// let changes=stats.
	return (stats+changes+determinings).into_tuple2();
}