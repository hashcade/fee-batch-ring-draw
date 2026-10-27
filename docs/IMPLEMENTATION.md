# Native implementation boundaries

Target: Engage 2.0.0, Build ID `8C08B9719E085F91847B5E0F935D9488`, Cobalt 1.31.0. Addresses below are relative to the main module.

## Initialization

The native unit-status SetLevel hook (`0x01C66980`) runs the complete original
setter first, then replaces only its level title/value through TrySetText
(`0x0290F0A0`). The localized title distinguishes current/total. Current display
uses GetEnhancedLevel; total uses the unenhanced unit's stored level (`+0x89`)
plus signed internal level (`+0x158`). Negative internal levels and invalid
inputs retain native text. This does not write unit data, EXP, arrows or colors.

One entry point validates the display version, mapped segment layout, loaded Build ID, draw/meld native functions, semantic call sites, and every activity patch's surrounding instructions before changing any game code.

Activities use 32 one-time instruction replacements. All original words must match before the first write. Each write is read back; a failure triggers restoration of startup writes and no hooks are registered. The memory writer imports only Cobalt's `sky_memcpy` ABI, not Skyline's NRO-loader APIs.

Draw, meld, Wyvern Ride, and collection hooks are installed only after this preflight and patch application succeed. The ring modules have separate thread-local sessions and uniquely named exported hook symbols. No repeating timer or background inventory writer is used.

## Ring transactions and presentation

Creation retains its existing count configuration, RNG preview on a copy, native per-ring transaction, capacity checks, native achievement pass, and guarded presentation bypass. The original CreateBind (`0x02956D00`) constructs the one/ten menu unchanged; a third native count item is appended with the same decision handler and native constructor. A configuration of ten adds nothing. Only the configured extra count can arm batch settlement and presentation bypass, never the original one or ten entries.

Melding is ported from former `fee-batch-ring-meld` v0.2.0 (`d92aed4`). Costs remain single-transaction costs in the native Result object; only confirmation text shows totals. Each transaction rechecks free materials, equipped copies, output capacity, fragments, and actual inventory deltas.

The normal meld flow remains:

1. Show one localized confirmation with single, batch, and original cancel entries. Single selection clears the batch plan and preserves native presentation. Batch selection alone confirms the displayed plan. If only one copy is possible or custom choices cannot be built, retain the original single confirmation.
2. Run `GodRoomPedestalSequence.ExecuteRingMerge` (`0x0233AE10`), including the original post-transaction call at `0x0233AF00`.
3. For this confirmed batch only, bypass `CreateRingMergeDemoSequence` (`0x0233AF20`) once.
4. Continue the original outer resource-unload and menu flow.

The presentation bypass happens **before** header/footer hiding (`0x0233B014`, `0x0233B064`), background unbinding, and creation of the child `RingSynthesisSequence`. It does not press a synthetic + button or repeatedly skip frames. A cancelled, stale, unrelated, or ordinary non-batch transaction does not inherit the bypass.

## Clean all at the original facility

`RingCleaningSelectSequence.CreateGodSelectMenu` (`0x0241A430`) first builds
the original eligible list. A private instance of its native `GodUnitMenuItem`
adds Polish All when a dirty entry exists, retaining the native selection and
decision delegates and their transition to the original cleaner selection.
The content builder calls `GetName` non-virtually, so a narrowly scoped name
hook recognizes only the private class. Original items keep their names.

The once-only plan is tied to that selection sequence, excludes clean entries,
and deduplicates by the underlying GodUnit rather than DLC portraits sharing
the same ring. Cancelling, rebuilding the ring selection or ending the sequence
discards the plan. Cleaner choice and static cleaner assignment stay native.

