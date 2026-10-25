# FEE Better Somniel

Somniel quality-of-life improvements for **Fire Emblem Engage 2.0.0**, using **Cobalt 1.31.0**.

## Features

- **Batch Bond Ring creation:** replace Create Ten with a configurable batch, defaulting to **300**. Keep normal prices, probabilities, RNG progression, acquisition records, and achievements. Skip the batch result presentation and inspect acquired rings in the original ring list.
- **Batch Bond Ring melding:** after the original confirmation, meld as many copies of the selected kind and current rank as materials, fragments, and capacity allow. Keep normal costs, preserve equipped copies, and do not chain ranks. Automatically skip the batch result animation; pressing + is unnecessary.
- **Fast ring and bracelet cleaning:** finish the current cleaning through the original flow after one effective cleaning action.
- **Fast fishing:** shorten the bite wait and simplify landing the fish. Keep the original fishing allowance, catches, and reward flow.
- **Fast perfect strength training:** retain the existing quick/perfect behavior for push-ups, sit-ups, and squats without increasing the training allowance.
- **Repeatable cooking and food selection:** remove the meal-use limit and the existing food-list stock restriction. Do not force a meal rating or replace native settlement.
- **Inventory-free ordinary gifts:** show normal gift items even at zero stock and do not consume them. Keep recipient preferences, support settlement, and the original ordinary-gift category filter.

All features are enabled together. No feature switches, hotkeys, extra menus, logs, result windows, or quantity dialogs are added. The optional configuration only controls the ring creation batch size.

Resources, shop stock, arena allowances, battle saving, and rewind limits are not changed by this plugin. An optional [companion cheat file](cheats/8C08B9719E085F91.txt) keeps those separate; Battle Save is its final entry.

## Installation

Requires [Cobalt](https://github.com/Raytwo/Cobalt). Supported executable: Engage **2.0.0**, Build ID `8C08B9719E085F91847B5E0F935D9488`. A version, instruction, or conflicting-patch mismatch prevents initialization.

```text
engage/mods/fee_better_somniel/fee_better_somniel.nro
```

### Nintendo Switch

Install Cobalt following its instructions, then extract the package's `engage` directory at the SD card root.

### Emulators

Install Cobalt for the emulator, then extract `engage` into its **virtual SD card root**, not its per-game mod folder. See [Cobalt mod management](https://github.com/Raytwo/Cobalt/wiki/Managing-your-Mods).

### Replacing the previous plugins and cheats

Remove **both** `fee_batch_ring_draw.nro` and `fee_batch_ring_meld.nro` before using this combined plugin. The old plugins must not run alongside it.

Disable/remove **Unlimited Cooking**, **Fast Fishing**, **Perfect Strength Training**, and **Fast Ring Cleaning**. The companion file already excludes them. Do not enable a separate all-gifts or gift-consumption code alongside this plugin.

Keep desired resource, stock, arena, battle-save, and rewind cheats. Fully restart the game (and emulator, if used) after changing plugins or cheats.

The plugin adds no custom item IDs, save fields, or inventory catalog entries. Its gift feature does not place free gifts into the player's inventory.

## Configuration

An optional configuration file is read once at startup from the SD card root:

```text
sdmc:/fee-better-somniel.cfg
```

```ini
draw_count=300
```

`draw_count` accepts **2–1000**, defaulting to **300** when absent or invalid. This is a plugin execution safety range, not an inventory cap. The batch label, price, affordability, execution limit, and result capacity follow the value. Single-ring creation remains unchanged.

Batch prices use exact, language-independent `k` notation (`30k`, `99.9k`, `100k`); native confirmation and charging retain full amounts. Draw labels and meld confirmations follow the game's current text language.

Rename an existing `fee-batch-ring-draw.cfg` to `fee-better-somniel.cfg` to keep a custom count. The [sample configuration](config/fee-better-somniel.cfg) is distributed separately; the plugin never writes it. Missing, unreadable, malformed, duplicate, unknown, oversized, or out-of-range settings use the default. Fully restart after editing it.

## Development

```sh
cargo test --locked
cargo skyline build --release -- --locked
python3 scripts/verify_game.py /path/to/main
python3 scripts/package.py target/aarch64-skyline-switch/release/libfee_better_somniel.nro --version "$(python3 scripts/release.py --check)"
uv run --with unicorn==2.1.4 python scripts/smoke_startup.py dist/fee_better_somniel.nro
```

Executable analysis is read-only. See [implementation](docs/IMPLEMENTATION.md) for native boundaries and [validation](docs/VALIDATION.md) for verification and remaining game checks.

## Releases

CI validates and builds the plugin. Tagged releases use **changelogithub** and publish the NRO, installable ZIP, and `SHA256SUMS`, then download and verify published artifacts.

```sh
python3 scripts/release.py --current
python3 scripts/release.py --bump patch
```

Release only when explicitly requested, from a clean `main` synchronized with GitHub. The release script does not repeat the CI build locally.

## References

- [Cobalt](https://github.com/Raytwo/Cobalt) and the [plugin development hub](https://github.com/DivineDragonFanClub/plugin_hub).
- [engage-il2cpp 0.1.0](https://github.com/DivineDragonFanClub/engage-il2cpp/tree/0.1.0), cross-checked against the supplied executable.
- **KaitoB / 怪盗B**, whose Engage cheat set provides the original activity and full-gift patch references: [author's publication](https://tekqart.com/thread-327061-1-1.html), [public code listing](https://gbatemp.net/threads/fire-emblem-engage-save-editor.624990/page-4). The activity instructions are migrated from the user's existing file; gift consumption is suppressed only at its confirmed gift-specific call site.
- Batch melding retains this project's former `fee-batch-ring-meld` v0.2.0 implementation (`d92aed4`); the source is preserved under `src/meld/`.

No game binaries, extracted assets, or save files are included.

## License

[MIT](LICENSE) License © [jinghaihan](https://github.com/jinghaihan)
