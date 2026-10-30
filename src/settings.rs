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

pub fn help(language: i32, failed: bool) -> &'static str {
  if failed {
    return match language {
      0 => "設定を保存できません。SDカードを確認してください。設定は変更されていません。",
      2 | 5 => "Échec de sauvegarde. Vérifiez la carte SD. Le réglage reste inchangé.",
      3 | 6 => "No se pudo guardar. Comprueba la tarjeta SD. El ajuste no ha cambiado.",
      7 => "Speichern fehlgeschlagen. SD-Karte prüfen. Einstellung unverändert.",
      8 => "Salvataggio non riuscito. Controlla la scheda SD. Impostazione invariata.",
      9 => "無法儲存設定，請檢查SD卡。原設定未改變。",
      10 => "无法保存设置，请检查SD卡。原设置未改变。",
      11 => "설정을 저장할 수 없습니다. SD 카드를 확인하세요. 기존 설정은 유지됩니다.",
      _ => "Cannot save settings. Check the SD card. The previous value is unchanged.",
    };
  }
  match language {
    0 => "まとめて精製する回数を変更します。1回と10回の選択肢はそのままです。",
    2 | 5 => "Nombre d'anneaux créés en lot. Les choix 1 et 10 sont conservés.",
    3 | 6 => "Cantidad de anillos por lote. Se mantienen las opciones de 1 y 10.",
    7 => "Ringanzahl pro Stapel. Die Optionen für 1 und 10 bleiben erhalten.",
    8 => "Anelli creati per gruppo. Le opzioni da 1 e 10 rimangono disponibili.",
    9 => "調整批量精煉次數。保留原版1次及10次選項。",
    10 => "调整批量精炼次数。保留原版1次和10次选项。",
    11 => "일괄 정제 횟수를 조정합니다. 기존 1회와 10회 선택지는 유지됩니다.",
    _ => "Adjust batch ring creation. Original one-ring and ten-ring choices remain available.",
  }
}

#[cfg(target_os = "switch")]
pub(crate) mod native;

pub fn option_title(language: i32, kind: usize) -> &'static str {
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
    9 => "原版",
    10 => "原版",
    11 => "기본",
    _ => "Native",
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
    0 => ["得意料理1", "得意料理2", "得意料理3", "普通", "失敗（通常の抽選）"],
    2 | 5 => ["Spécialité 1", "Spécialité 2", "Spécialité 3", "Ordinaire", "Échec (tirage normal)"],
    3 | 6 => {
      ["Especialidad 1", "Especialidad 2", "Especialidad 3", "Normal", "Fallo (sorteo normal)"]
    }
    7 => [
      "Spezialität 1",
      "Spezialität 2",
      "Spezialität 3",
      "Gewöhnlich",
      "Misslungen (Originalauswahl)",
    ],
    8 => ["Specialità 1", "Specialità 2", "Specialità 3", "Comune", "Fallito (estrazione normale)"],
    9 => ["廚師特色1", "廚師特色2", "廚師特色3", "普通", "失敗（原版抽選）"],
    10 => ["厨师特色1", "厨师特色2", "厨师特色3", "普通", "失败（原版抽选）"],
    11 => ["특기 요리 1", "특기 요리 2", "특기 요리 3", "보통", "실패 (기본 추첨)"],
    _ => {
      ["Chef Specialty 1", "Chef Specialty 2", "Chef Specialty 3", "Ordinary", "Bad (Native Roll)"]
    }
  };
  labels[usize::from(option.saturating_sub(1).min(4))]
}

pub fn option_help(language: i32, kind: usize) -> &'static str {
  if kind >= 15 {
    return match (language, kind == 16) {
      (9, false) => "調整經驗倍率，或在獲得經驗時升至目前職業滿級。",
      (10, false) => "调整经验倍率，或在获得经验时升至本职业满级。",
      (0, false) => "獲得経験値の倍率、または経験値獲得時に現在の兵種レベルを最大にします。",
      (9, true) => "獨立調整獲得的SP倍率，不影響技能書。",
      (10, true) => "独立调整获得的SP倍率，不影响技能书。",
      (0, true) => "獲得SPの倍率。経験値倍率とは独立。スキルの書は対象外。",
      (_, false) => "Multiply earned EXP, or reach the current class cap on gaining EXP.",
      (_, true) => "Multiply earned SP independently of EXP. Skill books stay unchanged.",
    };
  }
  if kind >= 8 {
    return feature_help(language, kind - 8);
  }
  if kind == 0 {
    return help(language, false);
  }
  if kind >= 4 {
    return preference_help(language, kind);
  }
  (match language {
    0 => [
      "加入済みの料理当番を選択。通常で元に戻します。",
      "料理当番の元の出来分岐を選択。食材や報酬は通常どおりです。",
      "再生可能なC/B/A支援会話だけを順に処理します。",
    ],
    2 | 5 => [
      "Choisir un chef recruté. Normal conserve le roulement du jeu.",
      "Choisir une branche native du chef. Ingrédients et récompenses restent normaux.",
      "Traiter uniquement les conversations C/B/A déjà disponibles, dans l'ordre.",
    ],
    3 | 6 => [
      "Elige un cocinero reclutado. Normal conserva el turno original.",
      "Elige una rama nativa del cocinero. Conserva ingredientes y recompensas.",
      "Procesa por orden solo las conversaciones C/B/A ya disponibles.",
    ],
    7 => [
      "Rekrutierten Koch wählen. Original behält die normale Rotation bei.",
      "Originalen Ergebniszweig des Kochs wählen. Zutaten und Belohnungen bleiben normal.",
      "Nur bereits verfügbare C/B/A-Gespräche nacheinander verarbeiten.",
    ],
    8 => [
      "Scegli un cuoco reclutato. Normale mantiene il turno originale.",
      "Scegli un ramo originale del cuoco. Ingredienti e ricompense rimangono normali.",
      "Elabora in ordine solo le conversazioni C/B/A già disponibili.",
    ],
    9 => [
      "選擇已加入的廚師。原版保留當前輪值。",
      "選擇廚師原生料理分支，保留食材及獎勵處理。",
      "依序處理已滿足條件的C/B/A支援對話。",
    ],
    10 => [
      "选择已加入的厨师。原版保留当前轮值。",
      "选择厨师原生料理分支，保留食材和奖励处理。",
      "依次处理已满足条件的C/B/A支援对话。",
    ],
    11 => [
      "영입한 요리사를 선택합니다. 기본값은 기존 당번을 유지합니다.",
      "요리사의 기본 결과 분기를 선택합니다. 재료와 보상은 유지됩니다.",
      "조건을 충족한 C/B/A 지원 회화만 순서대로 처리합니다.",
    ],
    _ => [
      "Choose a joined chef. Native keeps the original rotation.",
      "Choose the chef's native outcome branch. Keep ingredients and rewards.",
      "Process only already eligible C/B/A support conversations in order.",
    ],
  })[kind.saturating_sub(1).min(2)]
}

