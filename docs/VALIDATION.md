# Validation

## Completed locally

- Build the combined Switch NRO with the pinned Skyline dependency.
- Verify actual relocation-backed imports against Cobalt 1.31.0 / Engage 2.0.0. No unavailable `get_program_id`, NRO-load/unload hook, or plugin-address API is accepted.
- Read-only verification of the supplied NSO: Build ID, mapped segment layout, segment checksums, and **298** draw/meld/activity/Wyvern/collection/cleaning/arena/dialogue/settings/well/cooking/support/facility/shop instruction signatures.
- Focused meld planning/session/confirmation tests retain cost, capacity, equipped-copy, repeated-dispatch, and fragment-refill protections.
- Presentation-session tests cover matching owner, one-shot consumption, cancellation/cleanup, stale ownership, and null ownership.
- Activity tests cover all **32** instruction replacements, reapplication over existing changes, readback, failed-write restoration of the actual previous words, and separation from arena/rewind/resource code. There is no runtime original-instruction preflight.
- Focused compiled-NRO AArch64 startup checks register **40 hooks** and write precisely the activity replacements. Every startup case models Cobalt's preinstalled ProcInst.CreateBind hook; a separate case also models an existing inventory-getter hook. Neither entry is overwritten. Version, Build ID, layout, activity-write and configuration regression cases remain in CI.
- Well tests and compiled callbacks cover explicit localized entries, duplicate insertion prevention, unchanged original metadata, pending/ready/invalid state and level gates, grouped quantities, independent RNG, explicit-only reseeding and failed-preview seed recovery. No inventory-grant API is modeled or callable in this path. Actual rendered menus, waiting/collection and preview-to-claimed consistency still require in-game verification.
- Focused and compiled arena cases cover both ordinary and paid training, retaining native special-challenge and mode/EXP exclusions. Additional descriptors and changed callback addresses no longer suppress the edit; additional descriptors are untouched. The same native growth/fade steps remain in order for both modes. Real settlement still requires in-game verification.
- Compiled changed-activity-instruction and failed-write cases still register all 40 hooks and four settings. A failed activity write restores prior words without disabling unrelated features.
- Configuration cases cover missing/malformed/oversized/invalid UTF-8/short reads, mount failure, and counts 2, 10, 300, 600, and 1000. Startup only reads independent mod files; explicit UI edits use bounded recoverable transactions. Handles/mounts are released.
- Compiled draw text fixtures cover supported text languages, exact compact prices, unchanged actual native prices, and unchanged single-row behavior.
- Compiled draw-menu fixtures retain native one/ten and additional rows with their handlers, append the configured native count item once, and avoid a duplicate when configured to ten. Original ten draws cannot arm batch presentation bypass.
- Compiled meld-dialog fixtures construct single/batch/cancel choices with distinct localized labels and preserve additional choices. Alternating single/batch selections preserves their separate confirmation and presentation paths. Focused tests verify unit versus total costs and the single-copy fallback.
- Cleaning planning tests cover owner scoping, cancellation, valid native GodType values (-1 through 3), clean exclusions and shared-DLC-ring deduplication. The compiled menu/settlement fixture covers native cleaner attribution, current unavailability/state gates, repeated dispatch, cancellation, failed dirty setters, missing cleaners and restoration of the original static ring selection.
- Focused Wyvern tests cover difficulty-to-config mapping and reading the first native SSS border without imposing an array-length, sorting or rank-recalculation gate. The compiled-hook fixture covers all three difficulties, native tick/finish/score ordering, pending score additions, repeated dispatch, opening/event/test/pause/retirement conditions, null pointers, empty arrays and invalid difficulties.

- The compiled collection fixture covers first-position menu insertion, repeated binding without duplicate entries, unchanged original class/vtable, game-language message lookup, donation versus precomputed quantities, full-inventory dispatch to native settlement, duplicate source nodes, claimed and existing-zero flags, NPC result-index/access-count gating including reconstructed TalkItem, story exclusions, barn/donation gates, Somniel animal exclusion, adoption-only batches and repeated dispatch without duplicate grants. Pickup nodes with DisabledTalk remain eligible; disabled NPC gifts do not.

These tests model loader/native boundaries. They do not run Unity, render the game UI, or prove in-game state-machine completion.

