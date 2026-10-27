#![deny(unsafe_op_in_unsafe_fn)]

pub mod activities;
pub mod arena;
pub mod batch;
pub mod bond_talk;
pub mod cleaning;
pub mod collector;
pub mod config;
pub mod labels;
pub mod level;
pub mod meld;
pub mod profile;
pub mod random;
pub mod session;
pub mod settings;
pub mod wyvern;

#[cfg(target_os = "switch")]
mod native;
#[cfg(target_os = "switch")]
mod plugin;

#[cfg(target_os = "switch")]
#[skyline::main(name = "feesomniel")]
pub fn skyline_main() {
  plugin::install();
}
