//! Cobalt Plugin Settings labels and pinned game UI entry points.

pub const ITEM_CTOR: usize = 0x025379A0;
pub const UPDATE_TEXT: usize = 0x02537000;
pub const CHANGE_NUMBER: usize = 0x02537920;
pub const TEXT_SIGNATURES: &[(usize, [u32; 4])] = &[
  (0x025379A0, [0xA9BC7BFD, 0xF9000BF7, 0x910003FD, 0xA90257F6]),
  (0x02537000, [0xA9BE7BFD, 0xA9014FF4, 0x910003FD, 0xB001EF14]),
  (0x02537920, [0xA9BF7BFD, 0x910003FD, 0x1E620000, 0x4B010048]),
  (0x02536FD0, [0xA9BE7BFD, 0xF9000BF3, 0x910003FD, 0xAA1F03E1]),
  (0x02536FE0, [0xAA0003F3, 0x97FCBD63, 0x52800028, 0x39020268]),
  (0x02536FF0, [0xAA1303E0, 0xF9400BF3, 0xA8C27BFD, 0x14000001]),
  (0x025370F0, [0xA9BE7BFD, 0xF9000BF3, 0x910003FD, 0xAA1F03E1]),
  (0x02537100, [0xAA0003F3, 0x97FCBD73, 0x3902027F, 0xAA1303E0]),
  (0x02537110, [0xF9400BF3, 0xA8C27BFD, 0x17FFFFBA, 0xD503201F]),
];

pub fn title(language: i32) -> &'static str {
  match language {
    0 => "まとめて精製する回数",
    2 | 5 => "nombre de créations",
    3 | 6 => "cantidad de creaciones",
    7 => "Ringanzahl pro Stapel",
    8 => "quantità creazione anelli",
    9 => "牽絆戒指精煉次數",
    10 => "牵绊戒指精炼次数",
    11 => "일괄 정제 횟수",
    _ => "Batch Ring Count",
  }
}

#[cfg(target_os = "switch")]
pub(crate) mod native;

pub fn option_title(language: i32, kind: usize) -> &'static str {
  if kind == 33 || kind == 34 {
    return crate::cleaning::option_title(language, kind == 34);
  }
  if crate::resources::is_setting(kind) {
    return crate::resources::title(language, kind);
  }
  if kind >= 15 {
    return multiplier_title(language, kind == 16);
  }
  if kind >= 8 {
    return feature_title(language, kind - 8);
  }
  if kind == 0 {
    return title(language);
  }
  if kind >= 4 {
    return preference_title(language, kind);
  }
  let labels = match language {
    0 => ["料理当番", "料理の出来", "支援会話を自動再生"],
    2 | 5 => ["chef", "résultat du repas", "soutiens automatiques"],
    3 | 6 => ["cocinero", "resultado de cocina", "apoyos automáticos"],
    7 => ["Koch", "Kochergebnis", "automatische Unterstützungen"],
    8 => ["cuoco", "risultato cucina", "sostegni automatici"],
    9 => ["料理廚師", "料理結果", "自動完成支援對話"],
    10 => ["料理厨师", "料理结果", "自动完成支援对话"],
    11 => ["요리사", "요리 결과", "자동 지원 회화"],
    _ => ["Chef", "Cooking Outcome", "Automatic Supports"],
  };
  labels[kind.saturating_sub(1).min(2)]
}

pub fn default_label(language: i32) -> &'static str {
  match language {
    0 => "通常",
    2 | 5 => "Normal",
    3 | 6 => "Normal",
    7 => "Original",
    8 => "Normale",
    9 => "預設",
    10 => "默认",
    11 => "기본",
    _ => "Default",
  }
}

pub fn support_label(language: i32, enabled: bool) -> &'static str {
  let pair = match language {
    0 => ["オフ", "オン"],
    2 | 5 => ["Non", "Oui"],
    3 | 6 => ["No", "Sí"],
    7 => ["Aus", "Ein"],
    8 => ["No", "Sì"],
    9 => ["關閉", "開啟"],
    10 => ["关闭", "开启"],
    11 => ["끄기", "켜기"],
    _ => ["Off", "On"],
  };
  pair[usize::from(enabled)]
}

pub fn outcome_label(language: i32, option: u8) -> &'static str {
  if option == 0 {
    return default_label(language);
  }
  let labels = match language {
    0 => ["得意料理1", "得意料理2", "得意料理3", "普通", "失敗"],
    2 | 5 => ["Spécialité 1", "Spécialité 2", "Spécialité 3", "Ordinaire", "Échec"],
    3 | 6 => ["Especialidad 1", "Especialidad 2", "Especialidad 3", "Normal", "Fallo"],
    7 => ["Spezialität 1", "Spezialität 2", "Spezialität 3", "Gewöhnlich", "Misslungen"],
    8 => ["Specialità 1", "Specialità 2", "Specialità 3", "Comune", "Fallito"],
    9 => ["廚師特色1", "廚師特色2", "廚師特色3", "普通", "失敗"],
    10 => ["厨师特色1", "厨师特色2", "厨师特色3", "普通", "失败"],
    11 => ["특기 요리 1", "특기 요리 2", "특기 요리 3", "보통", "실패"],
    _ => ["Chef Specialty 1", "Chef Specialty 2", "Chef Specialty 3", "Ordinary", "Failure"],
  };
  labels[usize::from(option.saturating_sub(1).min(4))]
}

pub const OPTION_COUNT: usize = 35;
pub const MENU_ORDER: [usize; OPTION_COUNT] = [
  15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 5, 6, 3, 33, 34, 4, 7, 8,
  0, 9, 10, 12, 13, 14, 11, 1, 2,
];

