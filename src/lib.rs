#![deny(unsafe_op_in_unsafe_fn)]

pub mod activities;
pub mod batch;
pub mod cleaning;
pub mod collector;
pub mod config;
pub mod cooking;
pub mod facilities;
pub mod fishing;
pub mod labels;
pub mod meld;
pub mod preferences;
pub mod profile;
pub mod random;
pub mod session;
pub mod settings;
pub mod shops;
pub mod supports;
pub mod wyvern;

#[cfg(target_os = "switch")]
mod menu_item;
#[cfg(target_os = "switch")]
mod native;
#[cfg(target_os = "switch")]
mod plugin;
#[cfg(target_os = "switch")]
mod roster;

#[cfg(target_os = "switch")]
#[skyline::main(name = "feesomniel")]
pub fn skyline_main() {
  plugin::install();
}
