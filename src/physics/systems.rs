use std::{marker::PhantomData, ops::DerefMut};

use bevy::{app::{App, PluginGroup, PluginGroupBuilder}, ecs::{query::{QueryData, QueryItem, ReadOnlyQueryData}, schedule::{IntoScheduleConfigs, ScheduleConfigs}, system::{Query, ScheduleSystem}}, prelude::SystemParamFunction};
use frunk::{Func, HList, HNil, Poly, ToMut, ToRef, hlist::{HFoldLeftable, HMappable, HZippable}};
use nalgebra::{Const, DefaultAllocator, DimMin, DimName, RealField, allocator::Allocator};
use physics_basic::{body::{calculate_angular_state, calculate_position_state, calculate_rotation_matrix, collect_angular_state_det_agi_agm_change_agv, collect_position_state_det_pos_momentum_change_vel}, rotation::{AllocatorSyncSq, AllocatorSyncVMSq, ConstDimToSoDimT, DimSquare, DimToSoDim}, stats::Vel};
use statistic_physics::formulas::{calculate_density, calculate_vel_var};
use wacky_bag_hlist::{chain_fn::ChainFunc, h_list_helpers::{FoldVecPush, HMapP, HToMut, HToRef, HTypeFnToMapper, HTypeMapP, HZip, MapDeref, MapDerefMut, MapFromMut, MapFromRef, MapMut, MapRef, MapToPhantom}, type_fn::{ReverseFunc, TypeFnAsPhantomFn}};
use wacky_bag::{structures::owned::Owned, utils::{default_of::default, }};
use wacky_bag_bevy::{stat_component::stat_apply_change::StatChangeToApplyChanges, system::{multi_sets::{FoldScheduleConfigsAfterSets, FoldScheduleConfigsBeforeSets, FoldScheduleConfigsInSet, ScheduleConfigsAfterSets}, processing_system::{MapToProcessingSystemSet, ScheduleConfigsProcessing, processing_system_sets}}, utils::{h_list_query::{HToQuery, HToQueryType}, stat_for_hlist::{HChangeAdd, HChangeAddG, HStatSet, HTakeChagne, HTakeChagneG, MapFromStatRef, MapToChange, MapToDetermining, MapToStat, MapToWith}}};


use crate::{physics::bundle::PhyBodyStatisticBundleDetermining, schedule::schedule_apply_change};

/// use [to_calculate_system] for system
#[derive(Debug,Default,Clone, Copy)]
pub struct CalculateChangeSystem<F>(pub F);

/// convert a Fn(HList!(&A,&B,&C))->HList!(D,E,F) into a system with Query<(&Stat<A>,&Stat<B>,&Stat<C>,&Chagne<D>,&Change<E>,&Change<F>)>
pub fn to_calculate_change_system<F,FIR,FO>(f:F)->
impl SystemParamFunction<CalculateChangeSystemMarker<(FIR,FO)>,In = (),Out = ()>
// CalculateSystem<F>
	where F:Fn(FIR)->FO+Send+Sync+'static,
	CalculateChangeSystem<F>:SystemParamFunction<CalculateChangeSystemMarker<(FIR,FO)>,In = (),Out = ()>
{
	CalculateChangeSystem(f)
}

#[derive(Debug,Default,Clone, Copy)]
pub struct CalculateChangeSystemMarker<A>(pub A);

impl<
	F,
	FIR,FO,
	FIRS, FOC, FORC,
	FIRSQ, FORCQ,
	FIRSQR, FORCQR,
	// FIR2,
	FO2,
	// M
> SystemParamFunction<
	// (Self,FIR,FO)
	// (FIR,FO)
	// HList!(FIR,FO)
	CalculateChangeSystemMarker<(FIR,FO)>
	
	// (<<FIRSQR as QueryData>::Item<'static,'static> as HMappable<Poly<MapFromStatRef>>>::Output, FO2)
