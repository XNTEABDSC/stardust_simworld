use bevy_ecs::schedule::SystemSet;


#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct StatCalcSet;


#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct StatToDeterminingChangeSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ApplyDeterminingChangeSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeterminingStatSpreadSet;