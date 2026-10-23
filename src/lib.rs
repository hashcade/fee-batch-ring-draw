#![deny(unsafe_op_in_unsafe_fn)]

pub mod batch;
pub mod labels;
pub mod profile;
pub mod random;
pub mod session;

#[cfg(target_os = "switch")]
mod native;
#[cfg(target_os = "switch")]
mod plugin;

#[cfg(target_os = "switch")]
#[skyline::main(name = "feedraw")]
pub fn skyline_main() {
  plugin::install();
}
