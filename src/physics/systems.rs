use std::{marker::PhantomData};

use bevy::{app::{App, PluginGroup, PluginGroupBuilder}, ecs::{query::{QueryData, ReadOnlyQueryData}, schedule::{IntoScheduleConfigs, ScheduleConfigs}, system::{IntoSystem, Query, ScheduleSystem, SystemInput}, world::Mut}, prelude::SystemParamFunction};
use frunk::{Func, HNil, Poly,hlist , hlist::{HFoldLeftable, HMappable, HZippable}};
use nalgebra::{Const, DefaultAllocator, DimMin, DimName, RealField, allocator::Allocator};
use physics_basic::{body::{calculate_angular_state, calculate_position_state}, rotation::{DimNameToSoDimName, DimNameToSoDimNameType}};
use statistic_physics::formulas::{calculate_density, calculate_vel_var};
use wacky_bag::utils::{default_of::default, h_list_helpers::{HMapP, HTypeFnToMapper, HZip, MapFromRef, MapMut, MapRef, MapToPhantom}, type_fn::{ChainFunc, ReverseFunc}};
use wacky_bag_bevy::{stat_component::{determining_apply_changes::{MapToDeterminingApplyChanges2Plugin, MapToDeterminingApplyChangesPlugin, determining_apply_changes, determining_apply_changes_2}, stat::Stat}, system::{multi_sets::{FoldScheduleConfigsAfterSets, FoldScheduleConfigsBeforeSets}, processing_system::{MapToProcessingSystemSet, ScheduleConfigsProcessing}}, utils::{fold_plugin_group_add::FoldPluginGroupBuilderAdd, h_list_query::{HToQuery, HToQueryType}, plugin_add_systems::plugin_add_systems, stat_for_hlist::{HChangeAdd, HStatSet, MapFromStatRef, MapToChange, MapToStat}}};

use physics_basic::stats::*;

use crate::{physics::bundle::PhyBodyStatisticBundleDetermining, schedule::{schedule_apply_change, schedule_pre_sim}};

/// use [to_calculate_system] for system instead for type system to find marker
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


/// use [to_calculate_system] for system instead for type system to find marker
#[derive(Debug,Default,Clone, Copy)]
pub struct CalculateStatSystem<F>(pub F);

/// convert a Fn(HList!(&A,&B,&C))->HList!(D,E,F) into a system with Query<(&Stat<A>,&Stat<B>,&Stat<C>,&mut Stat<D>,&mut Stat<E>,&mut Stat<F>)>
pub fn to_calculate_stat_system<F,FIR,FO>(f:F)->
impl SystemParamFunction<CalculateStatSystemMarker<(FIR,FO)>,In = (),Out = ()>
// CalculateSystem<F>
	where F:Fn(FIR)->FO+Send+Sync+'static,
	CalculateStatSystem<F>:SystemParamFunction<CalculateStatSystemMarker<(FIR,FO)>,In = (),Out = ()>
{
	CalculateStatSystem(f)
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
	CalculateStatSystem<F>:SystemParamFunction<CalculateStatSystemMarker<(FIR,FO)>,In = (),Out = ()>,
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

#[derive(Debug,Default,Clone, Copy)]
pub struct CalculateStatSystemMarker<A>(pub A);

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
	CalculateStatSystemMarker<(FIR,FO)>
	
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
	use bevy::ecs::system::SystemParamFunction;
	use frunk::{HCons, ToRef};
	use physics_basic::stats::*;

	use crate::physics;

	use super::*;
	
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
	app.add_systems(schedule_pre_sim(), to_calculate_stat_system_with_processing(calculate_position_state::<Num,DIM>));
}

pub fn calculate_angular_state_plugin<Num,const DIM:usize>(app:&mut App)
where
	Num:RealField+Copy,
	Const<DIM>: DimNameToSoDimName + DimName,
	DefaultAllocator: Allocator<DimNameToSoDimNameType<DIM>, DimNameToSoDimNameType<DIM>,Buffer<Num>:Sync+Send>+Allocator<DimNameToSoDimNameType<DIM>,Buffer<Num>:Sync+Send>,
    DimNameToSoDimNameType<DIM>:
        DimMin<DimNameToSoDimNameType<DIM>, Output = DimNameToSoDimNameType<DIM>>,
{
	app.add_systems(schedule_pre_sim(), to_calculate_stat_system_with_processing(calculate_angular_state::<Num,DIM>));
}

pub fn calculate_vel_var_plugin<Num:RealField+Copy,const DIM:usize>(app:&mut App){
	app.add_systems(schedule_pre_sim(), to_calculate_stat_system_with_processing(calculate_vel_var::<Num,DIM>));
}

pub fn calculate_density_plugin<Num:RealField+Copy>(app:&mut App){
	app.add_systems(schedule_pre_sim(), to_calculate_stat_system_with_processing(calculate_density::<Num>));

}

pub struct CalculateSystemsPlugins<Num,const DIM:usize>(pub PhantomData<[Num;DIM]>)
where
	Num:RealField+Copy,
	Const<DIM>: DimNameToSoDimName + DimName,
	DefaultAllocator: Allocator<DimNameToSoDimNameType<DIM>, DimNameToSoDimNameType<DIM>,Buffer<Num>:Sync+Send>+Allocator<DimNameToSoDimNameType<DIM>,Buffer<Num>:Sync+Send>,
    DimNameToSoDimNameType<DIM>:
        DimMin<DimNameToSoDimNameType<DIM>, Output = DimNameToSoDimNameType<DIM>>,;

impl<Num,const DIM:usize> PluginGroup for CalculateSystemsPlugins<Num,DIM> 
where
	Num:RealField+Copy,
	Const<DIM>: DimNameToSoDimName + DimName + DimMin<Const<DIM>, Output = Const<DIM>>,
	DefaultAllocator: Allocator<DimNameToSoDimNameType<DIM>, DimNameToSoDimNameType<DIM>,Buffer<Num>:Sync+Send>+Allocator<DimNameToSoDimNameType<DIM>,Buffer<Num>:Sync+Send>,
    DimNameToSoDimNameType<DIM>:
        DimMin<DimNameToSoDimNameType<DIM>, Output = DimNameToSoDimNameType<DIM>>,

{
	fn build(self) -> PluginGroupBuilder {
		// let dwa:fn(&mut App)=calculate_position_state_plugin::<Num,DIM>;
		let res=PluginGroupBuilder::start::<Self>()
			.add(calculate_position_state_plugin::<Num,DIM>)
			.add(calculate_angular_state_plugin::<Num,DIM>)
			.add(calculate_vel_var_plugin::<Num,DIM>)
			.add(calculate_density_plugin::<Num>)
		;
		let fns=
			default::<HMapP<HZip<PhyBodyStatisticBundleDetermining<Num,DIM>,_>,MapToPhantom>>()
			.map(Poly(MapToDeterminingApplyChanges2Plugin));
		let res=fns.foldl(Poly(FoldPluginGroupBuilderAdd), res);
		res
	}
}