> 
for CalculateChangeSystem<F>
where 
	// Self:GetCalculateSystemMarker<M,Marker = CalculateSystemMarker<(FIR,FO)>>,
	F:Send+Sync+'static,
	for<'a,'w,'s> &'a F:
		Fn(FIR)->FO+
		Fn( <<FIRSQR as QueryData>::Item<'w,'s> as HMappable<Poly<MapFromStatRef>>>::Output )->FO2,
	
	// F:Fn(FIR)->FO,

	FIR:HMappable<Poly<HTypeFnToMapper<ReverseFunc<MapFromStatRef>>>,Output = FIRS>,
	FIRS:HMappable<Poly<MapFromStatRef>,Output = FIR>,

	FO:HMappable<Poly<MapToChange>,Output = FOC>,
	FOC:HMappable<Poly<HTypeFnToMapper<MapRef<'static>>>,Output = FORC>,
	// for<'a> FOC:HMappable<Poly<HTypeFnToMapper<MapRef<'a>>>>,

	// FIRS:'static,FORC:'static,
	FIRS:HToQuery<Output = FIRSQ>,
	FORC:HToQuery<Output = FORCQ>,
	// for<'a> HMapP<FOC,HTypeFnToMapper<MapRef<'a>>>:HToQuery,

	FIRSQ:'static+QueryData<ReadOnly = FIRSQR>,
	FORCQ:'static+QueryData<ReadOnly = FORCQR>,
	// for<'a> HToQueryType<HMapP<FOC,HTypeFnToMapper<MapRef<'a>>>>:'static+QueryData,

	FIRSQR:ReadOnlyQueryData,
	FORCQR:ReadOnlyQueryData,
	// for<'a> <HToQueryType<HMapP<FOC,HTypeFnToMapper<MapRef<'a>>>> as QueryData>::ReadOnly:ReadOnlyQueryData,

	for<'w,'s> <FIRSQR as QueryData>::Item<'w,'s>: HMappable<Poly<MapFromStatRef>/*,Output = FIR2*/>,
	// for<'w,'s> <FIRSQR as QueryData>::Item<'w,'s>: HMappable<Poly<MapFromStatRef>,Output = FIR2>,
	
	// for<'a,'w,'s> FO2:HZippable< <<HToQueryType<HMapP<FOC,HTypeFnToMapper<MapRef<'a>>>> as QueryData>::ReadOnly as QueryData>::Item<'w,'s>,Zipped : HMappable<Poly<HChangeAdd>> >
	for<'w,'s> FO2:HZippable< <FORCQR as QueryData>::Item<'w,'s>,Zipped : HMappable<Poly<HChangeAdd>> >
{
    type In = ();
    type Out = ();
    type Param = Query<'static,'static,(
		HToQueryType<FIRS>,
		HToQueryType<HMapP<FOC,HTypeFnToMapper<MapRef<'static>>>>
	)>;
	
    fn run(
            &mut self,
            _input:(),
            param_value: bevy::ecs::system::SystemParamItem<Self::Param>,
        ) -> () 
	{
		param_value.par_iter().for_each(|(a,b)|{
			let firs=a;
			let forc=b;
			let fir=firs.map(Poly(MapFromStatRef));
			fn call_inner<FI,FO>(f:impl Fn( FI )->FO,i:FI)->FO{
				f(i)
			}
			let f:&F=&self.0;
			// let fo=(f)(fir);
			let fo=call_inner(f,fir);
			fo.zip(forc).map(Poly(HChangeAdd));
		});
		
    }
}


/// use [to_calculate_system] for system
#[derive(Debug,Default,Clone, Copy)]
pub struct CalculateStatSystem<F>(pub F);

