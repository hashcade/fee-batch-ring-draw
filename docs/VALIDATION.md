# Validation

## Completed locally

- Build the combined Switch NRO with the pinned Skyline dependency.
- Verify actual relocation-backed imports against Cobalt 1.31.0 / Engage 2.0.0. No unavailable `get_program_id`, NRO-load/unload hook, or plugin-address API is accepted.
- Read-only verification of the supplied NSO: Build ID, mapped segment layout, segment checksums, and **152** draw/meld/activity/Wyvern/collection instruction signatures.
- Focused meld planning/session/confirmation tests retain cost, capacity, equipped-copy, repeated-dispatch, and fragment-refill protections.
- Presentation-session tests cover matching owner, one-shot consumption, cancellation/cleanup, stale ownership, and null ownership.
- Activity tests cover all **24** instruction replacements, whole-set preflight, readback, each possible failed write and restoration, and separation from arena/rewind/resource code.
- Compiled-NRO AArch64 startup smoke checks: supported game registers **18 hooks** and writes precisely the activity replacements; wrong game/version/layout/instruction and conflicting activity/Wyvern/collection instructions register none and write nothing.
- A compiled failed-write case restores the original instructions, returns normally, and installs no hooks.
- Configuration cases cover missing/malformed/oversized/invalid UTF-8/short reads, mount failure, and batch counts 2, 300, 600, and 1000. The filesystem is read-only; handles/mounts are released.
- Compiled draw text fixtures cover supported text languages, exact compact prices, unchanged actual native prices, and unchanged single-row behavior.
- Focused Wyvern tests cover difficulty-to-config mapping, dynamic SSS boundaries, and malformed arrays. The compiled-hook fixture covers all three difficulties, native tick/finish/score ordering, pending score additions, repeated dispatch, opening/event/test/pause/retirement guards, null pointers, and invalid difficulty/config/native-rank fallbacks.

- Collection reservation tests cover duplicate claim identities, shared inventory/species/convoy budgets, atomic source reservations, arithmetic overflow, invalid capacities and the bounded reward list.
- The compiled collection fixture covers first-position menu insertion, repeated binding without duplicate entries, unchanged original class/vtable, game-language message lookup, donation versus precomputed quantities, full inventory, duplicate source nodes, claimed and existing-zero flags, NPC result-index/access-count gating including reconstructed TalkItem, story exclusions, barn/donation/capture-cap gates, Somniel animal exclusion, adoption-only batches and repeated dispatch without duplicate grants.

These tests model loader/native boundaries. They do not run Unity, render the game UI, or prove in-game state-machine completion.

## In-game checks still required

Use a temporary test save or the user's explicitly chosen test environment. Do not automatically install anything or modify the main save.

- Start with only the combined plugin, then with the retained resource/stock cheats. Confirm there is no loading stall and both ring menus work.
- Create one ring and a configured batch; check costs, acquired results, capacity stopping, and original RNG/probabilities.
- Meld C→B, B→A, A→S; check exact material/fragment costs, equipped copies, capacity, first/repeat acquisitions, cancellation, and consecutive transactions.
- Confirm a batch meld returns directly to the menu without pressing +, the header/footer/background remain correct, and leaving/re-entering works.
- Clean a dirty ring and a bracelet through one effective action; confirm the original completion and bond settlement.
- Fish with different rods/fish; confirm quick bite/landing, catch inventory, records, rewards, and unchanged allowance.
- Run each exercise at available difficulties; confirm quick/perfect result, native reward, and unchanged allowance.
- Run Wyvern Ride on each unlocked difficulty, with and without Sommie. Confirm opening, quick ending, SSS score/rank, correct rewards and unchanged eligibility, higher existing records, consecutive visits, cancellation/retirement, and return to the hub. Hit statistics are not fabricated; confirm the original result screen remains usable. These Unity/state-machine checks require in-game verification and are not proven by the modeled fixture.
- Repeat cooking with different companions/recipes; inspect ingredient handling, support results, meal effects, and packed-lunch behavior.
- Open the food list at zero and positive stock; inspect Sommie feeding, native inventory handling, and the normal return flow.
- Gift an ordinary item with zero, one, and multiple owned copies; confirm visibility/selectability, no count change, normal recipient response/support, and no global inventory effect.
- Confirm special relationship/plot restrictions and non-gift item use are unchanged.
- In both the Somniel and post-battle exploration, use the collection entry and confirm the original reward dialog, menu/minimap restoration, markers and effects. Check ground drops with multiple donation levels, all five generated farm products, eligible and locked animals, and a protected villager's ordinary gift. Verify exact inventory and native claim/counter changes in a temporary save, reload, talk to the NPC and re-enter the map; none should duplicate. Check almost-full quantity stacks, money/fragments and a nearly full convoy; sources that do not fit must remain. Verify that plot, recruitment, support and special gifts remain untouched. The fixture does not establish these Unity or real-save results, or compatibility with other mods that alter the same inventory/settlement routines.
- Save and reload; remove the plugin and reload a test save to verify no custom IDs or schema dependency was introduced.
