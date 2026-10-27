# FEE Better Somniel

Somniel quality-of-life improvements for **Fire Emblem Engage 2.0.0**, using **Cobalt 1.31.0**.

## Features

- **Batch Bond Ring creation:** keep Create One and Create Ten, and add a configurable batch, defaulting to **300**. Batch creation keeps normal costs and probabilities, skips the result animation, and leaves acquired rings in the original ring list.
- **Single or batch Bond Ring melding:** choose one copy or as many copies of the selected kind and current rank as materials, fragments, and capacity allow, in the original confirmation. Keep normal costs and equipped copies; batch melding automatically skips the result animation.
- **Single or all-ring cleaning:** keep one-action individual cleaning, or select Polish All at the original facility and choose a cleaner once. Clean eligible dirty rings and bracelets without individual rubbing, keeping native bond rewards for that cleaner.
- **Quick Sommie interactions:** keep the original feeding and petting choices, shorten feeding waits, and automatically finish a petting round after its initial interaction. Keep native preferences, affection and reward eligibility.
- **Quick Emblem training:** skip the paid Emblem-training battle and automatically skip its bond-dialogue messages. Keep native costs, growth, conversation completion and cleanup. Ordinary arena training, support and story conversations stay unchanged.
- **Fast fishing:** shorten the bite wait and simplify landing fish while keeping the original fishing allowance and rewards.
- **Cumulative level display:** the character status screen shows current/total level. Total means stored current level plus internal level; no level, EXP or growth values are changed.
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

## Settings

Open **Settings → Plugin Settings → Somniel: Batch Ring Count** in the game.
Adjust the value with the original left/right controls. The default is **300**,
with a range of **2–1000**. Changes are saved automatically and apply when the
ring-creation menu is opened again; no restart or manual file editing is needed.

Original single-ring and ten-ring creation remain available. Setting the batch
count to **10** does not add a duplicate ten-ring choice.

Settings are stored independently at the SD card root:

```text
sdmc:/fee-better-somniel.cfg
```

The plugin does not add settings variables to game saves. Without a valid settings
file, the default is used. If saving fails, the old value remains active and the
setting's help text reports the failure.

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
