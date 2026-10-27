//! Presentation-only changes to the native paid Emblem-training child.

pub const CREATE_COMBAT: usize = 0x01CA3000;
pub const DESCRIPTOR_COUNT: usize = 14;
pub const KEPT_STEPS: [usize; 8] = [2, 3, 7, 8, 9, 10, 11, 13];
pub const STEP_TYPES: [i32; DESCRIPTOR_COUNT] =
  [11, 11, 10, 15, 11, 11, 10, 11, 11, 10, 10, 15, 11, 0];
pub const STEP_FUNCTIONS: [usize; DESCRIPTOR_COUNT] = [
  0x01CA38C0, 0x01CA3940, 0, 0, 0x01CA39A0, 0x01CA3A20, 0x01CA3A80, 0x01CA3B00, 0x01CA3B80,
  0x01CA3C00, 0, 0, 0x01CA3DB0, 0,
];

#[derive(Clone, Copy, Debug, Default)]
pub struct Step {
  pub kind: i32,
  pub function: usize,
}

pub fn selection(emblem: bool, special: bool, training_type: i32, bond_exp: i32) -> bool {
  emblem && !special && training_type == 1 && bond_exp > 0
}

pub fn layout(steps: &[Step]) -> Option<[usize; DESCRIPTOR_COUNT]> {
  if steps.len() != DESCRIPTOR_COUNT
    || steps.iter().enumerate().any(|(index, step)| {
      step.kind != STEP_TYPES[index]
        || STEP_FUNCTIONS[index] != 0 && step.function != STEP_FUNCTIONS[index]
    })
  {
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
  use crate::native::{base, field, reference, Method, Object};

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
      if !selection(emblem, special, field(parent, 0x8C), bond_exp)
        || field::<u8>(parent, 0x88) != 1
        || field::<u8>(parent, 0x89) != 0
        || field::<Object>(parent, 0xB0) != calculator
        || field::<Object>(parent, 0xB8) != simulation
      {
        return;
      }
      let child: Object = field(parent, 0x38);
      if child.is_null()
        || field::<Object>(child, 0x30) != parent
        || field::<i32>(child, 0x18) != 0
        || field::<Object>(child, 0x88) != calculator
        || field::<Object>(child, 0x90) != simulation
        || field::<i32>(child, 0x98) != bond_exp
        || field::<u8>(child, 0x9C) != 1
        || field::<u8>(child, 0x9D) != 0
      {
        return;
      }
      let descriptors: Object = field(child, 0x10);
      if descriptors.is_null() || field::<usize>(descriptors, 0x18) != DESCRIPTOR_COUNT {
        return;
      }
      let mut objects = [core::ptr::null_mut(); DESCRIPTOR_COUNT];
      let mut steps = [Step::default(); DESCRIPTOR_COUNT];
      for index in 0..DESCRIPTOR_COUNT {
        let descriptor: Object = field(descriptors, 0x20 + index * 8);
        if descriptor.is_null() {
          return;
        }
        objects[index] = descriptor;
        steps[index].kind = field(descriptor, 0x10);
        if STEP_FUNCTIONS[index] != 0 {
          let function: Object = field(descriptor, 0x18);
          if function.is_null() || field::<Object>(function, 0x20) != child {
            return;
          }
          let Some(offset) = field::<usize>(function, 0x10).checked_sub(base()) else { return };
          steps[index].function = offset;
        }
      }
      let Some(order) = layout(&steps) else { return };
      // This array is allocated per CreateBind. Keep native fades, Grow1/2/3
      // and End; do not edit a shared method, descriptor or original delegate.
      for (index, original) in order.into_iter().enumerate() {
        reference(descriptors, 0x20 + index * 8, objects[original]);
      }
    }
  }
}
