//! Compact confirmation text, following App.Language's text-language IDs.

use super::batch::Plan;

fn bond_fragments(language: i32) -> &'static str {
  match language {
    0 => "絆のかけら",
    2 | 5 => "Fragments de lien",
    3 | 6 => "Fragmentos de vínculo",
    7 => "Bond-Fragmente",
    8 => "Frammenti di legame",
    9 => "羈絆碎片",
    10 => "羁绊碎片",
    11 => "인연의 조각",
    _ => "Bond Fragments",
  }
}

pub fn format(language: i32, ring_name: &str, source_rank: i32, plan: Plan) -> Option<String> {
  let (source, output) = match source_rank {
    0 => ('C', 'B'),
    1 => ('B', 'A'),
    2 => ('A', 'S'),
    _ => return None,
  };
  let total = plan.total_cost();
  Some(format!(
    "{ring_name} {source} ×{} → {output} ×{}\n{} ×{}",
    total.rings,
    plan.count,
    bond_fragments(language),
    total.fragments
  ))
}

pub fn action(language: i32, count: u32) -> String {
  match language {
    0 => format!("{count}個合成"),
    2 | 5 => format!("Fusionner {count}"),
    3 | 6 => format!("Fusionar {count}"),
    7 => format!("{count} verschmelzen"),
    8 => format!("Fondi {count}"),
    9 => format!("合成 {count} 枚"),
    10 => format!("合成 {count} 枚"),
    11 => format!("{count}개 합성"),
    _ => format!("Meld {count}"),
  }
}

pub fn choices(language: i32, ring_name: &str, source_rank: i32, plan: Plan) -> Option<String> {
  let (source, output) = match source_rank {
    0 => ('C', 'B'),
    1 => ('B', 'A'),
    2 => ('A', 'S'),
    _ => return None,
  };
  let total = plan.total_cost();
  let fragments = bond_fragments(language);
  Some(format!(
    "{ring_name} {source} → {output}\n{}: {source} ×{} · {fragments} ×{}\n{}: {source} ×{} · {fragments} ×{}",
    action(language, 1), plan.cost.rings, plan.cost.fragments,
    action(language, plan.count), total.rings, total.fragments
  ))
}