fn multiplier_title(language: i32, sp: bool) -> &'static str {
  let labels = match language {
    0 => ["経験値倍率", "SP倍率"],
    2 | 5 => ["Multiplicateur d’EXP", "Multiplicateur de SP"],
    3 | 6 => ["Multiplicador de EXP", "Multiplicador de SP"],
    7 => ["EP-Multiplikator", "SP-Multiplikator"],
    8 => ["Moltiplicatore ESP", "Moltiplicatore SP"],
    9 => ["經驗倍率", "SP倍率"],
    10 => ["经验倍率", "SP倍率"],
    11 => ["경험치 배율", "SP 배율"],
    _ => ["EXP Multiplier", "SP Multiplier"],
  };
  labels[usize::from(sp)]
}

pub fn multiplier_label(language: i32, multiplier: u32) -> String {
  if multiplier != 0 {
    return format!("{multiplier}×");
  }
  match language {
    0 => "レベル最大",
    2 | 5 => "Niveau max.",
    3 | 6 => "Nivel máximo",
    7 => "Max. Level",
    8 => "Livello massimo",
    9 => "滿級",
    10 => "满级",
    11 => "최대 레벨",
    _ => "Max Level",
  }
  .to_owned()
}

pub fn feature_title(language: i32, feature: usize) -> &'static str {
  let labels = match language {
    0 => [
      "釣り回数を消費しない",
      "ショップ在庫無限",
      "贈り物を消費しない",
      "繰り返し料理",
      "高速釣り",
      "筋肉体操で即満点",
      "ドラゴンシューターで即SSS",
    ],
    2 | 5 => [
      "Pêche sans consommation",
      "Stocks illimités",
      "Cadeaux sans consommation",
      "Cuisine répétable",
      "Pêche rapide",
      "Musculation parfaite rapide",
      "Vol en wyverne SSS rapide",
    ],
    3 | 6 => [
      "Pesca sin gastar usos",
      "Existencias ilimitadas",
      "Regalos sin consumo",
      "Cocina repetible",
      "Pesca rápida",
      "Ejercicio perfecto rápido",
      "Vuelo en wyvern SSS rápido",
    ],
    7 => [
      "Angeln ohne Verbrauch",
      "Unbegrenzte Ladenbestände",
      "Geschenke ohne Verbrauch",
      "Wiederholt kochen",
      "Schnelles Angeln",
      "Schnelles perfektes Training",
      "Schneller SSS-Wyvernflug",
    ],
    8 => [
      "Pesca senza consumare usi",
      "Scorte illimitate",
      "Regali senza consumo",
      "Cucina ripetibile",
      "Pesca rapida",
      "Esercizi perfetti rapidi",
      "Volo wyvern SSS rapido",
    ],
    9 => [
      "釣魚次數不減",
      "商店庫存無限",
      "贈送禮物不消耗",
      "重複料理",
      "快速釣魚",
      "肌肉體操快速滿分",
      "飛龍射手快速SSS",
    ],
    10 => [
      "钓鱼次数不减",
      "商店库存无限",
      "赠送礼物不消耗",
      "重复料理",
      "快速钓鱼",
      "肌肉体操快速满分",
      "飞龙射手快速SSS",
    ],
    11 => [
      "낚시 횟수 감소 없음",
      "상점 재고 무제한",
      "선물 소모 없음",
      "반복 요리",
      "빠른 낚시",
      "빠른 체력 단련 만점",
      "빠른 드래곤 슈터 SSS",
    ],
    _ => [
      "Nondecreasing Fishing Uses",
      "Unlimited Shop Stock",
      "Inventory-free Gifts",
      "Repeat Cooking",
      "Fast Fishing",
      "Fast Perfect Strength Training",
      "Fast SSS Wyvern Ride",
    ],
  };
  labels[feature]
}

fn preference_title(language: i32, kind: usize) -> &'static str {
  let labels = match language {
    0 => ["竜の時水晶の回数", "戦闘中のセーブ", "専用衣装", "通常鍛錬の回数"],
    2 | 5 => [
      "Nombre de Cristaux temporels",
      "Sauvegarde en combat",
      "Tenues personnelles",
      "Nombre d'entraînements standard",
    ],
    3 | 6 => [
      "Usos del cristal temporal",
      "Guardado en combate",
      "Atuendos personales",
      "Entrenamientos estándar",
    ],
    7 => {
      ["Zeitkristall-Nutzungen", "Speichern im Kampf", "Persönliche Kleidung", "Standardtrainings"]
    }
    8 => [
      "Usi del cristallo temporale",
      "Salvataggio in battaglia",
      "Abiti personali",
      "Allenamenti standard",
    ],
    9 => ["龍之時水晶次數", "戰鬥存檔", "專屬服裝", "一般鍛鍊次數"],
    10 => ["龙之时水晶次数", "战斗存档", "专属服装", "一般锻炼次数"],
    11 => ["용의 시수정 횟수", "전투 중 저장", "전용 의상", "일반 단련 횟수"],
    _ => {
      ["Draconic Time Crystal Uses", "Battle Save", "Personal Outfits", "Standard Training Uses"]
    }
  };
  labels[kind - 4]
}

pub fn count_label(language: i32, count: u32) -> String {
  if count != 0 {
    return count.to_string();
  }
  match language {
    0 => "減らない",
    2 | 5 => "Sans diminution",
    3 | 6 => "Sin gastar",
    7 => "Ohne Verbrauch",
    8 => "Senza consumo",
    9 => "不減",
    10 => "不减",
    11 => "감소 없음",
    _ => "Nondecreasing",
  }
  .to_owned()
}
