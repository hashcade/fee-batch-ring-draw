//! Batch melding retained from fee-batch-ring-meld v0.2.0 (d92aed4).

pub mod batch;
pub mod confirmation;
pub mod profile;
pub mod session;

#[cfg(target_os = "switch")]
pub(crate) mod native;
#[cfg(target_os = "switch")]
pub(crate) mod plugin;
