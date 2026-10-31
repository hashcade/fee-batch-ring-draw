# FEE Better Somniel

Somniel quality-of-life improvements for **Fire Emblem Engage 2.0.0**, using **Cobalt 1.31.0**.

## Features

- **EXP and SP rewards:** independently choose 1×, 2×, 3×, 5× or 10× rewards. EXP also offers Max Level: the next positive EXP reward reaches the current class level cap. Keep normal experience/level-up animations, native growth and level/SP caps. No automatic reclassing or stat maximization; skill books stay unchanged.
- **Unlimited shop stock:** keep unlocked armory, item-shop and flea-market goods available, including sold-out goods. Keep native prices, chapter requirements and inventory limits. Outfit/accessory ownership rules stay unchanged.
- **Somniel facility shortcuts:** one outdoor menu entry opens Facilities / Activities / Shops beside the main menu. Selecting a category hides both panels and opens a native destination page; B restores the previous menus and selection. Facilities contains the notice board, Ring Chamber, Ancient Well, animal management and jukebox; Activities contains arena, cooking, Sommie, strength training, fishing, Wyvern Ride and recreation; Shops contains the armory, item shop, smithy, boutique and flea market. Each list fits without scrolling. Keep native unlocks, activity allowances, scene setup and cleanup; return to the destination page on facility exit. Ring cleaning remains at its original facility.
- **Automatic ready supports:** on entering the Somniel, process eligible C/B/A support conversations through their original menus and events once the scene is ready. Opening the Somniel exploration menu also checks newly available pairs. Post-battle exploration does not trigger automation. Do not add support points or force ranks; leave Pact Ring events and the conversation gallery unchanged.
- **Batch Bond Ring creation:** keep Create One and Create Ten, and add a configurable batch, defaulting to **300**. Batch creation keeps normal costs and probabilities, skips the result animation, and leaves acquired rings in the original ring list.
- **Single or batch Bond Ring melding:** choose one copy or as many copies of the selected kind and current rank as materials, fragments, and capacity allow, in the original confirmation. Keep normal costs and equipped copies; batch melding automatically skips the result animation.
- **Single or all-ring cleaning:** keep one-action individual cleaning, or select Polish All at the original facility and choose a cleaner once. Clean eligible dirty rings and bracelets without individual rubbing, keeping native bond rewards for that cleaner.
- **Quick Sommie interactions:** keep the original feeding and petting choices, shorten feeding waits, and automatically finish a petting round after its initial interaction. Keep native preferences, affection and reward eligibility.
- **Quick arena training:** skip ordinary and paid Emblem-training battles. Keep native EXP, costs, growth and cleanup. Standard training defaults to ten uses per native activity reset, with a configurable quota or nondecreasing mode. Paid Emblem training is unchanged.
- **Draconic Time Crystal uses:** choose a finite quota or nondecreasing uses, which is the default. Keep original unlocks and recorded rewind history.
- **Battle saves:** allow saving during battle; enabled by default and independently switchable.
- **Personal outfits:** use each character's original class clothing in ordinary battle and preparation models, even after reclassing. Enabled by default. Keep native hair, weapons, mounts, skeletons, animations and engaged forms; incompatible skeletons retain their native outfit. Changes apply when the model is rebuilt, not immediately to existing models.
- **Quick relationship dialogues:** automatically skip messages in selected C/B/A support and Emblem bond conversations, including paid training. Keep native scripts, viewed records, rank advancement and callbacks. Story and Pact Ring conversations stay unchanged.
- **Fishing controls:** independently switch nondecreasing fishing uses and faster bite/landing. Both default to on; native unlocks, fish records and rewards remain. If uses are already exhausted, nondecreasing mode supplies one available use on entering the Somniel or enabling the option.
- **Ancient Well preview and reroll:** the original well menu includes Preview Results and Reroll Results for a pending exchange. Preview uses a private RNG and lists item quantities without claiming anything. Reroll changes only that exchange's seed; normal waiting, deposits and collection remain.
- **Fast perfect strength training:** quickly finish push-ups, sit-ups, and squats with perfect results while keeping the original training allowance.
- **Fast SSS Wyvern Ride:** after the opening, finish at the selected difficulty's SSS score threshold and continue through the original result and reward flow. Keep difficulty unlocks and reward eligibility unchanged.
- **Cooking choices:** select a recruited chef and a native outcome in Plugin Settings. Preview its grade and basic stat modifiers in the help text. Shorten the preparation animation, cook multiple times between battles, and select food without the usual stock restriction. Default chef/outcome choices retain the original rotation and roll.
- **Inventory-free ordinary gifts:** give normal gifts even at zero stock without consuming owned copies. Keep the original recipient preferences and support gains.
- **Collect exploration rewards:** use the first entry in the original exploration menu to collect generated ground items, farm products, eligible animals, and available ordinary villager gifts in the Somniel or after battle. Keep native donation bonuses, adoption requirements and reward settlement. After dismissing the results, return to the same menu and selection.