pub const OPTION_COUNT: usize = 17;
pub const MENU_ORDER: [usize; OPTION_COUNT] =
  [15, 16, 5, 6, 3, 4, 7, 8, 0, 9, 10, 12, 13, 14, 11, 1, 2];

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
      "力量訓練快速滿分",
      "飛龍小遊戲快速SSS",
    ],
    10 => [
      "钓鱼次数不减",
      "商店库存无限",
      "赠送礼物不消耗",
      "重复料理",
      "快速钓鱼",
      "力量训练快速满分",
      "飞龙小游戏快速SSS",
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

fn feature_help(language: i32, feature: usize) -> &'static str {
  let labels = match language {
    10 => [
      "不扣除钓鱼次数。次数已用完时提供一次可用次数。",
      "武器店、道具店及跳蚤市场库存无限，价格和解锁条件不变。",
      "普通礼物不消耗库存，也可在零库存时赠送。",
      "允许在两场战斗之间重复料理。",
      "缩短等待并简化钓起过程，不改变次数设置。",
      "快速完成力量训练并获得满分，不增加训练次数。",
      "快速按当前难度的SSS门槛结算，不增加游玩次数。",
    ],
    9 => [
      "不扣除釣魚次數。次數已用完時提供一次可用次數。",
      "武器店、道具店及跳蚤市場庫存無限，價格與解鎖條件不變。",
      "普通禮物不消耗庫存，也可在零庫存時贈送。",
      "允許在兩場戰鬥之間重複料理。",
      "縮短等待並簡化釣起過程，不改變次數設定。",
      "快速完成力量訓練並獲得滿分，不增加訓練次數。",
      "快速按目前難度的SSS門檻結算，不增加遊玩次數。",
    ],
    _ => [
      "Do not spend fishing uses. An exhausted session receives one available use.",
      "Unlimited armory, item-shop and flea-market stock; keep prices and unlocks.",
      "Give ordinary gifts without consuming copies, including at zero stock.",
      "Allow repeated cooking between battles.",
      "Shorten bite waits and simplify landing, independently of use limits.",
      "Finish strength training quickly with a perfect result; keep its allowance.",
      "Finish at the difficulty's SSS threshold; keep its native allowance.",
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

fn preference_help(language: i32, kind: usize) -> &'static str {
  match language {
    9 => match kind {
      4 => "選擇使用次數或不減。保留原版解鎖條件及可回溯紀錄。",
      5 => "允許在戰鬥中儲存。",
      6 => "戰鬥及整備時使用角色初始職業服裝。重新建立模型時生效。",
      _ => "每輪一般鍛鍊的總次數，或不減。不改變特別指名鍛鍊。",
    },
    10 => match kind {
      4 => "选择使用次数或不减。保留原版解锁条件及可回溯记录。",
      5 => "允许在战斗中保存。",
      6 => "战斗及整备时使用角色初始职业服装。重新建立模型时生效。",
      _ => "每轮一般锻炼的总次数，或不减。不改变特别指名锻炼。",
    },
    0 => match kind {
      4 => "使用回数または減らない設定。解禁条件と巻き戻せる記録はそのまま。",
      5 => "戦闘中のセーブを許可します。",
      6 => "戦闘・準備画面で初期兵種の衣装を使用。モデル再生成時に反映。",
      _ => "通常鍛錬の合計回数、または減らない設定。特別指名鍛錬は変更しません。",
    },
    _ => match kind {
      4 => "Choose a use limit or Nondecreasing. Keep native unlocks and recorded history.",
      5 => "Allow saving during battle.",
      6 => "Use original character battle clothing in battle and preparation. Applies on model rebuild.",
      _ => "Total standard training uses per native reset, or Nondecreasing. Emblem training is unchanged.",
    },
  }
}
