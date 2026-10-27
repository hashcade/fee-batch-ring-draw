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
