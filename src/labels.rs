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
