# Native implementation boundaries

## Configurable battle conveniences and personal outfits

Settings remain in the independent SD file. Existing files retain chef, outcome,
draw count and support choices; missing new keys take defaults. Zero is the
nondecreasing selector for rewind and standard training, never zero available uses.
Seventeen Cobalt callbacks register the options in one flat list: EXP/SP
multipliers, general preferences, counts, stock, minigames and cooking. There are no category headers
or extra submenus. Cobalt's original PluginSubmenu adds each returned item to the
native ConfigMenu fullMenuItemList; it owns the menu's lifecycle and scrolling.
This plugin does not change ConfigMenu's class, visible row count, prefab rows,
viewport or scroll geometry. Private item classes preserve ConfigBasicMenuItem
InitContent and native selection callbacks through Cobalt's second class copy.

MapHistory.Rewind.GetMaxUseCount at 0x02707DD0 supplies the finite quota or the
native negative-unlimited sentinel. The original IsEnable still checks the game's
unlock flag. Nondecreasing mode replaces only the use increment at 0x0270DB44;
finite mode restores it. History recording, rollback and chapter rules are intact.
Battle saves reversibly replace the three original sequence checks at 0x02677284,
0x01E40CF8 and 0x01E40E88, without changing mode, difficulty or saved data.

The standard-training parameter is identified by its interned metadata key at
0x06295CA8. Unrelated Parameter.Get reads call the original with no string scan or
allocation. A finite selection returns the total quota; the native dictionary
counter G_拠点_闘技場済み and reset paths remain unchanged. Nondecreasing mode keeps
the counter's previous value at 0x01CAA7A4 and returns spent + native quota for
availability/UI, so exhausted training becomes available without erasing history
or displaying an enormous fake remaining count. Paid training is not changed.
Code writes occur only at startup or explicit settings edits, not per frame.
Settings write failures restore the previous runtime words and active values.

Seven additional booleans default to true: fishing_unlimited, shop_stock,
gift_stock, repeat_cooking, fast_fishing, fast_training and fast_wyvern.
Missing keys retain defaults, explicit zeroes persist disabled features.
Configuration loads before the combined activity/preference patch transaction;
there is no temporary default-on patch pass before reading saved choices.
Settings edits skip unchanged words and roll back the actual previous words
on failure. Shop/Wyvern hooks directly call their originals when disabled.

FishingGameSequence.DecreasePlayCount at 0x026FFF90 is the only suppressed use
settlement; IncreasePlayCounter, fish records and reward paths remain native.
Availability reads G_Fishing_PlayCount through GameVariable.GetNumber. The old
FishingSequence.GetFishingPlayCount at 0x0260B190 returns the rod's maximum,
not remaining uses. With nondecreasing mode enabled, Somniel initialization or
an off-to-on settings transition supplies one native remaining use only if
already exhausted, through GameVariable.SetNumber at 0x0251F290. This changes
the normal use counter, not unlocks or custom save settings. No frame polling
or global variable hook is added. Fast fishing remains independent.

Personal outfits integrate the former independent MIT-licensed implementation:
42 exact PID/dress-gender mappings cover 41 playable identities, both Alear genders
and DLC allies. The three appearance hooks retain original scene scoping and only
replace transient dress references on compatible rigs. The off path reads a
boolean and returns without catalog/string processing. No outfit inventory, job,
saved appearance ID, hair, resource asset or game-save setting is introduced.
Do not load the former standalone outfit NRO together with this implementation.
Models already instantiated refresh on the next native rebuild.

The menu uses the game's Simplified Chinese nouns 龙之时水晶, 一般锻炼,
牵绊戒指 and 支援对话; Chef is the user-requested concise 料理厨师 label.

## EXP and SP settlement

The first two settings independently select 1/2/3/5/10; both default to one.
EXP additionally accepts zero as the Max Level selector (UI: 满级), not zero EXP.
The independent file stores exp_multiplier and sp_multiplier only when they
differ from one. Existing configurations therefore retain native rewards.

ExpSequence.CreateBind at 0x024E8C20 receives separately calculated EXP and SP.
Scale each once, after the native calculation and SP eligibility checks.
Both map ItemUseSequence and InventorySubMenu item confirmation also reach
this entry. Walk the actual native ProcInst parent chain and leave those item
reward contexts unchanged, including SP books. Do not infer item use from zero
EXP: a capped combat unit can still legitimately receive SP.
The original SP conversion is not recalculated from amplified EXP. Keep zero
rewards zero, and limit amplified rewards to the current job's level cap and
9999 SP. Max Level replaces a positive EXP reward with the remaining EXP to
the current class cap, without changing SP or triggering on zero-EXP actions.
Both-one settings call the original with unchanged arguments.

