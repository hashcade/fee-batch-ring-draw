//! Labels follow the game's text language, independently of voice language.

pub fn count(language: i32, count: u32) -> String {
  match language {
    0 => format!("{count}回精製する"),
    2 | 5 => format!("Créer {count} anneaux"),
    3 | 6 => format!("Crear {count}"),
    7 => format!("{count} erschaffen"),
    8 => format!("Crea {count}"),
    9 => format!("精煉{count}次"),
    10 => format!("精炼{count}次"),
    11 => format!("{count}회 정제"),
    _ => format!("Create {count}"),
  }
}

/// Exact thousands, shared by every language. Never round the actual cost.
/// Small prices keep the original native display.
pub fn price(amount: i32) -> Option<String> {
  if amount < 1000 {
    return None;
  }
  let whole = amount / 1000;
  let remainder = amount % 1000;
  if remainder == 0 {
    Some(format!("{whole}k"))
  } else {
    let fraction = format!("{remainder:03}");
    Some(format!("{whole}.{}k", fraction.trim_end_matches('0')))
  }
}
