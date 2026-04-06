

use bevy::ecs::bundle::Bundle;

// use crate::components::{change_component::ChangeComponent, determining_components::DeterminingComponent, stat_component::StatComponent};


use frunk::{Poly, hlist::HMappable};

use nalgebra::RealField;
use statistic_physics::{formulas::calculate_matters_state, matters::MattersBasic};

use wacky_bag_bevy::utils::stat_for_hlist::{MapToChange, MapToDetermining, MapToStat};

// const DIM:usize=3;

// derive Bundle has touble with <const DIM:usize>
// #[derive(Default,Bundle)]
// pub struct MattersBundle<const DIM:usize>{
//     pub mass:Stat<Mass>,
//     pub momentum:Stat<Momentum<DIM>>,
//     pub energy:Stat<Energy>,
//     pub kinetic:Stat<Kinetic>,
//     pub internal:Stat<Internal>,
//     pub vel_var_sq:Stat<VelVarSq>,
//     pub vel_var:Stat<VelVar>,
//     pub vel_var_sq1_dir:Stat<VelVarSq1Dir>,
//     pub vel_var1_dir:Stat<VelVar1Dir>,

//     pub mass_determining:Determining<Mass>,
//     pub energy_determining:Determining<Energy>,
//     pub momentum_determining:Determining<Momentum<DIM>>,
	
//     pub mass_change:Change<Mass>,
//     pub momentum_change:Change<Momentum<DIM>>,
//     pub energy_change:Change<Energy>
// }


pub fn matters_bundle<Num:RealField+Copy+Default,const DIM:usize>(matters_basic:MattersBasic<Num,DIM>)->impl Bundle{
	let stat_matters_full=calculate_matters_state(matters_basic).map(Poly(MapToStat));
	let matters_determing=<MattersBasic<Num,DIM> as HMappable<Poly<MapToDetermining>>>::Output::default();
	let matters_change=<MattersBasic<Num,DIM> as HMappable<Poly<MapToChange>>>::Output::default();
	(stat_matters_full+matters_determing+matters_change).into_tuple2()
	// stat_matters_full.
}

// unsafe impl<const DIM: usize> Bundle for MattersBundle<DIM> {
//     fn component_ids(
//         components: &mut bevy::ecs::component::ComponentsRegistrator,
//     ) -> impl Iterator<Item = ComponentId> + use<DIM> {
//         // frunk::from_generic(repr)
// 		// let a:<Self as frunk::Generic>::Repr=todo!();
// 		let p=<
// 			<Self as frunk::Generic>::Repr
// 			as HMappable<Poly<MapToPhantom>>
// 		>::Output::default();

// 		struct F;
// 		impl<'w,Acc,X> Func<( (Acc,&mut bevy::ecs::component::ComponentsRegistrator<'w>),PhantomData<X> )> for F 
// 			where Acc:Iterator<Item = ComponentId>,X:Bundle
// 		{
// 			type Output;
		
// 			fn call(i: ( (Acc,&mut bevy::ecs::component::ComponentsRegistrator<'w>),PhantomData<X> )) -> _ {
// 				let a=i.0.0.chain(
// 					X::component_ids(i.0.1)
// 				);
// 				a
// 			}
// 		}

// 		p.foldl(folder, acc);

// 		std::iter::empty()
// 		.chain(Stat::<Mass>::component_ids(components))

// 	}

// 	fn get_component_ids(components: &bevy::ecs::component::Components) -> impl Iterator<Item = Option<ComponentId>> {
// 		todo!()
// 	}
// }