For this plan only, `RingCleaningSequence.CreateBind` (`0x0241AF90`) settles on
the menu thread instead of constructing the per-ring 3D rubbing child. Each
entry rechecks dirt, its native state and the unavailable flag. The native dirty
setter must read back zero before native `AddBondExp(true, Weak)` is invoked;
that helper applies its normal three-point finish reward, HasBond and level-up
guards to the selected cleaner. No reward is issued for already-clean, duplicate,
unavailable or failed-setter entries. Static GodUnit/type selection is restored
afterward. The original outer StartRingCleaning/EndRingCleaning header, footer
and return-label path is not bypassed; normal individual cleaning is unchanged.

The helper is verified not to dereference a cleaning controller receiver; it
does use static selection and native bond-level notification code. Unity menu,
notification and outer-state-machine behavior still require in-game checks.

## Sommie and existing activity effects

Sommie's original EatFood coroutine has five fixed reaction/telop waits shortened.
Its original food choice, conditional consumption, food preference branches,
ExitEatFood, GetBond and repeat-reward guards remain untouched. The original
Strok coroutine retains its initial animation/petting interval, then enters its
own finish-input branch at `0x01E5520C`, including motion stop and normal exit.
Only two following fixed waits are shortened. No new interaction menu, synthetic
global input, reward call, friendship overwrite or eligibility reset is added.
Actual animation events and affection settlement still require in-game checks.

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

## Exploration collection

One hook appends an independently implemented item to `HubMenu.CreateBind`
(`0x028B6EF0`) and moves it to the first position with managed write barriers.
Its four callbacks live in one process-lifetime private copy of BasicMenuItem's
vtable; original classes and MethodInfo structures are not modified. Labels
come from the game's existing message table. The entry is hidden when nothing
eligible fits, and dispatch rechecks the live source list. There is no automatic
entry trigger, polling, configuration, or hotkey.

The current scene's native HubAccessManager is selected through HubSequence.
Only sequence 4 (Somniel) or 5 (post-battle exploration) can collect. Existing
generated sources are the sole input; ungenerated IDs, disabled nodes, claimed
flags, missing data and unrelated interaction types are ignored. Types 6 and 9
use `HubUtil.GetItemCountWithBonus` (`0x02A629C0`); type 8 already contains its
native quantity and does not receive a second donation multiplier. Farm drops
are collected through these same generated pickup nodes, not by generating
another copy from pasture variables.

Post-battle animals require the original barn-complete flag and
`HubUtil.IsCaptureAnimal` (`0x02A622C0`). Capture uses the native increment
(`0x02A62640`) and verifies a one-copy increase before claiming the source.
The native setter clamps each species at 99; shared reservations exclude
animals that would exceed this bound. Existing Somniel pasture animals are
never adopted again.

Ordinary villager gifts must contain a current TalkItem and pass the same
result-talk-index/access-count comparison as `HubPlayTalk.ExecEventAfter`
(`0x023DC35C`–`0x023DC374`). Story, support, Emblem, party and animal dialogue
sources are excluded. After granting, only that conversation's existing native
access-count variable advances by one, and its cached TalkItem is cleared.
This consumes that ordinary gift conversation; other dialogue and recruitment
flags are not bulk-completed. Rebuilding the access data cannot make the old
talk index eligible again. Pickup done flags are not used as a substitute for
the NPC conversation counter.

The host-tested reservation planner deduplicates claim identities and shares
budgets for each quantity item/species, convoy slots, money and fragments.
Each source fits in full or remains pending. Native quantity limits and the
999-slot transporter layout are read before granting. The managed ItemData
list is bounded to 8,192 copies per dispatch; excess sources remain pending.

`CommonRewardSequence.CalcRewardItemList` (`0x025343B0`) synchronously grants
the selected ItemData list using native inventory/transport/resource calls.
After settlement, `CommonRewardDialog.CreateBind` (`0x0252FF50`) displays its
returned dictionary once. It does not grant again. Adoption-only batches use
the original decide sound and menu close, without an empty result dialog.
Only settled pickup/animal nodes have their loaded markers/effects disabled;
unrelated NPCs and sources that did not fit are left alone.