Native Unit.AddExp at 0x01A39D40 increments at most one level per call, even
for an amount above 100. Normal ExpSequence.Tick awards individual points,
but CreateBind's GameSkip branch and Tick's fast-forward branch pass a whole
remaining reward in one call. Scope the original CreateBind and large-reward
Tick to their unit on the menu/game thread; the AddExp hook splits only these
scoped large amounts into calls of at most 100. It never multiplies again.
Unrelated AddExp calls and the global AddSkillPoint function are unchanged.
Native LevelUpSequnece.Prepare (0x01BE1260) copies the post-EXP unit, restores
its old level in the preview and calls Unit.LevelUp(2) only once. For an enabled
EXP multiplier/Max Level, finish the remaining native LevelUp(2) calls on that
same preview until it reaches the real unit's earned level, before native
Reflect copies it back. Do not grow the real unit twice or bypass native growth
RNG, fixed-growth accumulation, skill handling or caps. Default 1x and reclass
previews retain their original behavior.
Do not select skip mode, synthesize input or bypass the native sequences.
Existing growth, history, displays and cleanup retain the original outer flow.

This is event-driven; there is no battle-frame polling, settings allocation
per point, custom experience bar or save-variable configuration.

## Facility, support and cooking extensions

The outdoor HubMenu receives one facility entry, alongside the existing collection
entry. It opens three categories: Facilities (five destinations), Activities
(seven), and Shops (five). Categories are not additional outdoor entries; no list
needs custom scrolling, a spare-row calculation or expanded prefab nodes.
Categories use the original HubMenu.SubSystemMenu construction pattern:
SortieSubMenuContent.Create (`0x024EB420`), BasicMenu construction and binding,
BindParentMenu, row count, and SetTransformAsSubMenu (`0x0245E330`) with the
actual selected parent item. This panel and the outdoor menu remain visible.
Destinations instead use native HubMenuContent.Create (`0x028B78C0`) with
no submenu transform. Both ancestors are hidden while this full-page list is
open, but remain bound and suspended with their selected rows intact.
The previous third-column OnBuild override, RectTransform measurements,
coordinate conversions and UpdatePos calls have been removed. Native content
owns page layout; there is no custom scrolling or positional adjustment. Its
BasicMenu and item classes are private copies, not shared-vtable mutations.
Hub entries instantiate native HubMenuItem, retaining its selection/help
visibility callbacks and 30-slot map interface, rather than stripping it to
MapBasicMenuItem or its 27-slot BasicMenuItem parent. Ordinary well entries need only BasicMenuItem and
do not register a nonexistent help slot. Each uses its own native constructor.
The factory resolves the actual inheritance and checks only requested slots
even when a private class was cached by an earlier menu. Native facility
CreateBind entry points keep their full descriptors and OnDispose. The wrapper
runs original cleanup before restoring the destination page and footer after a
facility scene; the outdoor and category panels stay hidden. The destination
page's private OnDispose similarly runs native cleanup, then restores those two
ancestors on B. Category B retains the original disposal/resume and removes only
the side panel, without reopening the already-visible outdoor menu.
Visibility changes use CloseAnime (slot 22) and OpenAnime (slot 20), never
OpenAnimeAll. Native binding, suspension and input ownership remain unchanged.
Category and destination B callbacks return CloseThis | SeCancel (513), not
DeleteAll (32). Each B press closes only the current submenu. Hub entry,
category, destination and collection help callbacks share an empty-text callback
rather than repeating their names or borrowing unrelated text beside the selected
row; native selection/deselection stays intact.
Only Hub_Solanel sequence 4 is accepted. Remote ring rubbing is hidden because
it requires the original ring-room scene; normal ring cleaning is unchanged.
Activity shortcuts bind the complete original facility sequences, including
resource loading, actors and scene setup. They do not use reference-mod fake
fish, fabricated results, extra allowances or hand-written arena settlement.
Fishing, strength training and Wyvern Ride also check their remaining allowance
before dispatch. Private sequence classes may have fewer virtual slots than a
BasicMenuItem; callback validation uses the requested slot, not a menu-only minimum.

