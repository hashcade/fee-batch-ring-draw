# FEE Better Somniel

Somniel quality-of-life improvements for **Fire Emblem Engage 2.0.0**, using **Cobalt 1.31.0**.

## Features

- **Batch Bond Ring creation:** replace Create Ten with a configurable batch, defaulting to **300**. Keep normal costs and probabilities, skip the result animation, and view acquired rings in the original ring list.
- **Batch Bond Ring melding:** meld as many copies of the selected kind and current rank as materials, fragments, and capacity allow. Keep normal costs and equipped copies, and automatically skip the result animation.
- **Fast ring and bracelet cleaning:** finish cleaning with one effective cleaning action.
- **Quick Sommie interactions:** keep the original feeding and petting choices, shorten feeding waits, and automatically finish a petting round after its initial interaction. Keep native preferences, affection and reward eligibility.
- **Fast fishing:** shorten the bite wait and simplify landing fish while keeping the original fishing allowance and rewards.
- **Fast perfect strength training:** quickly finish push-ups, sit-ups, and squats with perfect results while keeping the original training allowance.
- **Fast SSS Wyvern Ride:** after the opening, finish at the selected difficulty's SSS score threshold and continue through the original result and reward flow. Keep difficulty unlocks and reward eligibility unchanged.
- **Repeatable cooking and food selection:** cook multiple times between battles and select food without the usual stock restriction.
- **Inventory-free ordinary gifts:** give normal gifts even at zero stock without consuming owned copies. Keep the original recipient preferences and support gains.
- **Collect exploration rewards:** use the first entry in the original exploration menu to collect generated ground items, farm products, eligible animals, and available ordinary villager gifts in the Somniel or after battle. Keep donation bonuses, adoption requirements, and inventory limits; leave rewards that do not fit unclaimed.

All features are enabled automatically. Ring creation labels and melding confirmations follow the game's text language.

## Installation

Download the ZIP from [Releases](https://github.com/hashcade/fee-better-somniel/releases/latest).

### Nintendo Switch

Install [Cobalt](https://github.com/Raytwo/Cobalt), then extract the ZIP's `engage` directory at the SD card root.

### Emulators

Install Cobalt for the emulator, then extract `engage` into its **virtual SD card root**, not its per-game mod folder. See [Cobalt mod management](https://github.com/Raytwo/Cobalt/wiki/Managing-your-Mods).

The installed plugin path is:

```text
engage/mods/fee_better_somniel/fee_better_somniel.nro
```

Fully restart the game after installation or an update.

## Configuration

The optional configuration file belongs at the SD card root:

```text
sdmc:/fee-better-somniel.cfg
```

```ini
draw_count=300
```

`draw_count` accepts **2–1000**. Without a configuration file, or with an invalid value, the default is **300**. Single-ring creation remains unchanged.

A [sample configuration](config/fee-better-somniel.cfg) is provided separately. Fully restart the game after changing it.

## Credits

- [Cobalt](https://github.com/Raytwo/Cobalt) — Engage plugin loader and runtime.
- [skyline-rs](https://github.com/ultimate-research/skyline-rs) — plugin runtime bindings.
- [engage-il2cpp](https://github.com/DivineDragonFanClub/engage-il2cpp) — native interface references.
- [KaitoB](https://tekqart.com/thread-327061-1-1.html) — original activity and gift patch references.
- [Triabolical / MyCodeMods](https://github.com/triabolicals/triabolical-code-mods) — collection-menu behavior reference.

Fire Emblem and related names are trademarks of Nintendo and Intelligent Systems.
This unofficial fan project is not affiliated with or endorsed by them.

## License

[MIT](LICENSE) License © [jinghaihan](https://github.com/jinghaihan)