/// convert a Fn(HList!(&A,&B,&C))->HList!(D,E,F) into a system with Query<(&Stat<A>,&Stat<B>,&Stat<C>,&mut Stat<D>,&mut Stat<E>,&mut Stat<F>)>
pub fn to_calculate_stat_system<F,FIR,FO>(f:F)->
impl SystemParamFunction<(FIR,FO),In = (),Out = ()>
// CalculateSystem<F>
	where F:Fn(FIR)->FO+Send+Sync+'static,
	CalculateStatSystem<F>:SystemParamFunction<(FIR,FO),In = (),Out = ()>
{
	CalculateStatSystem(f)
}

#[derive(Debug,Default,Clone, Copy)]
pub struct CollectChangeSystem<F>(pub F);

pub fn to_collect_change_system<F,FIR,FIC,FO>(f:F)->
impl SystemParamFunction<(FIR,FIC,FO),In = (),Out = ()>
where F:Fn(FIR,FIC)->FO,
	CollectChangeSystem<F>:SystemParamFunction<(FIR,FIC,FO),In = (),Out = ()>
{
	CollectChangeSystem(f)
}

pub fn to_collect_change_system_with_processing<F,FIR,FIC,FO>(f:F)->
	ScheduleConfigs<ScheduleSystem>
// CalculateSystem<F>
where 
	F:Fn(FIR,FIC)->FO+'static,
	CollectChangeSystem<F>:SystemParamFunction<(FIR,FIC,FO),In = (),Out = ()>,
	FIR:'static,FIC:'static,FO:'static,
	// FIR:HMappable<
	// 	Poly<HTypeFnToMapper<ChainFunc<MapFromRef,MapToStat>>>,
	// 	Output : HMappable<
	// 		Poly<MapToProcessingSystemSet>,
	// 		Output :Default+HFoldLeftable<
	// 			Poly<FoldScheduleConfigsAfterSets>, 
	// 			ScheduleConfigs<ScheduleSystem>,
	// 			Output = ScheduleConfigs<ScheduleSystem>>>>,
	FIC:HMappable<
		Poly<MapToChange>,
		Output : HMappable<
			Poly<MapToProcessingSystemSet>,
			Output :Default+HFoldLeftable<
				Poly<FoldScheduleConfigsAfterSets>, 
				ScheduleConfigs<ScheduleSystem>,
				Output = ScheduleConfigs<ScheduleSystem>>>>,
	FO:HMappable<
		Poly<MapToChange>,
		Output : HMappable<
			Poly<MapToProcessingSystemSet>,
			Output :Default+HFoldLeftable<
				Poly<FoldScheduleConfigsBeforeSets>, 
				ScheduleConfigs<ScheduleSystem>,
				Output = ScheduleConfigs<ScheduleSystem>>>>
{
	let r=CollectChangeSystem(f);
	let cfg=r.into_configs()
		.config_processing::<
			// HMapP<FIR,HTypeFnToMapper<ChainFunc<MapFromRef,MapToStat>>>,
			HMapP<FIC,MapToChange>,
			HNil,
			HMapP<FO,MapToChange>
		>()
		;
	// .after_sets(processing_system_sets::<HMapP<FIC,MapToChange>>());
	cfg
}

impl<
	F,
	FIR,FIC,FO,
	FIRS, FICMC, FOC, FOMC,
	FIRSQ, FICMCQ, FOMSQ,
	
	// FIRSQR, FOMSQR,
	// FIR2,
	FO2,
	// M
> SystemParamFunction<
	// (Self,FIR,FO)
	// (FIR,FO)
	// HList!(FIR,FO)
	(FIR,FIC,FO)
	
	// (<<FIRSQR as QueryData>::Item<'static,'static> as HMappable<Poly<MapFromStatRef>>>::Output, FO2)