The added compiled convenience fixture exercises all four Plugin Settings rows,
stable-PID chef selection, missing-join fallback, native cooking-branch scoping,
missing-taste fallback, preservation of other preferences and independent files.
Support dispatch returns native A/B results, excludes unready/special pairs,
does not retry dispatched pairs and never writes relation ranks/EXP. Bond and
gallery modes fall back to original input. The facility fixture binds all seventeen
menu entry points, tests native cleanup before reopening the parent/footer,
checks unlock/Somniel gates and keeps the original classes unchanged. Remote
ring rubbing is hidden on the private shortcut only.
The scene-arrival fixture covers Somniel and post-battle exploration, busy/non-Main
deferral, once-per-visit dispatch and restoration without invoking menu virtuals
on HubSequence. Activity shortcuts reject exhausted allowances.

Focused compiled cooking checks cover signed preview modifiers, grade/stat text,
conditional labels and context-scoped outcome selection. The animation factory
retains its result and completes a fresh, settled iterator returned by the native
factory even when another mod has changed MoveNext. Already-started, off-Somniel
or unsettled cases are unchanged.
Compiled shop checks cover each allowlisted store with missing, infinite, zero,
one and positive stocks, unrelated/null generic MethodInfo and ordinary same-key
dictionary reads. The fixture enters through a modeled native far jump, executes
the compiled bridge and native continuation branch, and checks SP, x19-x29,
d8-d15, return address, comparison flags and one original lookup. Saved dictionaries
are not written and the general dictionary function has no hook. All 298 pinned
instruction windows match the supplied NSO during offline verification; runtime
startup does not compare those windows against entries already hooked by Cobalt.
An independent configuration with a nonempty chef PID verifies no allocations
on deferred or repeated idle support ticks. Dialogue non-Main labels do not
initialize class metadata; each required class initializes at most once.
No combat frame-time or FPS improvement has been measured in the actual game.

Menu inheritance regression checks use the native 27-slot BasicMenuItem and
30-slot MapBasicMenuItem, with concrete HubMenuItem/GodItem and well exchange
subclasses. They verify constructor/type agreement, collection at the first
position, all seventeen facility choices, well preview/reroll callbacks, repeated
openings and unchanged original class tables. Missing required map interfaces,
out-of-bounds callback slots and null items reject insertion without grants.
There are no arbitrary instance-size, maximum-vtable or ancestry-depth limits.
The same pass uses 27-slot ring-cleaning and meld-confirmation
items so neither rejects a valid native class for lacking unrelated help slots.
The startup regression also invokes collection, well, cleaning, meld and settings
callbacks after modeling Cobalt's preinstalled hook. These are compiled boundary
checks, not evidence that the real game UI has been displayed.

The shop fixture does not execute native purchase settlement. Verify actual
prices, capacity refusal, sold-out unlocked goods and locked chapters in game.
Activity resource transitions, menu restoration after repeated/cancelled use,
the meal animation/result flow and scene-arrival support events also need game
testing; compiled boundary checks are not a substitute for Unity execution.

Still requiring real-game validation: actual facility menus/resources and their
header/footer appearance, end-to-end queued Lua support completion and viewed
records, chef models/names through cooking and reload, and condition-specific
taste variations. The boundary fixtures are not evidence that these Unity flows
have been played successfully. No user save or installed mod was changed.

Settings transaction tests cover independent-file round trips, every individual
I/O failure, old-value recovery and stale transaction cleanup. The compiled
settings fixture invokes the registered Cobalt callback, changes arbitrary and
boundary counts, verifies exact file contents and current UI values, checks
mount/write/rename failure feedback, no-write unchanged input, resource release
and an unchanged original ConfigBasicMenuItem class. No game-variable API is
available to this settings path.

Settings fixtures distinguish Il2CppAssembly from its image, matching Cobalt
1.31's scanned Assembly::Load entry at 0x42918C. They use the native 27-slot
ConfigBasicMenuItem inheritance, not invented SetCommandText/SetHelpText slots,
and model PluginSubmenu's second class clone before editing each preference.
The old assembly/image handling and DisabledTalk pickup filter were each
reproduced as failures before fixing the plugin. Actual in-game opening and
rendering still require validation with the rebuilt plugin.

Focused planning tests and the compiled arena fixture cover per-instance descriptor ordering, native
growth/fade retention, unchanged delegates/calculators, ordinary/paid selections,
special/inconsistent-EXP exclusions, missing children and additional descriptors.
The compiled dialogue fixture covers paid training, deeply nested Lua
parents, manually selected C/B/A support/bond calls, repeated Main-to-Skip dispatch
and every other lifecycle label. Pact Ring, story, ordinary/special/zero-EXP arena
calls, wrong-order and missing-target cases retain original Tick. Redundant
parent/child mirror fields and a fixed eight-parent limit no longer suppress
eligible dialogue skipping. Parent sequences,
callbacks and data objects remain unchanged; no global skip flag is set.

