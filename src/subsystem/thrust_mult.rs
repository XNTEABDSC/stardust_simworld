use std::{marker::PhantomData, ops::AddAssign};

use bevy::{ecs::{component::Component, entity::Entity, message::{Message, MessageReader}, system::Query}, log::error, reflect::Reflect, utils::default};
use frunk::{Func, Poly, hlist};
use nalgebra::{Const, DefaultAllocator, DimName, OVector, RealField, SVector, Scalar, Vector3, allocator::Allocator};
use num_traits::Zero;
use physics_basic::{rotation::{ConstDimToSoDimT, DimToSoDim, Rotation, RotationMatrix}, stats::{Energy, Mass, Momentum, Pos, TimePass, Vel, mass_vel_2_kinetic, mass_vel_2_momentum}};
use statistic_physics::matters::MattersBasicStat;
use wacky_bag_bevy::{component::vec_component::VecComponent, stat_component::{change::Change, stat::Stat}, utils::stat_for_hlist::{HChangeAdd, MapToChange}};
use wacky_bag::utils::num_extend::NumExtends;
use wacky_bag_hlist::{h_list_helpers::{FoldApply, HMapP, MapToPhantom}, impl_func_closure, output_map::HMappableFrom, reverse_func::ReverseFunc};

use crate::grid_gas::at_grid_gas::AtGridCellGas;

#[derive(Debug,Reflect)]
pub struct ThrustDef<Num>{
    pub output_speed:Num,
    pub output_speed_sq:Num,
    pub max_power:Num,
}

#[derive(Debug,Reflect,Clone, Copy)]
pub struct ThrustControl<Num>{
    pub power:Num
}
#[derive(Debug,Reflect,Message)]
pub struct ThrustControlMsg<Num>{
    pub control:ThrustControl<Num>,
    pub e:Entity,
    pub idx:usize,
}

#[derive(Debug,Reflect)]
pub struct ThrustState<Num>{
    pub power:Num,
    pub mass_per_t:Num,
    pub force:Num,
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

pub fn thrust_control<Num>(mut mr:MessageReader<ThrustControlMsg<Num>>,mut q:Query<(&mut VecComponent<(ThrustDef<Num>,ThrustState<Num>)>,)>)
where Num:Sync+Send+'static+Copy+RealField
{
    mr.read().for_each(|m|{
        let Ok(mut c)=q.get_mut(m.e) else {
            error!("entity {} not exist",m.e);
            return;
        };
        let Some(t)=c.0.get_mut(m.idx) else {
            error!("entity {} dont have thrust id {}",m.e,m.idx);
            return;
        };
        t.1=calc_thrust(&t.0, &m.control);
    });
}

pub fn thrust_active<Num,const DIM:usize>(q:Query< 
    (
        &VecComponent<(ThrustDef<Num>,ThrustState<Num>)>,  
        &Stat<RotationMatrix<Num,DIM>>, &Change<Momentum<Num,DIM>>, &Stat<Vel<Num,DIM>>, &Stat<TimePass<Num>>, 
        &AtGridCellGas<Num,DIM>
) >)
where Num:Sync+Send+'static+Copy+RealField,
    Const<DIM>:DimToSoDim+DimName,
{
    q.par_iter().for_each(|(ts,r,m,v,time,agcg)|{
        
        let onevec=OVector::<Num,Const<DIM>>::from_fn(
            move |v,_|
            if v==0 {Num::one()} else {Num::zero()}
        );
        let dir = r.0.0 * onevec;

        let dt=time.0.0;

        let mut matters_sum:MattersBasicStat::<Num,DIM>=default::<HMapP<MattersBasicStat::<Num,DIM>,MapToPhantom>>().map(
            Poly(impl_func_closure!(<T>{where T:num_traits::Zero}:(PhantomData<T>)->(T)|_|T::zero()))
        );

        for t in &ts.0 {
            m.add_change(Momentum(dir*(t.1.force*dt)));
            let dm=Mass(t.1.mass_per_t*dt);
            let dmv=Vel(v.0.0 - dir*(t.0.output_speed));
            let dmm=mass_vel_2_momentum(hlist![&dm,&dmv]).head;
            let dmk=Energy(mass_vel_2_kinetic(hlist![&dm,&dmv]).head.0);
            (&mut matters_sum).to_mut().zip(hlist![dm,dmm,dmk]).map(Poly(
                impl_func_closure!(<'a,T>{where T:AddAssign}: ((&'a mut T,T)) |(a,b)|*a+=b)
            ));
        }
        matters_sum.zip( agcg.0.to_ref().sculpt().0 ).map(Poly(HChangeAdd));
    });
}