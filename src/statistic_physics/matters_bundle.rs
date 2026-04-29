

use bevy::ecs::bundle::Bundle;

// use crate::components::{change_component::ChangeComponent, determining_components::DeterminingComponent, stat_component::StatComponent};


use frunk::{Poly, hlist::HMappable};

use nalgebra::RealField;
use statistic_physics::{formulas::calculate_matters_state, matters::MattersBasic};

use wacky_bag_bevy::utils::stat_for_hlist::{MapToChange, MapToDetermining, MapToStat};

pub fn matters_bundle<Num:RealField+Copy+Default,const DIM:usize>(matters_basic:MattersBasic<Num,DIM>)->impl Bundle{
	let stat_matters_full=calculate_matters_state(matters_basic).map(Poly(MapToStat));
	let matters_determing=<MattersBasic<Num,DIM> as HMappable<Poly<MapToDetermining>>>::Output::default();
	let matters_change=<MattersBasic<Num,DIM> as HMappable<Poly<MapToChange>>>::Output::default();
	(stat_matters_full+matters_determing+matters_change).into_tuple2()
}
