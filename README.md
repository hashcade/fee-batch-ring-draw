# FEE Better Somniel

Somniel quality-of-life improvements for **Fire Emblem Engage 2.0.0**, using **Cobalt 1.31.0**.

## Features

- **Unlimited shop stock:** keep unlocked armory, item-shop and flea-market goods available, including sold-out goods. Keep native prices, chapter requirements and inventory limits. Outfit/accessory ownership rules stay unchanged.
- **Somniel facility shortcuts:** open the notice board, Ring Chamber, Ancient Well, animal management, shops, jukebox, arena, cooking, Sommie, strength training, fishing, Wyvern Ride and recreation from the outdoor exploration menu. Keep native unlocks, activity allowances, scene setup and cleanup; return to the menu on exit. Ring cleaning remains at its original facility.
- **Automatic ready supports:** on entering the Somniel or post-battle exploration, process eligible C/B/A support conversations through their original menus and events once the scene is ready. Opening the Somniel exploration menu also checks newly available pairs. Do not add support points or force ranks; leave Pact Ring events and the conversation gallery unchanged.
- **Batch Bond Ring creation:** keep Create One and Create Ten, and add a configurable batch, defaulting to **300**. Batch creation keeps normal costs and probabilities, skips the result animation, and leaves acquired rings in the original ring list.
- **Single or batch Bond Ring melding:** choose one copy or as many copies of the selected kind and current rank as materials, fragments, and capacity allow, in the original confirmation. Keep normal costs and equipped copies; batch melding automatically skips the result animation.
- **Single or all-ring cleaning:** keep one-action individual cleaning, or select Polish All at the original facility and choose a cleaner once. Clean eligible dirty rings and bracelets without individual rubbing, keeping native bond rewards for that cleaner.
- **Quick Sommie interactions:** keep the original feeding and petting choices, shorten feeding waits, and automatically finish a petting round after its initial interaction. Keep native preferences, affection and reward eligibility.
- **Quick arena training:** skip ordinary and paid Emblem-training battles. Keep native EXP, costs, growth, activity allowances and cleanup.
- **Quick relationship dialogues:** automatically skip messages in selected C/B/A support and Emblem bond conversations, including paid training. Keep native scripts, viewed records, rank advancement and callbacks. Story and Pact Ring conversations stay unchanged.
- **Fast fishing:** shorten the bite wait and simplify landing fish while keeping the original fishing allowance and rewards.
- **Ancient Well preview and reroll:** the original well menu includes Preview Results and Reroll Results for a pending exchange. Preview uses a private RNG and lists item quantities without claiming anything. Reroll changes only that exchange's seed; normal waiting, deposits and collection remain.
- **Fast perfect strength training:** quickly finish push-ups, sit-ups, and squats with perfect results while keeping the original training allowance.
- **Fast SSS Wyvern Ride:** after the opening, finish at the selected difficulty's SSS score threshold and continue through the original result and reward flow. Keep difficulty unlocks and reward eligibility unchanged.
- **Cooking choices:** select a recruited chef and a native outcome in Plugin Settings. Preview its grade and basic stat modifiers in the help text. Shorten the preparation animation, cook multiple times between battles, and select food without the usual stock restriction. Default chef/outcome choices retain the original rotation and roll.
- **Inventory-free ordinary gifts:** give normal gifts even at zero stock without consuming owned copies. Keep the original recipient preferences and support gains.
- **Collect exploration rewards:** use the first entry in the original exploration menu to collect generated ground items, farm products, eligible animals, and available ordinary villager gifts in the Somniel or after battle. Keep donation bonuses, adoption requirements, and inventory limits; leave rewards that do not fit unclaimed.

Convenience features are enabled automatically; automatic supports can be disabled in Plugin Settings. Menu labels follow the game's text language.

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

Open **Settings → Plugin Settings** in the game. Somniel entries include
**Batch Ring Count**, **Chef**, **Cooking Outcome**, and **Automatic Supports**.
Use the original left/right controls; changes save automatically.

Batch Ring Count defaults to **300**,
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

Chef and Cooking Outcome default to **Native**. Specialty choices use that
chef's existing taste definitions and retain native conditional variations;
the bad branch keeps its original eligible-outcome roll. A selected chef who
has not joined the current save falls back to the original rotation.
Previewed modifiers describe the taste template, not the final dish's total;
recipes and recipient conditions still apply normally.
Automatic Supports defaults to **On**. Mod preferences are not stored in game
saves; completing a support uses the game's normal progress and viewed records.

## Credits

- [Cobalt](https://github.com/Raytwo/Cobalt) — Engage plugin loader and runtime.
- [skyline-rs](https://github.com/ultimate-research/skyline-rs) — plugin runtime bindings.
- [engage-il2cpp](https://github.com/DivineDragonFanClub/engage-il2cpp) — native interface references.
- [KaitoB](https://tekqart.com/thread-327061-1-1.html) — original activity and gift patch references.
- [Triabolical / MyCodeMods](https://github.com/triabolicals/triabolical-code-mods) — collection, facility and relationship-convenience behavior references.

Fire Emblem and related names are trademarks of Nintendo and Intelligent Systems.
This unofficial fan project is not affiliated with or endorsed by them.

## License

[MIT](LICENSE) License © [jinghaihan](https://github.com/jinghaihan)
