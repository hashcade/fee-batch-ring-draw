use super::{border_offset, sss_score, CALC_RANK, FINISH_RAIL, GET_RETIRE, MAIN, SET_SCORE};
use crate::native::{base, field, Method, Object};
use core::ptr;

const NONE: Method = ptr::null();

pub(crate) fn install() {
  skyline::install_hooks!(wyvern_main);
}

unsafe fn retiring() -> bool {
  let get: unsafe extern "C" fn(Method) -> bool =
    unsafe { core::mem::transmute(base() + GET_RETIRE) };
  unsafe { get(NONE) }
}

unsafe fn plan(sequence: Object) -> Option<(Object, i32)> {
  if sequence.is_null()
    || unsafe { field::<u8>(sequence, 0xA0) } != 0
    || unsafe { field::<u8>(sequence, 0x114) } != 0
  {
    return None;
  }
  let camera: Object = unsafe { field(sequence, 0xA8) };
  let config: Object = unsafe { field(sequence, 0xE0) };
  if camera.is_null()
    || config.is_null()
    || unsafe { field::<u8>(camera, 0xA2) } != 1
    || [0x1B9, 0x1BB, 0x1F8, 0x1F9]
      .iter()
      .any(|&offset| unsafe { field::<u8>(camera, offset) != 0 })
  {
    return None;
  }
  let level: i32 = unsafe { field(sequence, 0xB8) };
  let borders: Object = unsafe { field(config, border_offset(level)?) };
  if borders.is_null() {
    return None;
  }
  let count: usize = unsafe { field(borders, 0x18) };
  if !(2..=9).contains(&count) {
    return None;
  }
  let scores =
    unsafe { core::slice::from_raw_parts(borders.cast::<u8>().add(0x20).cast::<i32>(), count) };
  let score = sss_score(scores)?;
  let rank: unsafe extern "C" fn(Object, i32, i32, Method) -> i32 =
    unsafe { core::mem::transmute(base() + CALC_RANK) };
  // Ask the original rank calculator to confirm both sides of the boundary.
  if unsafe { rank(sequence, level, score, NONE) } != 1
    || unsafe { rank(sequence, level, score - 1, NONE) } == 1
    || unsafe { retiring() }
  {
    return None;
  }
  Some((camera, score))
}

#[skyline::hook(offset = MAIN)]
fn wyvern_main(sequence: Object, method: Method) {
  let completion = unsafe { plan(sequence) };
  if let Some((camera, _)) = completion {
    // FinishRail only signals the same flag tested by the original Main loop.
    // Main still performs its normal tick and ProcInst.End; FinishMain, the
    // result sequence, prize eligibility, and cleanup are not bypassed.
    let finish: unsafe extern "C" fn(Object, Method) =
      unsafe { core::mem::transmute(base() + FINISH_RAIL) };
    unsafe { finish(camera, NONE) };
  }
  call_original!(sequence, method);
  if let Some((_, score)) = completion {
    // End advances the descriptor only. Write after the final tick so pending
    // target effects cannot raise this round above the selected SSS threshold.
    // CreateResultUI later owns the native rank and high-score persistence.
    if !unsafe { retiring() } {
      let set: unsafe extern "C" fn(i32, Method) =
        unsafe { core::mem::transmute(base() + SET_SCORE) };
      unsafe { set(score, NONE) };
    }
  }
}
