# Project guidelines

- Write source, comments, scripts, and documentation in English.
- Keep commits atomic. Do not release or install unless explicitly requested.
- Follow the engineering and Cobalt ABI conventions of fee-batch-ring-meld.
- Scope local verification to the change; CI owns full regression checks.
- Never modify user saves or installations. Supplied executable analysis is read-only.
- Check game version, Build ID, and every native entry/signature before hooking.
- Run transactions synchronously on the original game menu thread.
- Preserve native prices, probabilities, acquisition records, achievements, and equipped rings.
- Preview only a copy of RNG state; never advance or restore the live RNG to skip a full result.
- Stop before an unaffordable or unrecordable result. Never retry after an attempted transaction.
- Keep the original outer flow; bypass presentation before changing the scene, background, or title bar.
- Support the requested draw_count configuration only; do not add logs, hotkeys, result windows, or quantity dialogs.
- Document Nintendo Switch installation before emulators. Use standard MIT attribution.