> 
for CollectChangeSystem<F>
where 
	// Self:GetCalculateSystemMarker<M,Marker = CalculateSystemMarker<(FIR,FO)>>,
	F:Send+Sync+'static,
	for<'a,'w,'s> &'a F:
		Fn(FIR,FIC)->FO+
		Fn( 
			// <<FIRSQ as QueryData>::Item<'w,'s> as HMappable<Poly<MapFromStatRef>>>::Output,
			HMapP<QueryItem<FIRSQ>,MapFromStatRef>,
			// <<FICMCQ as QueryData>::Item<'w,'s> as HMappable<Poly<ChainFunc<MapDerefMut,HTakeChagne>>>>::Output
			// HMapP<HToMut<QueryItem<FICMCQ>>,ChainFunc<MapDerefMut,HTakeChagne>>
			HMapP<QueryItem<FICMCQ>,HTakeChagneG>
		)->FO2,
	
	// F:Fn(FIR)->FO,

	FIR:HMappable<Poly<HTypeFnToMapper<ReverseFunc<MapFromStatRef>>>,Output = FIRS>,
	
	FIRS:HMappable<Poly<MapFromStatRef>,Output = FIR>,

	FIC:HMappable<Poly<HTypeFnToMapper< ChainFunc< MapToChange, MapMut<'static>> >>, Output = FICMC>,

	FICMC:HMappable<Poly<HTakeChagne>,Output = FIC>,

	FO:HMappable<Poly<MapToChange>,Output = FOC>,
	FOC:HMappable<Poly<HTypeFnToMapper<MapMut<'static>>>,Output = FOMC>,
	// for<'a> FOC:HMappable<Poly<HTypeFnToMapper<MapRef<'a>>>>,

	// FIRS:'static,FORC:'static,
	FIRS:HToQuery<Output = FIRSQ>,
	FICMC:HToQuery<Output = FICMCQ>,
	FOMC:HToQuery<Output = FOMSQ>,
	// for<'a> HMapP<FOC,HTypeFnToMapper<MapRef<'a>>>:HToQuery,

	FIRSQ:'static+QueryData,
	FICMCQ:'static+QueryData,
	FOMSQ:'static+QueryData,
	// for<'a> HToQueryType<HMapP<FOC,HTypeFnToMapper<MapRef<'a>>>>:'static+QueryData,
	// for<'a> <HToQueryType<HMapP<FOC,HTypeFnToMapper<MapRef<'a>>>> as QueryData>::ReadOnly:ReadOnlyQueryData,

	for<'w,'s> <FIRSQ as QueryData>::Item<'w,'s>: HMappable<Poly<MapFromStatRef>/*,Output = FIR2*/>,
	// for<'w,'s> <FIRSQR as QueryData>::Item<'w,'s>: HMappable<Poly<MapFromStatRef>,Output = FIR2>,
	// for<'a,'w,'s> QueryItem<'w,'s,FICMCQ>: ToMut<'a,Output = FICMCQIM>,
	// FICMCQIM: HMappable<Poly<ChainFunc<MapDerefMut, HTakeChagne>>>,
	// for<'w,'s,'a> QueryItem<'w,'s,FICMCQ>: ToMut<'a>,
	// for<'w,'s,'a> HToMut<'a,QueryItem<'w,'s,FICMCQ>>:HMappable<Poly<HTakeChagneG>>,
	for<'w,'s> QueryItem<'w,'s,FICMCQ>:HMappable<Poly<HTakeChagneG>>,
	// for<'w,'s> <FICMCQ as QueryData>::Item<'w,'s>: ToMut<'w, Output : HMappable<Poly<ChainFunc<MapDerefMut, HTakeChagne>>>>,//HTakeChagne
	// for<'w,'s> <<FICMCQ as QueryData>::Item<'w,'s> as ToMut<'w>>::Output:HMappable<Poly<ChainFunc<MapDerefMut,HTakeChagne>>>,
	// for<'w, 's> <<FICMCQ as QueryData>::Item<'w, 's> as ToMut<'w>>::Output: HMappable<Poly<ChainFunc<MapDerefMut, HTakeChagne>>>,
	// for<'a,'w,'s> FO2:HZippable< <<HToQueryType<HMapP<FOC,HTypeFnToMapper<MapRef<'a>>>> as QueryData>::ReadOnly as QueryData>::Item<'w,'s>,Zipped : HMappable<Poly<HChangeAdd>> >
	// for<'w,'s,'a> QueryItem<'w,'s,FOMSQ>: ToRef<'a>,//HMappable<Poly<MapDerefMut>>,
	// for<'w,'s,'a> HToRef<'a,QueryItem<'w,'s,FOMSQ>>: HMappable<Poly<MapDeref>>,
	// for<'w,'s,'a> FO2:HZippable< HMapP<HToRef<'a,QueryItem<'w,'s,FOMSQ>>,MapDeref>,Zipped : HMappable<Poly<HChangeAdd>> >,
	
	for<'w,'s> FO2:HZippable< QueryItem<'w,'s,FOMSQ>,Zipped : HMappable<Poly<HChangeAddG>> >,

