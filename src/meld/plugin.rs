use super::{
  batch::{self, Cost},
  native::{self, Method, Object},
  profile,
  session::{Dispatch, Execution, Presentation, Session},
};
use core::{
  cell::{Cell, RefCell},
  ptr,
};

std::thread_local! {
  static SESSION: RefCell<Session> = RefCell::new(Session::default());
  static DISPLAYING: Cell<bool> = const { Cell::new(false) };
  static EXECUTION: RefCell<Execution> = RefCell::new(Execution::default());
  static IN_TRANSACTION: Cell<bool> = const { Cell::new(false) };
  static PRESENTATION: RefCell<Presentation> = RefCell::new(Presentation::default());
}

pub fn install() {
  skyline::install_hooks!(
    meld_on_select,
    meld_on_decide,
    meld_on_yes,
    meld_on_cancel,
    meld_destroy_menu,
    meld_execute_merge,
    meld_create_demo,
    meld_unload,
    meld_one,
    meld_yes_no_dialog
  );
}

#[skyline::hook(offset = profile::ON_SELECT)]
fn meld_on_select(menu: Object, ring: Object, method: Method) {
  SESSION.with(|s| s.borrow_mut().clear());
  PRESENTATION.with(|p| p.borrow_mut().clear());
  call_original!(menu, ring, method);
}

#[skyline::hook(offset = profile::ON_DECIDE)]
fn meld_on_decide(
  menu: Object,
  ring: Object,
  rings: i32,
  fragments: i32,
  god_index: i32,
  menu_select: Object,
  method: Method,
) {
  let selected = unsafe { native::selection(menu, ring, Cost { rings, fragments }) };
  SESSION.with(|s| s.borrow_mut().prepare(selected));
  let previous = DISPLAYING.with(|v| v.replace(true));
  // Original per-meld costs are never overwritten, even while constructing
  // the confirmation dialog. Result creation therefore remains native.
  call_original!(menu, ring, rings, fragments, god_index, menu_select, method);
  DISPLAYING.with(|v| v.set(previous));
}

#[skyline::hook(offset = profile::YES_NO_DIALOG)]
fn meld_yes_no_dialog(
  parent: Object,
  message: Object,
  yes: Object,
  no: Object,
  method: Method,
) -> Object {
  let replacement = if DISPLAYING.with(Cell::get) {
    let prompt = SESSION.with(|s| s.borrow().prompt().map(str::to_owned));
    prompt.map(|p| native::new_text(&p)).unwrap_or(ptr::null_mut())
  } else {
    ptr::null_mut()
  };
  let message = if replacement.is_null() { message } else { replacement };
  let dialog = call_original!(parent, message, yes, no, method);
  if !replacement.is_null() && !dialog.is_null() {
    SESSION.with(|s| s.borrow_mut().mark_displayed());
  }
  dialog
}

#[skyline::hook(offset = profile::ON_YES)]
fn meld_on_yes(menu: Object, method: Method) {
  SESSION.with(|s| s.borrow_mut().confirm(menu as usize));
  call_original!(menu, method);
}

#[skyline::hook(offset = profile::ON_CANCEL)]
fn meld_on_cancel(menu: Object, method: Method) {
  SESSION.with(|s| s.borrow_mut().clear());
  PRESENTATION.with(|p| p.borrow_mut().clear());
  call_original!(menu, method);
}

#[skyline::hook(offset = profile::DESTROY_MENU)]
fn meld_destroy_menu(menu: Object, method: Method) {
  SESSION.with(|s| s.borrow_mut().destroy_menu(menu as usize));
  call_original!(menu, method);
}

#[skyline::hook(offset = profile::EXECUTE_MERGE)]
fn meld_execute_merge(sequence: Object, method: Method) {
  let plan = SESSION.with(|s| s.borrow_mut().take_confirmed());
  let batch = plan.is_some();
  // Nested execution is not another opportunity to consume inventory.
  if !EXECUTION.with(|e| e.borrow_mut().begin(plan)) {
    return;
  }
  PRESENTATION.with(|p| p.borrow_mut().clear());
  // Keep footer refresh, acquisition state, and the original outer flow.
  call_original!(sequence, method);
  EXECUTION.with(|e| e.borrow_mut().end());
  if batch {
    PRESENTATION.with(|p| p.borrow_mut().arm(sequence as usize));
  }
}

#[skyline::hook(offset = profile::CREATE_DEMO)]
fn meld_create_demo(sequence: Object, method: Method) {
  if PRESENTATION.with(|p| p.borrow_mut().take(sequence as usize)) {
    // Skip before HideHeader, HideFooter, background unbinding, and the child
    // RingSynthesisSequence are created. ExecuteRingMerge has already settled
    // costs, records, and footer state; the outer resource meld_unload still runs.
    return;
  }
  call_original!(sequence, method);
}

#[skyline::hook(offset = profile::MELD_ONE)]
fn meld_one(rnid: Object, rings: i32, fragments: i32, method: Method) -> bool {
  if IN_TRANSACTION.with(Cell::get) {
    return false;
  }
  let selected = match EXECUTION.with(|e| e.borrow_mut().dispatch()) {
    Dispatch::Vanilla => return call_original!(rnid, rings, fragments, method),
    Dispatch::Suppress => return false,
    Dispatch::Batch(selected) => selected,
  };
  let cost = Cost { rings, fragments };
  if cost != selected.plan.cost || unsafe { native::text(rnid) }.as_deref() != Some(&selected.rnid)
  {
    return false;
  }
  let Some(mut inventory) = (unsafe { native::GameInventory::new(rnid, method, original!()) })
  else {
    return false;
  };
  IN_TRANSACTION.with(|v| v.set(true));
  let outcome = batch::execute(&mut inventory, selected.plan);
  IN_TRANSACTION.with(|v| v.set(false));
  // Never fall back to another original call after any attempted batch.
  outcome.any_new
}

#[skyline::hook(offset = profile::UNLOAD)]
fn meld_unload(sequence: Object, method: Method) {
  SESSION.with(|s| s.borrow_mut().clear());
  EXECUTION.with(|e| e.borrow_mut().end());
  PRESENTATION.with(|p| p.borrow_mut().clear());
  call_original!(sequence, method);
}
