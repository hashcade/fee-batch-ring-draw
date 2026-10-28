//! Collect already-generated exploration rewards through native settlement.

#[cfg(target_os = "switch")]
pub(crate) mod native;
pub mod profile;