The reference behavior was researched in
[Triabolical's collection menu](https://github.com/triabolicals/triabolical-code-mods).
No source was copied: the repository supplied no source license granting reuse.
Native layouts were checked against generated interfaces, the
[Unity IL2CPP ABI](https://github.com/DivineDragonFanClub/unity-nx/tree/0.1.2/src/il2cpp),
and the read-only supplied Engage executable. The compiled fixture models these
boundaries, not Unity's actual dialogue, animation, minimap or save/reload flow.

## Paid Emblem-training presentation

`ArenaCombatSequence.CreateBind` (`0x01CA3000`) constructs and binds a fresh
14-entry descriptor array without ticking its first step. For paid, non-special
Emblem training only, validate the parent/child flags, calculator identities,
untouched descriptor index, all descriptor types and callback targets. Reorder
that instance's array to retain the native fades, Grow1, Grow2, Grow3 and End.
Use managed write barriers; leave the unused tail as End entries. Unknown or
already modified layouts stay unchanged. FadeWait is a native user descriptor
(type 15), not a WaitWhileTrue descriptor.

This removes child combat setup, fight/result presentation and the matching
child rendering teardown. Parent calculation, equipment setup/restoration,
fragment charging, growth, bond-conversation dispatch, achievements and final
cleanup remain native. No ordinary-training steps or allowances change.

[Triabolical's arena implementation](https://github.com/triabolicals/triabolical-code-mods)
provided a behavior reference. No source was copied. Unlike its global
coroutine patches, this implementation edits only a validated fresh child array.
The compiled fixture checks the edit and fallbacks, not Unity execution.

### Training bond conversations

Hook only `App.Talk3D.TalkSequence.Tick` (`0x020C79B0`). Follow at most eight
native parent links and require the exact initialized IL2CPP classes for
MyRoomRelianceCallSequence, ArenaCombatSequence and ArenaOrderSequence.
Require a God target, no ordinary support target, paid/non-special Emblem
training flags and the live parent/child identity. Other conversations use the
original Tick. No cached session or global skip/input flag is introduced.

For an eligible talk in Main (label 3), use native `ProcInst.Jump(int)`
(`0x0281E6F0`) to enter Skip (label 6), matching Tick's own GameSkip branch.
The original Skip (`0x020C85E0`) processes remaining message tags and returns
through the native close/finish path. Do not bypass MyRoomRelianceCallSequence,
its Lua event, entry/exit callbacks, level-up flow or reward handling; do not
write viewed flags or unlock future bond levels manually. Subsequent event
instructions still run normally. The compiled fixture proves scope and jump
dispatch only; actual conversation completion, viewed records and level caps
must be checked in-game.

## Independent in-game settings

Cobalt 1.31.0's `cobapi_register_configmenuitem_cb` registers the batch-count
row in Plugin Settings. Its class lookup uses Cobalt's exported IL2CPP
forwarders. All four added imports were checked in the pinned release's
`subsdk9` (SHA-256 recorded in `tools/cobalt-imports.json`). No new dependency,
game-setting hook, custom save key or global class/vtable edit is required.

The callback constructs ConfigBasicMenuItem using the native constructor,
validates its size and bounded vtable, and gives it a process-lifetime private
class. Only CustomCall, SetCommandText and SetHelpText are replaced, located
by metadata name. Native `ChangeKeyValue(int,int,int,int)` handles the original
left/right repeat. Labels follow the game's current text language. The count
is changed in memory only after its independent file has been committed.

The existing SD-root `fee-better-somniel.cfg` is now managed by the UI; old
single-key files remain readable. Reads are bounded and do not touch saves.
Writes create `.cfg.tmp`, flush and read it back, rotate the old file to
`.cfg.previous`, and rename the complete temporary file into place. A failed
commit restores the old name where possible; the reader can also recover the
previous file after interruption. Successful commits remove the recovery file;
there is no backup history. All handles and mounts are scoped and released.
Failures keep the active count unchanged and return the native error sound
with a localized help message. Unchanged input does not write the SD card.
