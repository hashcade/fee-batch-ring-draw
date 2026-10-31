//! Native polishing primitives and independent preference labels.

pub fn option_title(language: i32, actor: bool) -> &'static str {
  let labels = match language {
    0 => ["自動研磨", "研磨する仲間"],
    2 | 5 => ["Polissage automatique", "Personnage du polissage"],
    3 | 6 => ["Pulido automático", "Personaje del pulido"],
    7 => ["Automatisch polieren", "Polierender Charakter"],
    8 => ["Lucidatura automatica", "Personaggio lucidatura"],
    9 => ["自動研磨", "研磨角色"],
    10 => ["自动研磨", "研磨角色"],
    11 => ["자동 연마", "연마 캐릭터"],
    _ => ["Auto Polish", "Polishing Character"],
  };
  labels[usize::from(actor)]
}

pub const GET_DIRTY: usize = 0x0233EB00;
pub const SET_DIRTY: usize = 0x0233EB10;
pub const GET_STATE: usize = 0x023353F0;

pub const TEXT_SIGNATURES: &[(usize, [u32; 4])] = &[
  (0x03207830, [0xD10103FF, 0xA9027BFD, 0x910083FD, 0xF9001BF3]),
  (0x03207840, [0xAA0803F3, 0xF9400C28, 0xF9406108, 0xF940C501]),
  (0x03207850, [0xF9401400, 0xF9400029, 0x910023E8, 0xD63F0120]),
  (0x001FD360, [0xA9BE7BFD, 0xA9014FF4, 0x910003FD, 0xF9400C14]),
  (0x02340600, [0xA9BE7BFD, 0xA9014FF4, 0x910003FD, 0xAA0103F3]),
  (0x02340700, [0xA9BE7BFD, 0xA9014FF4, 0x910003FD, 0xF9401C00]),
  (0x0233EB20, [0xA9BD7BFD, 0xF9000BF5, 0x910003FD, 0xA9024FF4]),
  (0x027CA680, [0xA9BE7BFD, 0xA9014FF4, 0x910003FD, 0xF001DA73]),
  (0x027CCA60, [0xD10183FF, 0xA9017BFD, 0x910043FD, 0xF90013F9]),
  (0x0233EB00, [0x39413000, 0xD65F03C0, 0xD503201F, 0xD503201F]),
  (0x0233EB10, [0x39013001, 0xD65F03C0, 0xD503201F, 0xD503201F]),
  (0x023353F0, [0xF9401408, 0xB40000A8, 0xF9402908, 0xB40000E8]),
];

#[cfg(target_os = "switch")]
pub(crate) mod automatic;
