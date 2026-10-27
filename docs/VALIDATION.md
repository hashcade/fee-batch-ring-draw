# Validation

## Completed locally

- Build the combined Switch NRO with the pinned Skyline dependency.
- Verify actual relocation-backed imports against Cobalt 1.31.0 / Engage 2.0.0. No unavailable `get_program_id`, NRO-load/unload hook, or plugin-address API is accepted.
- Read-only verification of the supplied NSO: Build ID, mapped segment layout, segment checksums, and **225** draw/meld/activity/Wyvern/collection/cleaning/arena/dialogue/settings/level instruction signatures.
- Focused meld planning/session/confirmation tests retain cost, capacity, equipped-copy, repeated-dispatch, and fragment-refill protections.
- Presentation-session tests cover matching owner, one-shot consumption, cancellation/cleanup, stale ownership, and null ownership.
- Activity tests cover all **32** instruction replacements, whole-set preflight, readback, each possible failed write and restoration, and separation from arena/rewind/resource code.
- Compiled-NRO AArch64 startup smoke checks: supported game registers **25 hooks** and writes precisely the activity replacements; wrong game/version/layout/instruction and conflicting activity/Wyvern/collection/cleaning/arena/dialogue/level instructions register none and write nothing.
- Focused level tests verify current + internal, initial/capped levels, malformed values and compact headings. The compiled fixture covers all game languages, enhanced display versus stored total, original-setter ordering and unchanged unit memory. Actual status-screen sizing requires in-game verification.
- Focused and compiled arena cases cover both ordinary and paid training, rejecting special challenges, inconsistent mode/EXP values and mismatched or modified children. The same native growth/fade steps remain in order for both modes. Real ordinary EXP, support and activity-use settlement still require in-game verification.
- A compiled failed-write case restores the original instructions, returns normally, and installs no hooks.
- Configuration cases cover missing/malformed/oversized/invalid UTF-8/short reads, mount failure, and counts 2, 10, 300, 600, and 1000. Startup only reads independent mod files; explicit UI edits use bounded recoverable transactions. Handles/mounts are released.
- Compiled draw text fixtures cover supported text languages, exact compact prices, unchanged actual native prices, and unchanged single-row behavior.
- Compiled draw-menu fixtures retain native one/ten entries and their handlers, append the configured native count item once, and avoid a duplicate when configured to ten. Original ten draws cannot arm batch presentation bypass.
- Compiled meld-dialog fixtures construct single/batch/cancel choices with distinct localized labels. Alternating single/batch/single selections preserves their separate confirmation and presentation paths. Focused tests verify unit versus total costs and the single-copy fallback.
- Cleaning planning tests cover owner scoping, cancellation, valid native GodType values (-1 through 3), clean exclusions and shared-DLC-ring deduplication. The compiled menu/settlement fixture covers native cleaner attribution, current unavailability/state gates, repeated dispatch, cancellation, failed dirty setters, missing cleaners and restoration of the original static ring selection.
- Focused Wyvern tests cover difficulty-to-config mapping, dynamic SSS boundaries, and malformed arrays. The compiled-hook fixture covers all three difficulties, native tick/finish/score ordering, pending score additions, repeated dispatch, opening/event/test/pause/retirement guards, null pointers, and invalid difficulty/config/native-rank fallbacks.

- Collection reservation tests cover duplicate claim identities, shared inventory/species/convoy budgets, atomic source reservations, arithmetic overflow, invalid capacities and the bounded reward list.
- The compiled collection fixture covers first-position menu insertion, repeated binding without duplicate entries, unchanged original class/vtable, game-language message lookup, donation versus precomputed quantities, full inventory, duplicate source nodes, claimed and existing-zero flags, NPC result-index/access-count gating including reconstructed TalkItem, story exclusions, barn/donation/capture-cap gates, Somniel animal exclusion, adoption-only batches and repeated dispatch without duplicate grants.

These tests model loader/native boundaries. They do not run Unity, render the game UI, or prove in-game state-machine completion.

Settings transaction tests cover independent-file round trips, every individual
I/O failure, old-value recovery and stale transaction cleanup. The compiled
settings fixture invokes the registered Cobalt callback, changes arbitrary and
boundary counts, verifies exact file contents and current UI values, checks
mount/write/rename failure feedback, no-write unchanged input, resource release
and an unchanged original ConfigBasicMenuItem class. No game-variable API is
available to this settings path.