{
    type In = ();
    type Out = ();
    type Param = Query<'static,'static,(
		HToQueryType<FIRS>,
		HToQueryType<FICMC>,
		HToQueryType<FOMC>
	)>;
	
    fn run(
            &mut self,
            _input:(),
            mut param_value: bevy::ecs::system::SystemParamItem<Self::Param>,
        ) -> () 
	{
		param_value.par_iter_mut().for_each(|(a,b,mut c)|{
			let firs=a;
			let ficmc=b;
			let fir=firs.map(Poly(MapFromStatRef));
			let fic=ficmc.map(Poly(HTakeChagneG));
			fn call_inner<FI,FI2,FO>(f:impl Fn( FI,FI2 )->FO,i:FI,i2:FI2)->FO{
				f(i,i2)
			}
			let f:&F=&self.0;
			// let fo=(f)(fir);
			let fo=call_inner(f,fir,fic);
			fo.zip(c).map(Poly(HChangeAddG));
		});
		
    }
}


/// convert a Fn(HList!(&A,&B,&C))->HList!(D,E,F) into a system with `Query<(&Stat<A>,&Stat<B>,&Stat<C>,&mut Stat<D>,&mut Stat<E>,&mut Stat<F>)>` (like)
/// 
/// with `config_processing<HList!(Stat<A>,Stat<B>,Stat<C>),HNil,HList!(Stat<D>,Stat<E>,Stat<F>)>`
pub fn to_calculate_stat_system_with_processing<F,FIR,FO>(f:F)->
	ScheduleConfigs<ScheduleSystem>
// CalculateSystem<F>
where 
	F:Fn(FIR)->FO+Send+Sync+'static,
	FIR:'static,FO:'static,
	CalculateStatSystem<F>:SystemParamFunction<(FIR,FO),In = (),Out = ()>,
	FIR:HMappable<Poly<HTypeFnToMapper<ChainFunc<MapFromRef,MapToStat>>>,Output : HMappable<Poly<MapToProcessingSystemSet>,Output :Default+HFoldLeftable<Poly<FoldScheduleConfigsAfterSets>, ScheduleConfigs<ScheduleSystem>,Output = ScheduleConfigs<ScheduleSystem>>>>,
	FO:HMappable<Poly<MapToStat>,Output : HMappable<Poly<MapToProcessingSystemSet>,Output :Default+HFoldLeftable<Poly<FoldScheduleConfigsBeforeSets>, ScheduleConfigs<ScheduleSystem>,Output = ScheduleConfigs<ScheduleSystem>>>>
{
	let r=CalculateStatSystem(f);
	let cfg=r.into_configs();
	let cfg=cfg.config_processing::<
		HMapP<FIR,HTypeFnToMapper<ChainFunc<MapFromRef,MapToStat>>>,
		HNil,
		HMapP<FO,MapToStat>
	>();
	cfg
}

// #[derive(Debug,Default,Clone, Copy)]
// pub struct CalculateStatSystemMarker<A>(pub A);
pub type CalculateStatSystemMarker<A>=A;

impl<
	F,
	FIR,FO,
	FIRS, FOS, FOMS,
	FIRSQ, FOMSQ,
	// FIRSQR, FOMSQR,
	// FIR2,
	FO2,
	// M
