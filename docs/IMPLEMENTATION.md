# Native implementation boundaries

Target: Engage 2.0.0, Build ID `8C08B9719E085F91847B5E0F935D9488`, Cobalt 1.31.0. Addresses below are relative to the main module.

## Initialization

One entry point validates the display version, mapped segment layout, loaded Build ID, draw/meld native functions, semantic call sites, and every activity patch's surrounding instructions before changing any game code.

Activities use 24 one-time instruction replacements. All original words must match before the first write. Each write is read back; a failure triggers restoration of startup writes and no hooks are registered. The memory writer imports only Cobalt's `sky_memcpy` ABI, not Skyline's NRO-loader APIs.

Draw, meld, and Wyvern Ride hooks are installed only after this preflight and patch application succeed. The ring modules have separate thread-local sessions and uniquely named exported hook symbols. No repeating timer or background inventory writer is used.

## Ring transactions and presentation

Creation retains its existing count configuration, RNG preview on a copy, native per-ring transaction, capacity checks, native achievement pass, and guarded presentation bypass.

Melding is ported from former `fee-batch-ring-meld` v0.2.0 (`d92aed4`). Costs remain single-transaction costs in the native Result object; only confirmation text shows totals. Each transaction rechecks free materials, equipped copies, output capacity, fragments, and actual inventory deltas.

The normal meld flow remains:

1. Show one localized batch confirmation.
2. Run `GodRoomPedestalSequence.ExecuteRingMerge` (`0x0233AE10`), including the original post-transaction call at `0x0233AF00`.
3. For this confirmed batch only, bypass `CreateRingMergeDemoSequence` (`0x0233AF20`) once.
4. Continue the original outer resource-unload and menu flow.

The presentation bypass happens **before** header/footer hiding (`0x0233B014`, `0x0233B064`), background unbinding, and creation of the child `RingSynthesisSequence`. It does not press a synthetic + button or repeatedly skip frames. A cancelled, stale, unrelated, or ordinary non-batch transaction does not inherit the bypass.

## Migrated activity effects

| Effect | Patched addresses | Preserved boundary |
| --- | --- | --- |
| Repeat cooking | `0x02D8158C` | Original cooking sequence and settlement |
| Food-list stock check | `0x01E546B4` | Local food-selection path, not global inventory |
| Fast bite | `0x026F2624`, `0x026F26BC`, `0x026F275C` | Native fishing selection and result |
| Easy landing | `0x026F2964`, `0x026F29D0`, `0x026F4714` | Native catch/reward flow |
| Fast perfect exercises | Six push-up, three sit-up, three squat sites in `src/activities.rs` | Native selected exercise/difficulty and activity allowance |
| Fast cleaning | `0x0241F920` in `RingCleaningSequence.DecreaseDirty` | Native dirty setter, completion voice, and outer cleaning settlement |

The previous `Unlimited Cooking` cheat bundled two separate changes. Both are retained at the user's explicit request. Static interface cross-checking places the second in the food-selection area associated with Sommie; it is not a global material-consumption patch.

## Fast SSS Wyvern Ride

One hook wraps `DragonRideGameSequence.Main` (`0x02ACA740`). It waits for the
camera's opening-complete flag, excludes event and test modes, and leaves
paused, retiring, already-finished, missing-data, or invalid-config runs alone.

The selected level at sequence `+0xB8` selects the live config's Normal, Hard,
or Expert score-border array at `+0x20`, `+0x28`, or `+0x30`. Its first descending
boundary is validated by the original `CalcRankNum` (`0x02ACC570`): the boundary
must produce SSS (1), and one point less must not. No numerical SSS score is
hardcoded or derived from the user's saved records.

For an eligible round, native `DragonRideCamera.FinishRail` (`0x01D02670`)
signals rail completion. The original Main still executes once and follows
its existing rail-finished branch into `ProcInst.End` (`0x0281D9A0`). End only
advances the current descriptor; it does not synchronously settle rewards.
After that final tick, `HubUtil.set_DragonRideTotalScore` (`0x02A649B0`)
assigns exactly the SSS threshold, unless retirement was requested. Writing
after the tick prevents pending target effects from inflating the score.

The original FinishMain, result scene, rank/high-score persistence, Sommie
settlement, play counter, reward eligibility/list, and resource cleanup remain.
The hook does not manufacture hit counts, append items, modify saved records
directly, unlock difficulties, or reset prize flags. It keeps no per-round
cached state and introduces no configuration, UI text, or runtime dependency.

## Inventory-free ordinary gifts

The published Full Gift Available reference bypasses the stock guards at `0x02524E6C` (menu construction) and `0x025245A8` (confirmation). The ordinary-item kind comparison at `0x02524E54` remains unchanged.

The plugin additionally suppresses only the gift-specific `ItemData.AddInventory(-1)` call at `0x025245F8`. Thus zero-stock gifts do not cause negative-stock attempts, and owned gift counts do not decrease.

The original recipient/support operation at `0x025245E4` remains. The plugin does not change support preferences, global `ItemData.GetInventory`/`AddInventory`, shops, battle items, or special relationship/plot prerequisites. No free gift copies or custom IDs are inserted into saves.

References: [KaitoB's publication](https://tekqart.com/thread-327061-1-1.html), [public 2.0.0 code listing](https://gbatemp.net/threads/fire-emblem-engage-save-editor.624990/page-4), and the [generated native interfaces](https://github.com/DivineDragonFanClub/engage-il2cpp/tree/0.1.0). Every replacement is independently checked against the supplied executable.
