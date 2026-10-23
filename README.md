# FEE Batch Ring Draw

Create a configurable batch of Bond Rings in one operation in **Fire Emblem Engage 2.0.0**, using Cobalt. The default is **300**.

The original **Create One / Create Ten** menu becomes **Create One / Create 300**. The game calculates and displays the normal total price and disables unaffordable selections. The batch skips the result presentation and returns to the menu; inspect acquired rings in the game's ring list.

- Normal prices, rank probabilities, random sequence, acquisition records, and achievements are retained.
- Stops before the next result would exceed the native inventory limits. It does not reroll a full result or discard a ring after charging for it.
- Equipped rings remain equipped. Nothing is automatically melded.
- Single-ring creation keeps its original behavior and presentation.
- Menu text follows the game's text language and configured count. No logging, hotkeys, quantity window, or result summary is added.

The implementation is compiled and covered by host tests, executable signature checks, and a compiled-NRO startup smoke test. **In-game validation is pending.** See [validation](docs/VALIDATION.md).

## Installation

Requires [Cobalt](https://github.com/Raytwo/Cobalt). Runtime imports are checked against Cobalt **1.31.0** and Engage **2.0.0**, Build ID `8C08B9719E085F91847B5E0F935D9488`. Other executable versions are rejected before hooks are installed.

```text
engage/mods/fee_batch_ring_draw/fee_batch_ring_draw.nro
```

### Nintendo Switch

Install Cobalt following its instructions, then extract the package's `engage` directory at the SD card root.

### Emulators

Install Cobalt for the emulator, then extract `engage` into its **virtual SD card root**, not its per-game mod folder. See [Cobalt mod management](https://github.com/Raytwo/Cobalt/wiki/Managing-your-Mods).

This is independent of [FEE Batch Ring Meld](https://github.com/hashcade/fee-batch-ring-meld). The two plugins hook different menu and transaction entry points.

## Configuration

Like FETH Bench EXP, this plugin reads an optional `.cfg` file at the console's SD card root (or the emulator's virtual SD card root), not alongside the NRO:

```text
sdmc:/fee-batch-ring-draw.cfg
```

```ini
draw_count=300
```

`draw_count` accepts **2–1000**; 300 is the default. This is a plugin execution safety range, not a game inventory limit. The batch menu label, displayed price, affordability, execution limit, and result-list capacity follow this value. Single-ring creation is unchanged.

Missing, unreadable, malformed, duplicate, unknown, or out-of-range settings use the default. Configuration files larger than 4096 bytes are rejected. Empty lines and `#` comment lines are accepted. The plugin only reads the file; it does not create or overwrite it. A [sample configuration](config/fee-batch-ring-draw.cfg) is provided separately, so extracting an update does not reset existing settings.

Fully restart the game (and the emulator, if used) after changing the file.

## Development

```sh
cargo test --locked
cargo skyline build --release -- --locked
python3 scripts/package.py target/aarch64-skyline-switch/release/libfee_batch_ring_draw.nro --version 0.1.0
uv run --with unicorn==2.1.4 python scripts/smoke_startup.py dist/fee_batch_ring_draw.nro
```

Optionally validate a locally supplied executable without writing extracted files:

```sh
python3 scripts/verify_game.py /path/to/main
```

## Releases

CI validates and builds the plugin. Tagged releases use **changelogithub** and publish the NRO, installable ZIP, and `SHA256SUMS`, then download and verify the published artifacts.

```sh
python3 scripts/release.py --current
python3 scripts/release.py --bump patch
```

Only release when explicitly requested, from a clean `main` synchronized with GitHub. The release script does not repeat the CI build locally.

## References

- [Cobalt](https://github.com/Raytwo/Cobalt) and the [plugin development hub](https://github.com/DivineDragonFanClub/plugin_hub).
- [engage-il2cpp 0.1.0](https://github.com/DivineDragonFanClub/engage-il2cpp/tree/0.1.0) for native interface definitions, cross-checked against the supplied executable.

No game binaries, extracted assets, or save files are included.

## License

[MIT](LICENSE) License © [jinghaihan](https://github.com/jinghaihan)
