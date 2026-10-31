//! Settle ready ordinary supports with native progression and achievement APIs.

pub const RELATION: usize = 0x01C5A040;
pub const CAN_ADVANCE: usize = 0x01C5A930;
pub const LEVEL_UP: usize = 0x01C5ABF0;
pub const UPDATE_ACHIEVE: usize = 0x02098500;
pub const HUB_INIT: usize = 0x023EBCA0;
pub const HUB_TICK: usize = 0x023F26D0;
pub const HUB_IS_MAIN: usize = 0x023E6BD0;
pub const TEXT_SIGNATURES: &[(usize, [u32; 4])] = &[
  (0x01C5A040, [0xA9BD7BFD, 0xF9000BF5, 0x910003FD, 0xA9024FF4]),
  (0x01C5A930, [0xB40000A0, 0xB4000081, 0xF9402000, 0xF9402021]),
  (0x023EBCA0, [0xA9BC7BFD, 0xF9000BF7, 0x910003FD, 0xA90257F6]),
  (0x023F26D0, [0xFC1B0FE8, 0xA9017BFD, 0x910043FD, 0xA9025FF8]),
  (0x023E6BD0, [0xA9BE7BFD, 0xF9000BF3, 0x910003FD, 0xAA1F03E1]),
  (0x01C5A950, [0xA9BD7BFD, 0xF9000BF5, 0x910003FD, 0xA9024FF4]),
  (0x01C5A960, [0xF00235D5, 0x39499EA8, 0xAA0103F3, 0xAA0003F4]),
  (0x01C5A970, [0x37000128, 0xB00231A0, 0xF945F400, 0x97A0DDB9]),
  (0x01C5A980, [0xB00231A0, 0xF945F800, 0x97A0DDB6, 0x52800028]),
  (0x01C5A990, [0x39099EA8, 0xF9000FBF, 0xB4000394, 0xB4000373]),
  (0x01C5A9A0, [0xB00231A8, 0xF945F908, 0xB00231A9, 0xF945F529]),
  (0x01C5A9B0, [0x910063A3, 0xB9401681, 0xB9401662, 0xF9400108]),
  (0x01C5A9C0, [0xF9405D08, 0xF9400500, 0xF9400124, 0x94173C21]),
  (0x01C5A9D0, [0xF9400FA8, 0xB40001A8, 0x36000180, 0xB9401902]),
  (0x01C5A9E0, [0x7100085F, 0x540000CC, 0x39407109, 0x11000442]),
  (0x01C5A9F0, [0xB9001902, 0x11000529, 0x39007109, 0xAA1403E0]),
  (0x01C5AA00, [0xAA1303E1, 0x94000013, 0xA9424FF4, 0xF9400BF5]),
  (0x01C5AA10, [0xA8C37BFD, 0xD65F03C0, 0xD503201F, 0xD503201F]),
  (0x01C5ABF0, [0xB40000A0, 0xB4000081, 0xF9402000, 0xF9402021]),
  (0x01C5AC00, [0x17FFFF54, 0xD65F03C0, 0xD503201F, 0xD503201F]),
  (0x02098500, [0xA9BD7BFD, 0xA90157F6, 0x910003FD, 0xA9024FF4]),
  (0x02098510, [0xD00213F6, 0x39788AC8, 0x2A0303F5, 0xAA0203F3]),
  (0x02098520, [0xAA0103F4, 0x370000C8, 0xF0020F60, 0xF9456800]),
  (0x02098530, [0x978FE6CC, 0x52800028, 0x39388AC8, 0xF9402280]),
  (0x02098540, [0xAA1F03E1, 0x97FA46DB, 0x370000A0, 0xF9402260]),
  (0x02098550, [0xAA1F03E1, 0x97FA46D7, 0x36000200, 0x71000EBF]),
  (0x02098560, [0x54000240, 0x71000ABF, 0x54000341, 0xF0020F68]),
  (0x02098570, [0xF9456908, 0xF9400100, 0x3944CC08, 0x36080088]),
  (0x02098580, [0xB940E008, 0x35000048, 0x978FE6FE, 0xAA1F03E0]),
  (0x02098590, [0x941CC1C8, 0x1400000F, 0xA9424FF4, 0xA94157F6]),
  (0x020985A0, [0xA8C37BFD, 0xD65F03C0, 0xF0020F68, 0xF9456908]),
  (0x020985B0, [0xF9400100, 0x3944CC08, 0x36080088, 0xB940E008]),
  (0x020985C0, [0x35000048, 0x978FE6EF, 0xAA1F03E0, 0x941CC301]),
  (0x020985D0, [0xF9402280, 0xAA1F03E1, 0x97FA46B6, 0xF9402275]),
  (0x020985E0, [0x360000E0, 0xF0020F68, 0xF9456908, 0xF9400100]),
  (0x020985F0, [0x3944CC08, 0x37080188, 0x1400000E, 0xAA1503E0]),
  (0x02098600, [0xAA1F03E1, 0x97FA46AB, 0x360001A0, 0xF0020F68]),
  (0x02098610, [0xF9456908, 0xF9402295, 0xF9400100, 0x3944CC08]),
  (0x02098620, [0x36080088, 0xB940E008, 0x35000048, 0x978FE6D5]),
  (0x02098630, [0xAA1503E0, 0xAA1F03E1, 0x941CC0B6, 0xAA1403E0]),
  (0x02098640, [0xAA1F03E1, 0x97E6756B, 0xAA1F03E1, 0x94075AAD]),
  (0x02098650, [0xAA1303E0, 0xAA1F03E1, 0x97E67566, 0xA9424FF4]),
  (0x02098660, [0xAA1F03E1, 0xA94157F6, 0xA8C37BFD, 0x14075AA5]),
];