> SystemParamFunction<
	// (Self,FIR,FO)
	// (FIR,FO)
	// HList!(FIR,FO)
	(FIR,FO)
	
	// (<<FIRSQR as QueryData>::Item<'static,'static> as HMappable<Poly<MapFromStatRef>>>::Output, FO2)
> 
for CalculateStatSystem<F>
where 
	// Self:GetCalculateSystemMarker<M,Marker = CalculateSystemMarker<(FIR,FO)>>,
	F:Send+Sync+'static,
	for<'a,'w,'s> &'a F:
		Fn(FIR)->FO+
		Fn( <<FIRSQ as QueryData>::Item<'w,'s> as HMappable<Poly<MapFromStatRef>>>::Output )->FO2,
	
	// F:Fn(FIR)->FO,

	FIR:HMappable<Poly<HTypeFnToMapper<ReverseFunc<MapFromStatRef>>>,Output = FIRS>,
	FIRS:HMappable<Poly<MapFromStatRef>,Output = FIR>,

	FO:HMappable<Poly<MapToStat>,Output = FOS>,
	FOS:HMappable<Poly<HTypeFnToMapper<MapMut<'static>>>,Output = FOMS>,
	// for<'a> FOC:HMappable<Poly<HTypeFnToMapper<MapRef<'a>>>>,

	// FIRS:'static,FORC:'static,
	FIRS:HToQuery<Output = FIRSQ>,
	FOMS:HToQuery<Output = FOMSQ>,
	// for<'a> HMapP<FOC,HTypeFnToMapper<MapRef<'a>>>:HToQuery,

	FIRSQ:'static+QueryData,
	FOMSQ:'static+QueryData,
	// for<'a> HToQueryType<HMapP<FOC,HTypeFnToMapper<MapRef<'a>>>>:'static+QueryData,
	// for<'a> <HToQueryType<HMapP<FOC,HTypeFnToMapper<MapRef<'a>>>> as QueryData>::ReadOnly:ReadOnlyQueryData,

	for<'w,'s> <FIRSQ as QueryData>::Item<'w,'s>: HMappable<Poly<MapFromStatRef>/*,Output = FIR2*/>,
	// for<'w,'s> <FIRSQR as QueryData>::Item<'w,'s>: HMappable<Poly<MapFromStatRef>,Output = FIR2>,
	
	// for<'a,'w,'s> FO2:HZippable< <<HToQueryType<HMapP<FOC,HTypeFnToMapper<MapRef<'a>>>> as QueryData>::ReadOnly as QueryData>::Item<'w,'s>,Zipped : HMappable<Poly<HChangeAdd>> >
	for<'w,'s> FO2:HZippable< <FOMSQ as QueryData>::Item<'w,'s>,Zipped : HMappable<Poly<HStatSet>> >
{
    type In = ();
    type Out = ();
    type Param = Query<'static,'static,(
		HToQueryType<FIRS>,
		HToQueryType<FOMS>
	)>;
	
    fn run(
            &mut self,
            _input:(),
            mut param_value: bevy::ecs::system::SystemParamItem<Self::Param>,
        ) -> () 
	{
		param_value.par_iter_mut().for_each(|(a,b)|{
			let firs=a;
			let forc=b;
			let fir=firs.map(Poly(MapFromStatRef));
			fn call_inner<FI,FO>(f:impl Fn( FI )->FO,i:FI)->FO{
				f(i)
			}
			let f:&F=&self.0;
			// let fo=(f)(fir);
			let fo=call_inner(f,fir);
			fo.zip(forc).map(Poly(HStatSet));
		});
		
    }
}


#[cfg(test)]
mod test{

	
}


