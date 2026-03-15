

//! 
//! [schedule_spawn] [FixedFirst] Spawn entities
//! [schedule_pre_sim] [FixedPreUpdate] Generate basic datas for later systems
//! [schedule_sim] [FixedUpdate] Calculate. Most systems here should run parallelly.
//! [schedule_apply_change] [FixedPostUpdate] Apply changes
//! [schedule_despawn] [FixedLast] Despawn entities

use bevy::app::{FixedFirst, FixedLast, FixedPostUpdate, FixedPreUpdate, FixedUpdate};

/// Spawn entities
pub fn schedule_spawn()->FixedFirst{FixedFirst}

/// Generate basic datas for later systems
pub fn schedule_pre_sim()->FixedPreUpdate{FixedPreUpdate}

/// calculate. Most systems here should run parallelly.
pub fn schedule_sim()->FixedUpdate{FixedUpdate}

/// Apply changes
pub fn schedule_apply_change()->FixedPostUpdate{FixedPostUpdate}

/// Despawn entities
pub fn schedule_despawn()->FixedLast{FixedLast}
