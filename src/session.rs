//! Parent-scoped, one-shot dispatch and presentation bypass.

#[derive(Default)]
pub struct Session {
  executing: usize,
  dispatched: bool,
  ready: usize,
}

impl Session {
  pub fn begin(&mut self, parent: usize) -> bool {
    if parent == 0 || self.executing != 0 {
      return false;
    }
    self.ready = 0;
    self.executing = parent;
    self.dispatched = false;
    true
  }

  pub fn dispatch(&mut self, count: i32, configured: u32) -> Option<usize> {
    if self.executing == 0
      || self.dispatched
      || !crate::config::valid_count(configured)
      || count != configured as i32
    {
      return None;
    }
    self.dispatched = true;
    Some(self.executing)
  }

  pub fn executing(&self) -> bool {
    self.executing != 0
  }

  pub fn end(&mut self) {
    self.ready = if self.dispatched { self.executing } else { 0 };
    self.executing = 0;
    self.dispatched = false;
  }

  pub fn presentation(&mut self, parent: usize) -> bool {
    if parent == 0 || self.ready != parent {
      return false;
    }
    self.ready = 0;
    true
  }

  pub fn clear(&mut self) {
    *self = Self::default();
  }
}