// fn test_2(app:&mut App){
// 	// let dwa=CalculateSystem(calculate_position_state);
// 	let dwa=
// 	CalculateSystemTest::<_>(calculate_position_state,Default::default());
// 	// test_is_system(dwa);
// 	// let awd=IntoSystem::into_system(dwa);
// 	// app.add_systems(schedule_pre_sim(), IntoSystem::into_system(CalculateSystem(calculate_position_state)));
// 	// app.add_systems(schedule_pre_sim(), systems)
// 	let awd=IntoSystem::into_system(dwa);
// }

// fn is_into_system<F,A:SystemInput,B,M>(f:&F)
// 	where F:IntoSystem<A,B,M>
// {}

// fn is_system_param_function<T,M>(v:&T)
// 	where T:SystemParamFunction<M>
// {

// }

pub fn calculate_position_state_plugin<Num:RealField+Copy,const DIM:usize>(app:&mut App){
	app.add_systems(schedule_apply_change(), to_calculate_stat_system_with_processing(calculate_position_state::<Num,DIM>));
}

pub fn calculate_angular_state_plugin<Num,const DIM:usize>(app:&mut App)
where
	Num:RealField+Copy,
	Const<DIM>: DimToSoDim + DimName + DimSquare,
	DefaultAllocator: AllocatorSyncVMSq<ConstDimToSoDimT<DIM>,Num>,
    ConstDimToSoDimT<DIM>:DimSquare,
{
	app.add_systems(schedule_apply_change(), to_calculate_stat_system_with_processing(calculate_angular_state::<Num,DIM>));
}

pub fn calculate_vel_var_plugin<Num:RealField+Copy,const DIM:usize>(app:&mut App){
	app.add_systems(schedule_apply_change(), to_calculate_stat_system_with_processing(calculate_vel_var::<Num,DIM>));
}

pub fn calculate_density_plugin<Num:RealField+Copy>(app:&mut App){
	app.add_systems(schedule_apply_change(), to_calculate_stat_system_with_processing(calculate_density::<Num>));
}

pub fn calculate_rotation_matrix_plugin<Num:RealField+Copy,const DIM:usize>(app:&mut App)
where
	Const<DIM>: DimToSoDim + DimName + DimSquare,
	DefaultAllocator: AllocatorSyncVMSq<ConstDimToSoDimT<DIM>,Num>,
    ConstDimToSoDimT<DIM>:DimSquare,
{
	app.add_systems(schedule_apply_change(), to_calculate_stat_system_with_processing(calculate_rotation_matrix::<Num,DIM>));

}

pub fn collect_position_state_det_pos_momentum_change_vel_plugin<Num:RealField+Copy,const DIM:usize>(app:&mut App){
	// let dwa:<HList!(Vel<Num,DIM>) as HMappable<Poly<HTypeFnToMapper< ChainFunc< MapToChange, ReverseFunc<MapFromMut>> >>> >::Output;
	// let a:bevy::ecs::world::Mut<'static,i32>;
	// let b=MapDerefMut::call(&mut a);
	// a.der
	// let wat:<bevy::ecs::world::Mut<'static,i32> as std::ops::Dere>::Target;
	// let dwa:<frunk::HCons<bevy::ecs::world::Mut<'static, wacky_bag_bevy::stat_component::change::Change<physics_basic::stats::Vel<Num, DIM>>>, HNil> as HMappable<Poly<MapDerefMut>>>::Output;
	app.add_systems(schedule_apply_change(), to_collect_change_system_with_processing(collect_position_state_det_pos_momentum_change_vel::<Num,DIM>));
}

pub fn collect_angular_state_det_agi_agm_change_agv_plugin<Num,const DIM:usize>(app:&mut App)
where
	Num:RealField+Copy,
	Const<DIM>: DimToSoDim + DimName,
	DefaultAllocator: AllocatorSyncVMSq<ConstDimToSoDimT<DIM>,Num>,
    ConstDimToSoDimT<DIM>:DimSquare,
{
	app.add_systems(schedule_apply_change(), to_collect_change_system_with_processing(collect_angular_state_det_agi_agm_change_agv::<Num,DIM>));
}
#[derive(Debug,Clone, Copy)]
pub struct CalculateSystemsPlugins<Num,const DIM:usize>(pub PhantomData<[Num;DIM]>)
where
	Num:RealField+Copy,
	Const<DIM>: DimToSoDim + DimName,
	DefaultAllocator: AllocatorSyncVMSq<ConstDimToSoDimT<DIM>,Num>,
    ConstDimToSoDimT<DIM>:DimSquare;

