//! Labels follow the game's text language, independently of voice language.

pub fn count(language: i32) -> &'static str {
  match language {
    0 => "300回精製する",
    2 | 5 => "Créer 300 anneaux",
    3 | 6 => "Crear 300",
    7 => "300 erschaffen",
    8 => "Crea 300",
    9 => "精煉300次",
    10 => "精炼300次",
    11 => "300회 정제",
    _ => "Create 300",
  }
}