Focused planning tests and the compiled arena fixture cover per-instance descriptor ordering, native
growth/fade retention, unchanged delegates/calculators, ordinary/special/zero-EXP
exclusions, started or missing children, changed layouts and wrong targets.
The compiled dialogue fixture covers paid training and nested Lua parents,
repeat calls, native Main-to-Skip dispatch and every other lifecycle label.
Support, story, outside-training, ordinary/special/zero-EXP, stale/wrong-class,
missing-target and cyclic-parent cases retain original Tick. Parent sequences,
callbacks and data objects remain unchanged; no global skip flag is set.

## In-game checks still required

Use a temporary test save or the user's explicitly chosen test environment. Do not automatically install anything or modify the main save.

- Start with only the combined plugin, then with the retained resource/stock cheats. Confirm there is no loading stall and both ring menus work.
- Create one ring, ten rings and a configured batch; check all menu entries, costs, acquired results, capacity stopping, and original RNG/probabilities. Check configuration ten does not add a duplicate row.
- Adjust the batch count through Plugin Settings, reopen ring creation and check labels, prices and actual counts. Restart and switch save slots to confirm independent persistence; check localized labels and an unwritable SD card. These Unity/rendering/storage-device checks are not proven by the compiled fixture.
- Meld C→B, B→A, A→S; check exact material/fragment costs, equipped copies, capacity, first/repeat acquisitions, cancellation, and consecutive transactions.
- Confirm a batch meld returns directly to the menu without pressing +, the header/footer/background remain correct, and leaving/re-entering works.
- Clean a dirty ring and a bracelet through one effective action; confirm the original completion and bond settlement.
- Select Polish All, choose a cleaner and check every eligible dirty ring/bracelet, shared DLC rings, actual bond gains, level-up notifications, cancellation, consecutive visits and return to the facility without a rubbing scene. Confirm individual cleaning remains unchanged. The modeled fixture does not prove Unity rendering or this real state-machine hand-off.
- Feed and pet Sommie through the original entries. Check food preference, food consumption rules, affection/animation events, first versus repeated reward eligibility, automatic petting finish, cancellation and return cleanup. Shortened coroutine waits are statically checked, not game-tested.
- Fish with different rods/fish; confirm quick bite/landing, catch inventory, records, rewards, and unchanged allowance.
- Run each exercise at available difficulties; confirm quick/perfect result, native reward, and unchanged allowance.
- Use paid Emblem training below and across bond-conversation thresholds. Confirm exact fragment costs, correct bond/skill growth, conversation completion, equipment/HP restoration, consecutive visits and return to the menu without a fight. Check ordinary training retains its battle, EXP and allowance. These require real-game verification; the fixture does not prove settlement or Unity scene completion.
- At training bond thresholds, confirm automatic message skipping returns to the menu, updates native viewed records and allows subsequent eligible bond levels. Check repeated and DLC Emblems, then an ordinary support/story conversation and a bond conversation outside training: those must still play normally. Native Lua events, rewards and callback execution are not emulated by the fixture.
- Run Wyvern Ride on each unlocked difficulty, with and without Sommie. Confirm opening, quick ending, SSS score/rank, correct rewards and unchanged eligibility, higher existing records, consecutive visits, cancellation/retirement, and return to the hub. Hit statistics are not fabricated; confirm the original result screen remains usable. These Unity/state-machine checks require in-game verification and are not proven by the modeled fixture.
- Repeat cooking with different companions/recipes; inspect ingredient handling, support results, meal effects, and packed-lunch behavior.
- Open the food list at zero and positive stock; inspect Sommie feeding, native inventory handling, and the normal return flow.
- Gift an ordinary item with zero, one, and multiple owned copies; confirm visibility/selectability, no count change, normal recipient response/support, and no global inventory effect.
- Confirm special relationship/plot restrictions and non-gift item use are unchanged.
- In both the Somniel and post-battle exploration, use the collection entry and confirm the original reward dialog, menu/minimap restoration, markers and effects. Check ground drops with multiple donation levels, all five generated farm products, eligible and locked animals, and a protected villager's ordinary gift. Verify exact inventory and native claim/counter changes in a temporary save, reload, talk to the NPC and re-enter the map; none should duplicate. Check almost-full quantity stacks, money/fragments and a nearly full convoy; sources that do not fit must remain. Verify that plot, recruitment, support and special gifts remain untouched. The fixture does not establish these Unity or real-save results, or compatibility with other mods that alter the same inventory/settlement routines.
- Save and reload; remove the plugin and reload a test save to verify no custom IDs or schema dependency was introduced.