impl<Num,const DIM:usize> Default for CalculateSystemsPlugins<Num,DIM>
where
	Num:RealField+Copy,
	Const<DIM>: DimToSoDim + DimName,
	DefaultAllocator: AllocatorSyncVMSq<ConstDimToSoDimT<DIM>,Num>,
    ConstDimToSoDimT<DIM>:DimSquare,
{
	fn default() -> Self {
		Self(Default::default())
	}
}

impl<Num,const DIM:usize> PluginGroup for CalculateSystemsPlugins<Num,DIM> 
where
	Num:RealField+Copy,
	Const<DIM>: DimToSoDim + DimName + DimSquare,
	DefaultAllocator: AllocatorSyncVMSq<ConstDimToSoDimT<DIM>,Num>,
    ConstDimToSoDimT<DIM>:DimSquare,
{
	fn build(self) -> PluginGroupBuilder {
		// let dwa:fn(&mut App)=calculate_position_state_plugin::<Num,DIM>;
		let res=
		PluginGroupBuilder::start::<Self>()
			.add(calculate_position_state_plugin::<Num,DIM>)
			.add(calculate_angular_state_plugin::<Num,DIM>)
			.add(calculate_vel_var_plugin::<Num,DIM>)
			.add(calculate_density_plugin::<Num>)
			.add(calculate_rotation_matrix_plugin::<Num,DIM>)
			.add(collect_position_state_det_pos_momentum_change_vel_plugin::<Num,DIM>)
			.add(collect_angular_state_det_agi_agm_change_agv_plugin::<Num,DIM>)
			.add(spawn_stat_apply_change_system_plugin::<Num,DIM>)
		;
		
		res
	}
}

pub fn spawn_stat_apply_change_system_plugin<Num,const DIM:usize>(app:&mut App)
where 
	Num:RealField+Copy,
	Const<DIM>: DimToSoDim + DimName,
	DefaultAllocator: AllocatorSyncVMSq<ConstDimToSoDimT<DIM>,Num>,
    ConstDimToSoDimT<DIM>:DimSquare,
{
	let cfgsh=
	default::<
	HMapP<
		HZip<
		HZip<
			PhyBodyStatisticBundleDetermining<Num,DIM>,
			_>,
			HTypeMapP<PhyBodyStatisticBundleDetermining<Num,DIM>,ChainFunc<MapToDetermining,MapToWith>>
		>,
		MapToPhantom>
	>()
	.map(Poly(StatChangeToApplyChanges));

	let cfgsh=
	cfgsh.zip( 
		default::<HMapP< HMapP<PhyBodyStatisticBundleDetermining<Num,DIM>,MapToDetermining>,MapToProcessingSystemSet>>() )
	.map(
		Poly(FoldScheduleConfigsInSet)
	);

	// cfgsh.zip(default::<HMapP<PhyBodyStatisticBundleDetermining<Num,DIM>,MapToPhantom>>()).map(
	// 	impl_func_clause!(<T>:
	// 		((ScheduleConfigs<ScheduleSystem>,PhantomData<T>))
	// 		|(sys,a)|{

	// 		}
	// 	)
	// );

	let Owned(cfgs)=cfgsh.foldl(Poly(FoldVecPush), Owned(Vec::<ScheduleConfigs<ScheduleSystem>>::new()));
	let cfg= ScheduleConfigs::Configs { configs: cfgs, collective_conditions:default(),metadata:default() };
	
	app.add_systems(schedule_apply_change(), cfg);
}

