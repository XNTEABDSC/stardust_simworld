use bevy_ecs::world::World;

pub mod components;
pub mod systems;
pub mod bundles;
pub mod transform;
pub mod statistic_physics;
pub mod physics;
pub mod grid;
pub mod grid_gas;
pub mod system_sets;

pub fn add(left: u64, right: u64) -> u64 {
    let aew=World::new();

    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