After HubSequence.Init, the first idle Main tick in sequence 4 checks ready
ordinary supports. Loading, script children and interactions take precedence;
the check runs once per visit and resets even when a managed address is reused.
HubMenu opening also checks newly ready pairs. Both entry points bind the complete native
MyRoomRelianceSequence as a child. Private KeyCall overrides on its unit/pair
menus return native ACall/BCall results to TickInput; they do not replace the
dialogue sequence. Native IsLevelUpOnly/IsGodReliance gates, IsTalk,
UnitReliance.TryGet and CanLevelUp must permit an ordinary C/B/A pair.
One unordered pair is attempted once per owning sequence. Messages use the
existing narrow skip branch. Lua, rank advancement, completion callbacks and
cleanup remain native. No rank/EXP/viewed write API is used by the dispatcher.
Gallery and Pact Ring calls are excluded. Temporary menu mode flags are restored
on disposal, independently of settings.
Post-battle exploration (sequence 5) never starts automation: the original
support item's ACall rejects sequences 2, 3 and 5 with the location-restriction
message (`0x020994E0`, `0x0209950C`, `0x02099538`). The plugin retains those
native rules. Outside sequence 4, private menu KeyCall wrappers return to the
original input handler rather than sending automatic A/B actions.
Only menu-origin dispatch reopens a BasicMenu; scene-origin dispatch must not
call menu virtual slots on HubSequence. Original OnDispose runs in both cases.
Idle support ticks read only the boolean preference, not a cloned Settings/chef
string. The already-checked visit gate precedes preference access. Dialogue
skipping checks Main before inspecting ancestors and caches only process-lifetime
IL2CPP class metadata; live dialogue pointers and eligibility are never cached.

Cooking preferences use stable PID strings and a bounded native outcome branch
(0 = native; 1..3 = chef specialties; 4 = ordinary; 5 = native bad-result roll).
Only joined units with CookData are offered. CurrentCookingPid and GetCookData
are read overrides; Cook receives the selected existing Unit without changing
the rotation's stored PID. A thread-local Cook owner scopes DetermineTaste to
that transaction and keeps conditional variation/RNG native. Outside Cook,
DetermineTaste is untouched. Outcome names come from TasteData/Mess without
running a roll. Each independent settings edit preserves the other options;
legacy count-only files remain valid.

Outcome help reads the native grade, signed recipe/other-stat modifiers and
CapabilitySbyte array. Grade/stat names use game text; conditional templates are
marked rather than presented as a guaranteed recipient-specific result. No
Dish constructor, reward API or RNG is invoked for a preview.

The preparation shortcut hooks DoCooking's iterator factory, not ExecuteCooking
or the outer sequence. Only a fresh iterator with its current owner and a settled
dish is set to its native completed state (-1), without a MoveNext-address check.
That MoveNext only starts/waits for cooking motions and a fixed delay.
Already-started iterators fall back. Result/support/lunch/achievement and resource
cleanup descriptors remain original; the rest of the meal scene is not bypassed.

## Unlimited shop stock

Closed generic MethodInfo allowlists cover WeaponShopData, ItemShopData and
FleaMarketData. Stock reads return the game's infinity sentinel (-1), IsInfinity
returns true and PurchaseImpl skips only saved stock deduction. Native callers
still charge prices and grant goods with their original capacity checks.

Content-list generation performs its own saved-stock dictionary lookup. Intercept
only that BL site at `0x031FD608`, not the shared dictionary function. The local
bridge passes the enclosing MethodInfo (`x19 - 0x18`) to an allowlisted read
override, calls native TryGetValue once and replaces only its returned integer.
This includes sold-out goods without bypassing chapter/additional-stock checks.
Other shop types, clothing ownership, keys and saved values stay native.
FleaMarketData.SetupContentList loads its shared-body MethodInfo through
`0x062B8228`, not the adjacent RegistImpl slot `0x062B8220`. Its native GOT load
and argument load are verified offline, and compiled fixtures seed the concrete
wrapper metadata independently of the plugin allowlists.

The bridge uses Cobalt's existing A64HookFunction export. It replays the four
post-call instructions, preserving AAPCS64 callee-saved registers and the native
comparison flags, then resumes at `0x031FD61C`. Both the call and continuation
windows are verified offline against the target NSO. No general dictionary, list-entry or stock-key hooks,
thread-local stock context, new loader imports, code caves or polling remain.
Battle dictionary reads no longer dispatch into this plugin. This removes that
specific overhead; it is not an in-game frame-time measurement.

Target: Engage 2.0.0, Build ID `8C08B9719E085F91847B5E0F935D9488`, Cobalt 1.31.0. Addresses below are relative to the main module.

## Initialization