Preferences below can be changed independently in Plugin Settings. Menu labels follow the game's text language.

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

Open **Settings → Plugin Settings** in the game. All options appear directly
in one scrolling list, in the following order, without extra group menus.
Use the original left/right controls; changes save automatically.

| Option | Default | Choices |
| --- | --- | --- |
| EXP Multiplier | 1× | 1× / 2× / 3× / 5× / 10× / Max Level |
| SP Multiplier | 1× | 1× / 2× / 3× / 5× / 10× |
| Battle Save | On | On / Off |
| Personal Outfits | On | On / Off |
| Automatic Supports | On | On / Off |
| Draconic Time Crystal Uses | Nondecreasing | Nondecreasing / 1–1000 |
| Standard Training Uses | 10 | Nondecreasing / 1–1000 |
| Nondecreasing Fishing Uses | On | On / Off |
| Batch Ring Count | 300 | 2–1000 |
| Unlimited Shop Stock | On | On / Off |
| Gifts Without Consumption | On | On / Off |
| Fast Fishing | On | On / Off |
| Fast Perfect Strength Training | On | On / Off |
| Fast SSS Wyvern Ride | On | On / Off |
| Repeat Cooking | On | On / Off |
| Chef | Native | Native / recruited chef |
| Cooking Outcome | Native | Native / chef's native outcome branches |

Turning a feature off restores its original behavior on subsequent actions;
it does not undo previously earned items, scores or progress. Reopen shop/activity
menus to refresh availability after a change. Personal outfits refresh when
models are rebuilt. Other conveniences, such as batch melding, all-ring cleaning,
collection and shortened relationship dialogues, keep their existing behavior.

EXP and SP choices apply to the next native EXP/SP reward, including ordinary
arena training. The settings do not multiply each other: 20 EXP and 10 SP at
2× EXP / 3× SP award 40 EXP and 30 SP. Max Level requires a positive EXP
reward; it does not grant levels on zero-EXP actions or change SP on its own.
Normal playback and manual skipping retain the same rewards; no animation is
automatically skipped by these options.

Rewind and standard training offer **Nondecreasing** or **1–1000** uses.
Finite values are total quotas, not remaining-use refills. Native spent counters
and reset timing remain in place. Nondecreasing training preserves the current
spent counter and keeps normal training available even if it was exhausted.

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
- [DeathChaos / Battle Outfits and More Outfits](https://gamebanana.com/mods/424968) — appearance-system research reference; no BOAMO files are redistributed.
- [FireEmblemEngageData](https://github.com/Xzonn/FireEmblemEngageData) — original outfit mappings and official game terminology references.

Fire Emblem and related names are trademarks of Nintendo and Intelligent Systems.
This unofficial fan project is not affiliated with or endorsed by them.

## License

[MIT](LICENSE) License © [jinghaihan](https://github.com/jinghaihan)
