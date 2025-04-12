use bevy_ecs::{component::Component, query::With, system::Query};

use crate::components::{change::ChangeComponent, determining_components::DeterminingComponent};


pub fn determining_components_apply_changes<T>(mut query:Query<(&mut T,&mut ChangeComponent<T>),With<DeterminingComponent<T>>>)
    where T:Component+std::ops::AddAssign<T>+Default
{
    for (mut value,mut delta) in &mut query {
        *value+=delta.get_and_reset();
    }
}