Ancient Well menu extensions use a private BasicMenuItem class per action, shared
factory code with collection, and native `WellTopMenu.CreateBind` (`0x0293C6A0`).
Original choices and their indices remain before two appended entries. Both
entries require a native pending/ready exchange (flags 1/2) and level 1–5.
Callbacks recheck state and require an idle menu with its original parent.
There are no extra hotkeys or replacement facility scenes.

Preview allocates native Random (`0x06287BF8`), seeds it with the stored exchange
seed (`0x02374F10`) and calls `CalcItemExchange` (`0x0293C890`) with explicit
sequence, level and private RNG arguments. This ABI and argument order were
checked against the original GetItem call site (`0x0293CC20`). Duplicate ItemData
references are counted, translated through Mess, and displayed with native
GameMessage.CreateKeyWait. No grant, acquired-record or use-flag method is called.
Only explicit reroll draws once from the system RNG and replaces the native
exchange seed; failed calculation/display restores the previous seed. Preview
does not advance any live RNG. Deposits, exchange level, waiting and native
collection remain unchanged. This is a normal native seed change, not a custom
mod-settings variable in the save.

One entry point validates the display version, mapped segment layout, loaded Build ID, draw/meld native functions, semantic call sites, and every activity patch's surrounding instructions before changing any game code.

Activities have 32 instruction sites without runtime original-word matching. Settings choose the enabled or original word at startup and explicit edits; there are no per-frame writes. Each write is read back; an actual write failure restores the words observed before this operation. It does not disable unrelated hooks or settings. The memory writer imports only Cobalt's `sky_memcpy` ABI, not Skyline's NRO-loader APIs.

Draw, meld, Wyvern Ride, collection and settings initialize independently of activity-write success. The ring modules have separate thread-local sessions and uniquely named exported hook symbols. No repeating timer or background inventory writer is used.

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
boundary is used directly. The plugin does not impose another array-length,
sort-order or `CalcRankNum` validation gate. No numerical SSS score is hardcoded
or derived from the user's saved records.

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
Its four callbacks live in one process-lifetime private copy of MapBasicMenuItem's
vtable; original classes and MethodInfo structures are not modified. Labels
come from the game's existing message table. The entry is disabled when nothing
is pending, and dispatch rechecks the live source list. There is no automatic
entry trigger, polling, configuration, or hotkey.

The current scene's native HubAccessManager is selected through HubSequence.
Only sequence 4 (Somniel) or 5 (post-battle exploration) can collect. Existing
generated sources are the sole input; ungenerated IDs, claimed
flags, missing data and unrelated interaction types are ignored. Types 6 and 9
use `HubUtil.GetItemCountWithBonus` (`0x02A629C0`); type 8 already contains its
native quantity and does not receive a second donation multiplier. Farm drops
are collected through these same generated pickup nodes, not by generating
another copy from pasture variables.

Post-battle animals require the original barn-complete flag and
`HubUtil.IsCaptureAnimal` (`0x02A622C0`). Capture uses the native increment
(`0x02A62640`) without adding a plugin-side species budget. Native capture
settlement owns the count limit. Existing Somniel pasture animals are
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

Claim identities are deduplicated so one generated source is not granted twice.
There are no plugin-side item/species, convoy, money or fragment capacity budgets
and no extra per-dispatch quantity cap. Native reward settlement owns its limits.
DisabledTalk at HubDisposData+0xB5 only gates NPC gifts, not ground pickups.

`CommonRewardSequence.CreateBindForWell` (`0x02533950`) receives the selected
ItemData list and the original TitleMID key once. The complete native sequence
loads/waits for resources, grants items, displays results, handles overflow and
unloads. Collection does not call CalcRewardItemList or CommonRewardDialog
directly. A fresh exploration visit need not have the message-content prefab
already loaded. The outdoor menu is retained rather than closed: after binding,
its panel hides with CloseAnime, and the reward sequence's private OnDispose
runs original cleanup before reopening that same menu and restoring the footer.
The action returns only the decide sound (128), without CloseThis or DeleteAll.
The pending native child prevents a second collection before results complete.
Adoption-only batches leave the menu visible without an empty result dialog.
Menu names/help retain the reference's original keys;
no rename or new localization table is introduced.
Only settled pickup/animal nodes have their loaded markers/effects disabled;
unrelated NPCs and unselected sources are left alone.

