use std::{marker::PhantomData, ops::AddAssign};

use bevy::{app::App, ecs::{bundle::Bundle, component::Component, entity::Entity, message::{Message, MessageReader}, schedule::IntoScheduleConfigs, system::Query}, log::error, reflect::Reflect, utils::default};
use frunk::{Func, Poly, hlist};
use nalgebra::{Const, DefaultAllocator, DimName, OVector, RealField, SVector, Scalar, Vector3, allocator::Allocator};
use num_traits::Zero;
use physics_basic::{rotation::{ConstDimToSoDimT, DimToSoDim, Rotation, RotationMatrix}, stats::{Energy, Mass, Momentum, Pos, TimePass, Vel, mass_vel_2_kinetic, mass_vel_2_momentum}};
use statistic_physics::matters::MattersBasicStat;
use wacky_bag_bevy::{component::CAM, stat_component::{change::Change, stat::Stat}, utils::stat_for_hlist::HChangeAdd};
use wacky_bag::utils::num_extend::NumExtends;
use wacky_bag_hlist::{h_list_helpers::{FoldApply, HMapP, MapToPhantom}, impl_func_closure, output_map::HMappableFrom, reverse_func::ReverseFunc};
use bevy::prelude::ReflectComponent;
use crate::{grid_gas::at_grid_gas::AtGridCellGas, schedule::schedule_sim};

#[derive(Debug,Reflect,Component,Clone, Copy)]
#[reflect(Component)]
pub struct ThrustDef<Num>{
    pub output_speed:Num,
    pub output_speed_sq:Num,
    pub max_power:Num,
}

impl<Num:RealField+Copy> ThrustDef<Num> {
    pub fn new(output_speed: Num, max_power: Num) -> Self {
        Self { output_speed, output_speed_sq:output_speed*output_speed, max_power }
    }
}

/// Control the thrust
/// 
#[derive(Debug, Reflect, Clone, Copy, Component)]
#[reflect(Component)]
pub struct ThrustControl<Num>{
    pub power:Num
}

impl<Num: Zero> Default for ThrustControl<Num> {
    fn default() -> Self {
        Self { power: Num::zero() }
    }
}
#[derive(Debug,Reflect,Message)]
pub struct ThrustControlMsg<Num>{
    pub control:ThrustControl<Num>,
    pub e:Entity,
}

#[derive(Debug, Reflect, Component, Clone, Copy)]
#[reflect(Component)]
pub struct ThrustState<Num>{
    power:Num,
    mass_per_t:Num,
    force:Num,
}

impl<Num: Zero> Default for ThrustState<Num> {
    fn default() -> Self {
        Self { power: Num::zero(), mass_per_t: Num::zero(), force: Num::zero() }
    }
}

pub fn bundle<Num:Send+Sync+'static+RealField+Copy>(def:ThrustDef<Num>)->impl Bundle{
    (
        def,
        ThrustState::<Num>::default(),
        ThrustControl::<Num>::default()
    )
}

pub fn calc_thrust<Num>(def:&ThrustDef<Num>,control:&ThrustControl<Num>)->ThrustState<Num>
where Num:RealField+Copy
{
    
    let mut power=control.power;
    
    if power<Num::zero() {
        power=Num::zero();
    }else if power>def.max_power {
        power=def.max_power;
    }

    ThrustState{
        power:power,
        mass_per_t:power/def.output_speed_sq*Num::p2(),
        force:power/def.output_speed*Num::p2()
    }
}

pub fn thrust_control<Num>(mut mr:MessageReader<ThrustControlMsg<Num>>,mut q:Query<&mut ThrustControl<Num>>)
where Num:Sync+Send+'static+Copy+RealField
{
    mr.read().for_each(|m|{
        let Ok(mut c)=q.get_mut(m.e) else {
            error!("entity {} not exist",m.e);
            return;
        };
        // let Some(t)=c.0.get_mut(m.idx) else {
        //     error!("entity {} dont have thrust id {}",m.e,m.idx);
        //     return;
        // };
        *c=m.control;
    });
}


pub fn thrust_control_apply<Num>(mut q:Query<(&ThrustDef<Num>,&mut ThrustState<Num>,&ThrustControl<Num>)>)
where Num:Sync+Send+'static+Copy+RealField
{
	q.par_iter_mut().for_each(|mut c|{
        
		*c.1=calc_thrust(&c.0, &c.2);
	});
}

