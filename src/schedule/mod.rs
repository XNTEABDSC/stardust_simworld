

//! 
//! [schedule_sim] [FixedFirst] Calculate. Most systems here should run parallelly.
//! [schedule_apply_change] [FixedPreUpdate] Apply changes
//! [schedule_despawn] [FixedUpdate] Despawn entities
//! [schedule_spawn] [FixedPostUpdate] Spawn entities
//! [schedule_pre_sim] [FixedLast] Generate basic datas for later systems
//! 
//! Render

use bevy::app::{FixedFirst, FixedPreUpdate, FixedUpdate, FixedPostUpdate, FixedLast};

pub type FixedSim=FixedFirst;

/// calculate. Most systems here should run parallelly.
pub fn schedule_sim()->FixedFirst{FixedFirst}

/// Apply changes
pub fn schedule_apply_change()->FixedPreUpdate{FixedPreUpdate}

/// Despawn entities
pub fn schedule_despawn()->FixedUpdate{FixedUpdate}

/// Spawn entities
pub fn schedule_spawn()->FixedPostUpdate{FixedPostUpdate}

/// Generate basic datas for later systems
pub fn schedule_pre_sim()->FixedLast{FixedLast}

// /// Spawn entities
// pub fn schedule_spawn()->FixedFirst{FixedFirst}

// /// Generate basic datas for later systems
// pub fn schedule_pre_sim()->FixedPreUpdate{FixedPreUpdate}

// /// calculate. Most systems here should run parallelly.
// pub fn schedule_sim()->FixedUpdate{FixedUpdate}

// /// Apply changes
// pub fn schedule_apply_change()->FixedPostUpdate{FixedPostUpdate}

// /// Despawn entities
// pub fn schedule_despawn()->FixedLast{FixedLast}
