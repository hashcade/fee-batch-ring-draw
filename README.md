# FEE Better Somniel

Somniel quality-of-life improvements for **Fire Emblem Engage 2.0.0**, using **Cobalt 1.31.0**.

## Features

- **Bond Ring creation:** retain Create One and Create Ten, with a configurable batch defaulting to 300. Keep normal costs, probabilities and acquisition records; skip the batch result animation.
- **Bond Ring melding:** choose one copy or a batch of the selected kind and rank. Keep normal costs and equipped copies; confirmed batches skip their presentation.
- **Polishing:** finish manual individual polishing with one action, or automatically polish eligible dirty rings and bracelets on Somniel arrival and exploration-menu opening. The cleaner is configurable, defaulting to Alear. Keep native bond caps and completion records, without an extra row in the original polishing list.
- **Collection:** collect generated ground items, farm products, eligible animals and available ordinary villager gifts from the original exploration menu, in the Somniel or after battle. Keep donation/adoption requirements and return to the same menu after results.
- **Facilities:** one outdoor entry opens Facilities / Activities / Shops, with 6 / 6 / 5 destinations. Preserve original facility unlocks. Manual polishing is available only through the actual Ring Chamber, not its outdoor shortcut.
- **Relationships:** automatically complete ready ordinary C/B/A supports in the Somniel. Manually started support and Emblem bond conversations retain the original playback and skip controls. Story and Pact Ring events stay unchanged.
- **Standard training:** choose 1–1000 uses or nondecreasing uses, defaulting to 10. Training battles use their original presentation.
- **Shop stock:** independently enable unlimited unlocked armory, item-shop and flea-market stock. Keep prices and outfit/accessory ownership rules.
- **Ordinary gifts:** give normal gifts at zero stock without consuming owned copies.
- **Fishing:** independently enable faster fishing and nondecreasing fishing uses.
- **Minigames:** quickly finish muscle training with perfect results and Wyvern Ride at the current difficulty's SSS threshold, retaining normal records and rewards.
- **Cooking:** allow repeat cooking, select a recruited chef and a native outcome, and shorten preparation.
- **Sommie:** shorten feeding and petting, and bypass food-list stock restrictions while retaining native preferences and reward rules.

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

Open **Settings → Plugin Settings**. The fourteen preferences appear in one flat list, with native left/right controls and no extra footer descriptions.

| Option | Default | Choices |
| --- | --- | --- |
| Automatic Supports | On | On / Off |
| Auto Polish | On | On / Off |
| Polishing Character | Alear | Joined ally |
| Standard Training Uses | 10 | Nondecreasing / 1–1000 |
| Nondecreasing Fishing Uses | On | On / Off |
| Batch Ring Count | 300 | 2–1000; step 10 |
| Unlimited Shop Stock | On | On / Off |
| Gifts Without Consumption | On | On / Off |
| Fast Fishing | On | On / Off |
| Fast Perfect Strength Training | On | On / Off |
| Fast SSS Wyvern Ride | On | On / Off |
| Repeat Cooking | On | On / Off |
| Chef | Default | Default / recruited chef |
| Cooking Outcome | Default | Default / chef's outcome branches |

Preferences save automatically in `sdmc:/fee-better-somniel.cfg`, independently of game-save variables. Without a valid file, defaults apply. Reopen the affected menu after changing a preference. A batch count of 10 does not add a duplicate ten-ring choice.

Standard training changes the total quota, not the remaining counter; nondecreasing mode retains native spent counters. Fishing can restore one available use if the session was already exhausted. Other conveniences retain their existing behavior without additional switches.

Chef and Cooking Outcome default to the original rotation and roll. Recipe and recipient conditions remain unchanged. Turning automation off does not undo rewards or progress already obtained.

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