## In-game checks still required

Use a temporary test save or the user's explicitly chosen test environment. Do not automatically install anything or modify the main save.

- Start with only the combined plugin, then with the user's other enabled modifications. Confirm there is no loading stall and both ring menus work. Compare the same battle scene against the previous release for frame time; do not infer performance from hook counts alone.
- Create one ring, ten rings and a configured batch; check all menu entries, costs, acquired results, capacity stopping, and original RNG/probabilities. Check configuration ten does not add a duplicate row.
- Adjust the batch count through Plugin Settings, reopen ring creation and check labels, prices and actual counts. Restart and switch save slots to confirm independent persistence; check localized labels and an unwritable SD card. These Unity/rendering/storage-device checks are not proven by the compiled fixture.
- Meld C→B, B→A, A→S; check exact material/fragment costs, equipped copies, capacity, first/repeat acquisitions, cancellation, and consecutive transactions.
- Confirm a batch meld returns directly to the menu without pressing +, the header/footer/background remain correct, and leaving/re-entering works.
- Clean a dirty ring and a bracelet through one effective action; confirm the original completion and bond settlement.
- Select Polish All, choose a cleaner and check every eligible dirty ring/bracelet, shared DLC rings, actual bond gains, level-up notifications, cancellation, consecutive visits and return to the facility without a rubbing scene. Confirm individual cleaning remains unchanged. The modeled fixture does not prove Unity rendering or this real state-machine hand-off.
- Feed and pet Sommie through the original entries. Check food preference, food consumption rules, affection/animation events, first versus repeated reward eligibility, automatic petting finish, cancellation and return cleanup. Shortened coroutine waits are statically checked, not game-tested.
- Fish with different rods/fish; confirm quick bite/landing, catch inventory, records, rewards, and unchanged allowance.
- Run each exercise at available difficulties; confirm quick/perfect result, native reward, and unchanged allowance.
- Use paid Emblem training below and across bond-conversation thresholds. Confirm exact fragment costs, correct bond/skill growth, conversation completion, equipment/HP restoration, consecutive visits and return to the menu without a fight. Check ordinary training skips its fight but retains EXP, support and allowance use. These require real-game verification; the fixture does not prove settlement or Unity scene completion.
- At training bond thresholds, confirm automatic message skipping returns to the menu, updates native viewed records and allows subsequent eligible bond levels. Check repeated and DLC Emblems, C/B/A support and selected/replayed bond conversations outside training. Story and Pact Ring events must still play normally. Native Lua events, rewards and callback execution are not emulated by the fixture. Also verify ready ordinary supports dispatch once after Somniel/post-battle scene entry without interrupting entry scripts.
- Run Wyvern Ride on each unlocked difficulty, with and without Sommie. Confirm opening, quick ending, SSS score/rank, correct rewards and unchanged eligibility, higher existing records, consecutive visits, cancellation/retirement, and return to the hub. Hit statistics are not fabricated; confirm the original result screen remains usable. These Unity/state-machine checks require in-game verification and are not proven by the modeled fixture.
- Repeat cooking with different companions/recipes; inspect ingredient handling, support results, meal effects, and packed-lunch behavior.
- Open the food list at zero and positive stock; inspect Sommie feeding, native inventory handling, and the normal return flow.
- Gift an ordinary item with zero, one, and multiple owned copies; confirm visibility/selectability, no count change, normal recipient response/support, and no global inventory effect.
- Confirm special relationship/plot restrictions and non-gift item use are unchanged.
- In both the Somniel and post-battle exploration, use the collection entry and confirm the original reward dialog, menu/minimap restoration, markers and effects. Check ground drops with multiple donation levels, all five generated farm products, eligible and locked animals, and a protected villager's ordinary gift. Verify exact inventory and native claim/counter changes in a temporary save, reload, talk to the NPC and re-enter the map; none should duplicate. At full quantity stacks, money/fragments and convoy capacity, verify the original reward settlement behavior without plugin-side filtering. Verify that plot, recruitment, support and special gifts remain untouched. The fixture does not establish these Unity or real-save results, or compatibility with other mods that alter the same inventory/settlement routines.
- Save and reload; remove the plugin and reload a test save to verify no custom IDs or schema dependency was introduced.