The reference behavior was researched in
[Triabolical's collection menu](https://github.com/triabolicals/triabolical-code-mods).
No source was copied: the repository supplied no source license granting reuse.
Native layouts were checked against generated interfaces, the
[Unity IL2CPP ABI](https://github.com/DivineDragonFanClub/unity-nx/tree/0.1.2/src/il2cpp),
and the read-only supplied Engage executable. The compiled fixture models these
boundaries, not Unity's actual dialogue, animation, minimap or save/reload flow.

## Arena-training presentation

`ArenaCombatSequence.CreateBind` (`0x01CA3000`) constructs and binds a fresh
14-entry descriptor array without ticking its first step. For ordinary training
(type 0, non-Emblem, zero bond EXP) or paid, non-special Emblem training (type 1,
positive bond EXP), reorder that instance's array to retain the native fades,
Grow1, Grow2, Grow3 and End. Only the accessed descriptor indices must exist;
there is no original-function, descriptor-type or mirrored-object equality gate.
Use managed write barriers; fill the unused part of the original 14 entries
with End and leave any additional entries untouched. FadeWait is a native user descriptor
(type 15), not a WaitWhileTrue descriptor.

This removes child combat setup, fight/result presentation and the matching
child rendering teardown. Parent calculation, equipment setup/restoration,
fragment charging, growth, bond-conversation dispatch, achievements and final
cleanup remain native. Ordinary training retains Grow1/2/3 and its original
parent FinishTraining, including EXP, support settlement and allowance use.
Special challenges and inconsistent mode/EXP combinations are not changed.

[Triabolical's arena implementation](https://github.com/triabolicals/triabolical-code-mods)
provided a behavior reference. No source was copied. Unlike its global
coroutine patches, this implementation edits only the child array returned by
the original factory.
The compiled fixture checks the edit and fallbacks, not Unity execution.

### Relationship conversation messages

Hook only `App.Talk3D.TalkSequence.Tick` (`0x020C79B0`). Follow the native parent
chain to identify the initialized IL2CPP class for
MyRoomRelianceCallSequence. Accept a native C/B/A support target (levels 1–3,
no God target), or a God target with no ordinary target. In an arena child,
additionally require the native paid/non-special Emblem training mode and EXP.
Do not recheck mirrored parent/child identity or duplicate mode fields.
This includes manually selected/replayed relationship
events outside training; it does not start or complete unselected supports.
Pact Ring/A+ events, story talks and malformed/missing targets use original
Tick. No cached session or global skip/input flag is introduced.

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

Cobalt 1.31.0's `cobapi_register_configmenuitem_cb` registers all seventeen
rows directly in Plugin Settings. Their class lookup uses Cobalt's exported IL2CPP
forwarders. All four added imports were checked in the pinned release's
`subsdk9` (SHA-256 recorded in `tools/cobalt-imports.json`). No new dependency,
game-setting hook, custom save key or global class/vtable edit is required.

The callback constructs ConfigBasicMenuItem using the native constructor and
gives it a process-lifetime private class. Only BuildAttribute (8), ACall (18)
and CustomCall (26) are replaced; command/help text use the original fields.
Only the accessed slots must exist, with no arbitrary instance-size or maximum
vtable-size requirement. Cobalt's Assembly::Load export returns an assembly;
its image field is passed to class lookup. Distinct static class names identify
all seventeen options after Cobalt clones their classes again.
Native `ChangeKeyValue(int,int,int,int)` handles the original
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

## Remaining checks after runtime restriction cleanup

- Target detection retains the supported game version, module layout and Build ID;
  offline instruction verification is not a runtime hook-registration gate.
- Native access retains null pointers, negative lengths, actual array capacities
  and the specific virtual slots or descriptor indices used by a feature. It
  imposes no maximum object/list/vtable size or parent-chain depth.
- Feature scope retains Somniel/exploration context, generated/pending claim
  state, ordinary NPC gift counters, native animal eligibility, support readiness,
  non-story/non-Pact dialogue, and the native activity lifecycle. Deduplication
  prevents the same pending claim or shared ring from being processed twice.
- Ring transactions retain prices, resources, equipped-copy rules, output capacity,
  rank/cost compatibility, native pool shape needed by RNG prediction, initial
  batch budgets and actual settlement deltas. These are the requested transaction
  rules, not extra collection inventory budgets.
- Independent settings retain their existing count range (2–1000), valid option
  values, PID syntax/length (128 bytes), file read limit (4096 bytes) and actual
  storage-error handling. None of these disable hook/settings registration.
- Actual failed activity writes restore previously observed words. Storage errors
  retain the old preference. Neither condition disables unrelated features.
