use bevy_ecs::{component::Component, query::With, system::Query};

use crate::components::{change::ChangeComponent, determining_components::DeterminingComponent, finals::FinalComponent};

/// a system that run apply_delta for TFinalComponent
/// put `final_apply_delta::<YourType>` into system
pub fn final_apply_delta_<
T:Default+std::ops::AddAssign<T>+ std::marker::Send+ std::marker::Sync+'static
>(mut query:Query<&mut FinalComponent<T>>){
    for mut i in &mut query{
        //<TFinalComponent as FinalComponent>::apply_delta(&mut i);
        i.0.apply_delta();
    }
}

pub fn determining_components_apply_changes<T>(mut query:Query<(&mut T,&mut ChangeComponent<T>),With<DeterminingComponent<T>>>)
    where T:Component+std::ops::AddAssign<T>+Default
{
    for (mut value,mut delta) in &mut query {
        *value+=delta.get_and_reset();
    }
}