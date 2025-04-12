//!Final components are components that 

use bevy::prelude::Component;

use statistic_physics::{num::Num,vec2_fix::Vec2Fix};
use wacky_bag::structures::state::AState;

use super::{matters::{Energy, Mass, Momentum}, tramsform::Pos};

#[derive(Component,Debug)]
pub struct FinalComponent<T:
Default+std::ops::AddAssign<T>
>(pub AState<T,T>);

pub type MassFinal=FinalComponent<Mass>;
pub type MomentumFinal=FinalComponent<Momentum>;
pub type EnergyFinal=FinalComponent<Energy>;
pub type PosFinal=FinalComponent<Pos>;
/*
pub trait FinalComponent:Component {
    fn apply_delta(&mut self);
}

macro_rules! DeriveFinalComponent {
    ($at:ty) => {
        impl FinalComponent for $at{
            fn apply_delta(&mut self) {
                self.0.apply_delta();
            }
        }
        
    };
}

#[derive(Component,Debug)]
pub struct MassFinal(pub AState<Num,Num>);
DeriveFinalComponent!{MassFinal}

#[derive(Component,Debug)]
pub struct MomentumFinal(pub AState<Vec2Fix,Vec2Fix>);
DeriveFinalComponent!{MomentumFinal}

#[derive(Component,Debug)]
pub struct EnergyFinal(pub AState<Num,Num>);
DeriveFinalComponent!{EnergyFinal}

#[derive(Component,Debug)]
pub struct PosFinal(pub AState<Vec2Fix,Vec2Fix>);
DeriveFinalComponent!{PosFinal}

 */