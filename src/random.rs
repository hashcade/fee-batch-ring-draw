//! Read-only look-ahead matching App.Random.GetValue in Engage 2.0.0.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct State(pub [u32; 4]);

impl State {
  pub fn value(&mut self, modulus: u32) -> Option<u32> {
    if modulus == 0 || modulus > i32::MAX as u32 || self.0 == [0; 4] {
      return None;
    }
    let [x, y, z, w] = self.0;
    let t = x ^ (x << 11);
    let next = t ^ (t >> 8) ^ w ^ (w >> 19);
    self.0 = [y, z, w, next];
    Some((next & 0x7fff_ffff) % modulus)
  }
}

#[derive(Clone, Copy, Debug)]
pub struct Rates {
  pub s: u32,
  pub a: u32,
  pub b: u32,
}

impl Rates {
  pub fn rank(self, roll: u32) -> Option<usize> {
    let a = self.s.checked_add(self.a)?;
    let b = a.checked_add(self.b)?;
    if b > 100 || roll >= 100 {
      return None;
    }
    Some(if roll < self.s {
      3
    } else if roll < a {
      2
    } else if roll < b {
      1
    } else {
      0
    })
  }
}

pub fn preview(state: State, rates: Rates, counts: [usize; 4]) -> Option<(usize, usize, State)> {
  let mut next = state;
  let rank = rates.rank(next.value(100)?)?;
  let count = u32::try_from(counts[rank]).ok()?;
  let index = next.value(count)? as usize;
  Some((rank, index, next))
}
