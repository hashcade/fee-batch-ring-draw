use crate::{
  batch, config,
  native::{self, Method, Object},
  profile,
  session::Session,
};
use core::{cell::RefCell, ptr};

std::thread_local! {
  static SESSION: RefCell<Session> = RefCell::new(Session::default());
}

pub fn install() {
  if !supported_game() {
    return;
  }
  config::load();
  crate::preferences::native::install();
  crate::fishing::native::install();
  crate::settings::native::install();
  skyline::install_hooks!(count_menu, build_text, execute_gacha, draw, create_demo, unload);
  crate::meld::plugin::install();
  crate::wyvern::native::install();
  crate::collector::native::install();
  crate::cooking::native::install();
  crate::supports::native::install();
  crate::facilities::native::install();
  crate::shops::native::install();
}

fn supported_game() -> bool {
  // Cobalt 1.31 does not provide Skyline's get_program_id export.
  let mut version = skyline::nn::oe::DisplayVersion { name: [0; 16] };
  unsafe {
    skyline::nn::oe::GetDisplayVersion(&mut version);
  }
  if version.name[..6] != [b'2', b'.', b'0', b'.', b'0', 0] {
    return false;
  }
  let base = native::base();
  let rodata = unsafe { skyline::hooks::getRegionAddress(skyline::hooks::Region::Rodata) as usize };
  let data = unsafe { skyline::hooks::getRegionAddress(skyline::hooks::Region::Data) as usize };
  if base == 0
    || rodata.checked_sub(base) != Some(profile::RODATA_OFFSET)
    || data <= rodata
    || data - base < profile::BUILD_ID_OFFSET + 16
  {
    return false;
  }
  let id =
    unsafe { core::slice::from_raw_parts((base + profile::BUILD_ID_OFFSET) as *const u8, 16) };
  if id != profile::BUILD_ID {
    return false;
  }
  // Cobalt installs its own hooks before loading plugins. Instruction windows
  // belong to offline verification, not runtime compatibility detection.
  true
}

#[skyline::hook(offset = profile::COUNT_MENU)]
fn count_menu(parent: Object, handler: Object, method: Method) -> Object {
  let menu = call_original!(parent, handler, method);
  unsafe { native::append_draw_count(menu, handler) };
  menu
}

#[skyline::hook(offset = profile::BUILD_TEXT)]
fn build_text(content: Object, method: Method) {
  call_original!(content, method);
  unsafe {
    native::set_menu_text(content);
  }
}

#[skyline::hook(offset = profile::EXECUTE_GACHA)]
fn execute_gacha(sequence: Object, method: Method) {
  if !SESSION.with(|session| session.borrow_mut().begin(sequence as usize)) {
    return;
  }
  // Retain original result assignment, footer refresh, and outer state machine.
  call_original!(sequence, method);
  SESSION.with(|session| session.borrow_mut().end());
}

#[skyline::hook(offset = profile::DRAW)]
fn draw(god: Object, times: i32, flags: *mut Object, method: Method) -> Object {
  let active = SESSION.with(|session| session.borrow().executing());
  let count = config::draw_count();
  if !active || times == 1 || times == 10 || times != count as i32 {
    return call_original!(god, times, flags, method);
  }
  let Some(sequence) = SESSION.with(|session| session.borrow_mut().dispatch(times, count)) else {
    return ptr::null_mut();
  };
  let sequence = sequence as Object;
  // This exact out-parameter is rooted by the original sequence's managed field.
  if flags != unsafe { sequence.cast::<u8>().add(0xA0).cast::<Object>() } {
    return ptr::null_mut();
  }
  let Some(mut inventory) = (unsafe { native::GameInventory::new(sequence, god, original!()) })
  else {
    return unsafe { native::empty_results(sequence) };
  };
  let price = inventory.price();
  let _outcome = batch::execute(&mut inventory, price, count);
  inventory.results()
}

#[skyline::hook(offset = profile::CREATE_DEMO)]
fn create_demo(sequence: Object, method: Method) {
  if SESSION.with(|session| session.borrow_mut().presentation(sequence as usize)) {
    // This entry only hides the header/footer, unbinds the background, and
    // creates the presentation child. Skip BEFORE any of those state changes.
    // Native AddAchieveOfRefineRing and UnloadResourcesForRefineRing remain.
    return;
  }
  call_original!(sequence, method);
}

#[skyline::hook(offset = profile::UNLOAD)]
fn unload(sequence: Object, method: Method) {
  SESSION.with(|session| session.borrow_mut().clear());
  call_original!(sequence, method);
}