#[cfg(target_os = "switch")]
pub(crate) mod native {
  use super::*;
  use crate::{
    config,
    native::{base, field, list_count, object_at, Method, Object},
  };
  use core::{cell::Cell, ptr};
  use std::collections::BTreeSet;

  const NONE: Method = ptr::null();
  std::thread_local! { static ENTERED: Cell<usize> = const { Cell::new(0) }; }

  macro_rules! native {
    ($address:expr, $signature:ty) => {
      core::mem::transmute::<usize, $signature>(base() + $address)
    };
  }

  pub(crate) fn install() {
    skyline::install_hooks!(hub_init, hub_tick);
  }

  #[skyline::hook(offset = HUB_INIT)]
  fn hub_init(hub: Object, method: Method) {
    ENTERED.with(|entered| entered.set(0));
    call_original!(hub, method);
    crate::fishing::native::ensure_allowance();
  }

  #[skyline::hook(offset = HUB_TICK)]
  fn hub_tick(hub: Object, method: Method) {
    call_original!(hub, method);
    if hub.is_null()
      || ENTERED.with(|entered| entered.get() == hub as usize)
      || !config::auto_support()
    {
      return;
    }
    unsafe {
      if !in_somniel()
        || !field::<Object>(hub, 0x38).is_null()
        || !native!(HUB_IS_MAIN, unsafe extern "C" fn(Object, Method) -> bool)(hub, NONE)
      {
        return;
      }
      ENTERED.with(|entered| entered.set(hub as usize));
      settle_ready();
    }
  }

  unsafe fn in_somniel() -> bool {
    unsafe {
      let user = crate::native::user_data();
      !user.is_null() && field::<i32>(user, 0x28) == 4
    }
  }

  pub(crate) unsafe fn on_hub_menu(parent: Object) {
    if parent.is_null() || !config::auto_support() || !unsafe { in_somniel() } {
      return;
    }
    unsafe {
      let menu: Object = field(parent, 0x38);
      if !menu.is_null() && field::<Object>(menu, 0x38).is_null() {
        settle_ready();
      }
    }
  }

  unsafe fn settle_ready() {
    unsafe {
      let units = native!(crate::cooking::UNIT_LIST, unsafe extern "C" fn(Method) -> Object)(NONE);
      let Some(count) = list_count(units) else { return };
      let mut seen = BTreeSet::new();
      for i in 0..count {
        let Some(a) = object_at(units, i).filter(|unit| !unit.is_null()) else { continue };
        for j in i + 1..count {
          let Some(b) = object_at(units, j).filter(|unit| !unit.is_null()) else { continue };
          let pair = (a as usize, b as usize).min((b as usize, a as usize));
          if a == b || !seen.insert(pair) {
            continue;
          }
          let relation =
            native!(RELATION, unsafe extern "C" fn(Object, Object, Method) -> Object)(a, b, NONE);
          if relation.is_null() {
            continue;
          }
          let before: i32 = field(relation, 0x18);
          if !(0..=2).contains(&before)
            || !native!(CAN_ADVANCE, unsafe extern "C" fn(Object, Object, Method) -> bool)(
              a, b, NONE,
            )
          {
            continue;
          }
          native!(LEVEL_UP, unsafe extern "C" fn(Object, Object, Method))(a, b, NONE);
          let after: i32 = field(relation, 0x18);
          if after != before + 1 {
            continue;
          }
          // The pinned native UpdateAchive body never reads its receiver. It
          // preserves protagonist-specific achievements and friend-list updates.
          native!(UPDATE_ACHIEVE, unsafe extern "C" fn(Object, Object, Object, i32, Method))(
            ptr::null_mut(),
            a,
            b,
            after,
            NONE,
          );
        }
      }
    }
  }
}
