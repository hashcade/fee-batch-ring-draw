//! Presentation-only changes to native ordinary and paid Emblem-training children.

pub const CREATE_COMBAT: usize = 0x01CA3000;
pub const DESCRIPTOR_COUNT: usize = 14;
pub const KEPT_STEPS: [usize; 8] = [2, 3, 7, 8, 9, 10, 11, 13];

pub fn selection(emblem: bool, special: bool, training_type: i32, bond_exp: i32) -> bool {
  emblem && !special && training_type == 1 && bond_exp > 0
}

pub fn quick_training(emblem: bool, special: bool, training_type: i32, bond_exp: i32) -> bool {
  selection(emblem, special, training_type, bond_exp)
    || !emblem && !special && training_type == 0 && bond_exp == 0
}

pub fn layout(count: usize) -> Option<[usize; DESCRIPTOR_COUNT]> {
  // These native presentation indices must exist; additional steps stay intact.
  if count < DESCRIPTOR_COUNT {
    return None;
  }
  let mut result = [13; DESCRIPTOR_COUNT];
  result[..KEPT_STEPS.len()].copy_from_slice(&KEPT_STEPS);
  Some(result)
}

pub const TEXT_SIGNATURES: &[(usize, [u32; 4])] = &[
  (0x01CA3000, [0xD101C3FF, 0xA9017BFD, 0x910043FD, 0xA9026FFC]),
  (0x01CA36E0, [0xA9BA7BFD, 0xF9000BFB, 0x910003FD, 0xA90267FA]),
  (0x01CA38C0, [0xA9BD7BFD, 0xF9000BF5, 0x910003FD, 0xA9024FF4]),
  (0x01CA3940, [0xA9BE7BFD, 0xA9014FF4, 0x910003FD, 0xD0023393]),
  (0x01CA39A0, [0xA9BD7BFD, 0xF9000BF5, 0x910003FD, 0xA9024FF4]),
  (0x01CA3A20, [0xA9BE7BFD, 0xA9014FF4, 0x910003FD, 0xD0023393]),
  (0x01CA3A80, [0xA9BD7BFD, 0xF9000BF5, 0x910003FD, 0xA9024FF4]),
  (0x01CA3B00, [0xA9BD7BFD, 0xF9000BF5, 0x910003FD, 0xA9024FF4]),
  (0x01CA3B80, [0xA9BD7BFD, 0xF9000BF5, 0x910003FD, 0xA9024FF4]),
  (0x01CA3C00, [0xA9BD7BFD, 0xF9000BF5, 0x910003FD, 0xA9024FF4]),
  (0x01CA3DB0, [0xA9BD7BFD, 0xF9000BF5, 0x910003FD, 0xA9024FF4]),
  (0x01CA51F0, [0x39422000, 0xD65F03C0, 0xD503201F, 0xD503201F]),
  (0x01CA5210, [0x39422400, 0xD65F03C0, 0xD503201F, 0xD503201F]),
  (0x01CA5230, [0xB9408C00, 0xD65F03C0, 0xD503201F, 0xD503201F]),
  (0x01CA9C10, [0xA9BC7BFD, 0xF9000BF7, 0x910003FD, 0xA90257F6]),
  (0x01CA9E80, [0xD10243FF, 0xA9037BFD, 0x9100C3FD, 0xA9046FFC]),
  (0x01CAA3A0, [0xA9BE7BFD, 0xA9014FF4, 0x910003FD, 0xF0023354]),
  (0x01CAA4B0, [0xD101C3FF, 0xA9037BFD, 0x9100C3FD, 0xA9045FF8]),
  (0x01CA37CC, [0x12000308, 0x39027268, 0x120002E8, 0xAA1303F7]),
  (0x01CA33B4, [0xAA1503E0, 0xAA1603E1, 0xF8048C16, 0x979FBAFC]),
  (0x01CA347C, [0xD0022F73, 0xF9434E73, 0xAA1503E0, 0xF8058C16]),
  (0x01BAC5A8, [0xF85F83A8, 0xAA1403E0, 0xAA1503E1, 0xB85EC3A3]),
  (0x01BAC6A0, [0xF85F83A5, 0xAA1403E0, 0xAA1503E1, 0xAA1703E2]),
];

#[cfg(target_os = "switch")]
pub(crate) mod native {
  use super::*;
  use crate::native::{field, reference, Method, Object};

  pub(crate) fn install() {
    skyline::install_hooks!(arena_create);
  }

  #[skyline::hook(offset = CREATE_COMBAT)]
  fn arena_create(
    parent: Object,
    calculator: Object,
    simulation: Object,
    emblem: bool,
    special: bool,
    bond_exp: i32,
    method: Method,
  ) {
    call_original!(parent, calculator, simulation, emblem, special, bond_exp, method);
    if parent.is_null() || calculator.is_null() || simulation.is_null() {
      return;
    }
    unsafe {
      if !quick_training(emblem, special, field(parent, 0x8C), bond_exp) {
        return;
      }
      let child: Object = field(parent, 0x38);
      if child.is_null() {
        return;
      }
      let descriptors: Object = field(child, 0x10);
      if descriptors.is_null() {
        return;
      }
      let Some(order) = layout(field(descriptors, 0x18)) else { return };
      let mut objects = [core::ptr::null_mut(); DESCRIPTOR_COUNT];
      for index in 0..DESCRIPTOR_COUNT {
        let descriptor: Object = field(descriptors, 0x20 + index * 8);
        if descriptor.is_null() {
          return;
        }
        objects[index] = descriptor;
      }
      // This array is allocated per CreateBind. Keep native fades, Grow1/2/3
      // and End; do not edit a shared method, descriptor or original delegate.
      for (index, original) in order.into_iter().enumerate() {
        reference(descriptors, 0x20 + index * 8, objects[original]);
      }
    }
  }
}
