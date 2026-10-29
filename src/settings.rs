//! Cobalt Plugin Settings labels and pinned game UI entry points.

pub const ITEM_CTOR: usize = 0x025379A0;
pub const UPDATE_TEXT: usize = 0x02537000;
pub const CHANGE_NUMBER: usize = 0x02537920;
pub const TEXT_SIGNATURES: &[(usize, [u32; 4])] = &[
  (0x025379A0, [0xA9BC7BFD, 0xF9000BF7, 0x910003FD, 0xA90257F6]),
  (0x02537000, [0xA9BE7BFD, 0xA9014FF4, 0x910003FD, 0xB001EF14]),
  (0x02537920, [0xA9BF7BFD, 0x910003FD, 0x1E620000, 0x4B010048]),
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

pub const MENU_ORDER: [usize; 8] = [4, 5, 6, 7, 0, 3, 1, 2];

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
