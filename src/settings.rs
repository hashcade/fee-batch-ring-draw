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
    0 => "ソラネル：まとめて精製する回数",
    2 | 5 => "Somniel : nombre de créations",
    3 | 6 => "Somniel: cantidad de creaciones",
    7 => "Somniel: Ringanzahl pro Stapel",
    8 => "Somniel: quantità creazione anelli",
    9 => "索拉涅爾：批量精煉次數",
    10 => "索拉涅尔：批量精炼次数",
    11 => "솔라넬: 일괄 정제 횟수",
    _ => "Somniel: Batch Ring Count",
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
  let labels = match language {
    0 => ["ソラネル：料理当番", "ソラネル：料理の出来", "ソラネル：支援会話を自動再生"],
    2 | 5 => ["Somniel : chef", "Somniel : résultat du repas", "Somniel : soutiens automatiques"],
    3 | 6 => ["Somniel: cocinero", "Somniel: resultado de cocina", "Somniel: apoyos automáticos"],
    7 => ["Somniel: Koch", "Somniel: Kochergebnis", "Somniel: automatische Unterstützungen"],
    8 => ["Somniel: cuoco", "Somniel: risultato cucina", "Somniel: sostegni automatici"],
    9 => ["索拉涅爾：廚師", "索拉涅爾：料理結果", "索拉涅爾：自動完成支援對話"],
    10 => ["索拉涅尔：厨师", "索拉涅尔：料理结果", "索拉涅尔：自动完成支援对话"],
    11 => ["솔라넬: 요리사", "솔라넬: 요리 결과", "솔라넬: 자동 지원 회화"],
    _ => ["Somniel: Chef", "Somniel: Cooking Outcome", "Somniel: Automatic Supports"],
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
    9 | 10 => ["关闭", "开启"],
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
  (match language {
    0 => [
      "加入済みの料理当番を選択。通常で元に戻します。",
      "料理当番の元の出来分岐を選択。食材や報酬は通常どおりです。",
      "再生可能なC/B/A支援会話だけを順に処理します。",
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