pub fn thrust_active<Num,const DIM:usize>(q:Query< 
    (
        (&ThrustDef<Num>,&ThrustState<Num>),  
        &Stat<RotationMatrix<Num,DIM>>, &Change<Momentum<Num,DIM>>, &Stat<Vel<Num,DIM>>, &Stat<TimePass<Num>>, 
        &AtGridCellGas<Num,DIM>
) >)
where Num:Sync+Send+'static+Copy+RealField,
    Const<DIM>:DimToSoDim+DimName,
{
    q.par_iter().for_each(|(t,r,m,v,time,agcg)|{
        
        let onevec=OVector::<Num,Const<DIM>>::from_fn(
            move |v,_|
            if v==0 {Num::one()} else {Num::zero()}
        );
        let dir = r.0.0 * onevec;

        let dt=time.0.0;

        let mut matters_sum:MattersBasicStat::<Num,DIM>=default::<HMapP<MattersBasicStat::<Num,DIM>,MapToPhantom>>().map(
            Poly(impl_func_closure!(<T>{where T:num_traits::Zero}:(PhantomData<T>)->(T)|_|T::zero()))
        );

        m.add_change(Momentum(dir*(t.1.force*dt)));
        let dm=Mass(t.1.mass_per_t*dt);
        let dmv=Vel(v.0.0 - dir*(t.0.output_speed));
        let dmm=mass_vel_2_momentum(hlist![&dm,&dmv]).head;
        let dmk=Energy(mass_vel_2_kinetic(hlist![&dm,&dmv]).head.0);
        (&mut matters_sum).to_mut().zip(hlist![dm,dmm,dmk]).map(Poly(
            impl_func_closure!(<'a,T>{where T:AddAssign}: ((&'a mut T,T)) |(a,b)|*a+=b)
        ));
        matters_sum.zip( agcg.0.to_ref().sculpt().0 ).map(Poly(HChangeAdd));
    });
}

pub fn plugin<Num,const DIM:usize>(app:&mut App)
where Num:Sync+Send+'static+Copy+RealField,
    Const<DIM>:DimToSoDim+DimName,
{
    app.add_systems(schedule_sim(), (
        thrust_control_apply::<Num>,thrust_active::<Num,DIM>
    ).chain());
}

pub mod control_pipe{

    use super::*;
    use bevy::{app::App, ecs::{schedule::IntoScheduleConfigs, system::Query}};
    use wacky_bag_bevy::component::CAM;

    use crate::{schedule::schedule_sim, subsystem::{dyn_object::DynObject::self, pipe::{dyn_key::DynKey, tree_router::{TreeRouter, TreeRouterPropagateLeafToRootSystemSet, TreeRouterPropagateRootToLeafSystemSet}}}};

    pub const THRUST_PIPE_KEY:&str="thrust";
    pub type ThrustPipe<Num>=(ThrustDef<Num>,ThrustState<Num>,ThrustControl<Num>);
    pub fn thrust_pipe_send<Num>(mut q:Query<(&ThrustDef<Num>,&ThrustState<Num>,&ThrustControl<Num>,&CAM<TreeRouter<Vec<Option<DynObject>>>>)>)
    where Num:Sync+Send+'static+Copy+RealField
    {
        q.par_iter_mut().for_each(|(td,ts,tc,r)|{
            let tp:ThrustPipe<Num>=(td.clone(),ts.clone(),tc.clone());
            let mut r=r.lock().unwrap();
            let key=DynKey::Str(THRUST_PIPE_KEY.into());
            let idx=r.key(key);
            let entry=&mut r.container_mut()[idx];
            if let Some(e)=entry {
                if let Some(e)=e.downcast_mut::<ThrustPipe<Num>>() {
                    *e=tp;
                }else {
                    *e=DynObject::Box(Box::new(tp));
                }
            }else {
                *entry=Some(DynObject::Box(Box::new(tp)));
            }
        });
    }

    pub fn thrust_pipe_receive<Num>(mut q:Query<(&mut ThrustControl<Num>,&CAM<TreeRouter<Vec<Option<DynObject>>>>)>)
    where Num:Sync+Send+'static+Copy+RealField
    {
        q.par_iter_mut().for_each(|(mut tc,r)|{
            let mut r=r.lock().unwrap();
            let key=DynKey::Str(THRUST_PIPE_KEY.into());
            let idx=r.key(key);
            r.container_mut()[idx].as_mut().and_then(|v|v.downcast_ref::<ThrustPipe<Num>>()).map(|v|{
                *tc=v.2;
            });
        });
    }

    pub fn plugin<Num:Sync+Send+'static+Copy+RealField>(app:&mut App){
        app.add_systems(schedule_sim(), (
            thrust_pipe_send::<Num>.before(TreeRouterPropagateLeafToRootSystemSet),
            thrust_pipe_receive::<Num>.after(TreeRouterPropagateRootToLeafSystemSet).after(thrust_control_apply::<Num>),
        ));
    }
}