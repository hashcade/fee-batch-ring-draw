# Project guidelines

- Write source, comments, scripts, and documentation in English.
- Keep commits atomic. Do not release or install unless explicitly requested.
- Retain the existing ring transaction safety checks and Cobalt 1.31 ABI conventions.
- Scope local verification to the change; CI owns full regression checks.
- Never modify user saves or installations. Supplied executable analysis is read-only.
- Check game version, Build ID, and every native entry/signature before hooking.
- Run transactions synchronously on the original game menu thread.
- Preserve native prices, probabilities, acquisition records, achievements, and equipped rings.
- Preview only a copy of RNG state; never advance or restore the live RNG to skip a full result.
- Stop before an unaffordable or unrecordable result. Never retry after an attempted transaction.
- Keep the original outer flow; bypass presentation before changing the scene, background, or title bar.
- Support the requested draw_count configuration only; do not add logs, hotkeys, result windows, or quantity dialogs.
- Integrate batch ring creation, batch melding, fast ring cleaning, fast fishing, fast perfect strength training, and repeatable cooking in one plugin. These features are always enabled; do not add feature toggles or increase activity allowances.
- Leave arena training, battle saves, rewind limits, resource refills, and shop stock to cheats. Do not install or release without an explicit request.
- Document Nintendo Switch installation before emulators. Use standard MIT attribution.
