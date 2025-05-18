use bevy_ecs::{component::Component, query::With, system::Query};

use crate::components::{change_component::ChangeComponent, determining_components::DeterminingComponent};


pub fn determining_components_apply_changes<T>(mut query:Query<(&mut T,&mut ChangeComponent<T>),With<DeterminingComponent<T>>>)
    where T:Component+std::ops::AddAssign<T>+Default
{
    (&mut query).par_iter_mut().for_each(|(mut value,mut delta)|{
        *value+=delta.get_and_reset();
    });
}
