# Validation

## Completed locally

- Build the combined Switch NRO with the pinned Skyline dependency.
- Verify actual relocation-backed imports against Cobalt 1.31.0 / Engage 2.0.0. No unavailable `get_program_id`, NRO-load/unload hook, or plugin-address API is accepted.
- Read-only verification of the supplied NSO: Build ID, mapped segment layout, segment checksums, and **98** draw/meld/activity instruction signatures.
- Focused meld planning/session/confirmation tests retain cost, capacity, equipped-copy, repeated-dispatch, and fragment-refill protections.
- Presentation-session tests cover matching owner, one-shot consumption, cancellation/cleanup, stale ownership, and null ownership.
- Activity tests cover all **24** instruction replacements, whole-set preflight, readback, each possible failed write and restoration, and separation from arena/rewind/resource code.
- Compiled-NRO AArch64 startup smoke checks: supported game registers **16 hooks** and writes precisely the activity replacements; wrong game/version/layout/instruction and conflicting activity instruction register none and write nothing.
- A compiled failed-write case restores the original instructions, returns normally, and installs no hooks.
- Configuration cases cover missing/malformed/oversized/invalid UTF-8/short reads, mount failure, and batch counts 2, 300, 600, and 1000. The filesystem is read-only; handles/mounts are released.
- Compiled draw text fixtures cover supported text languages, exact compact prices, unchanged actual native prices, and unchanged single-row behavior.

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
- Repeat cooking with different companions/recipes; inspect ingredient handling, support results, meal effects, and packed-lunch behavior.
- Open the food list at zero and positive stock; inspect Sommie feeding, native inventory handling, and the normal return flow.
- Gift an ordinary item with zero, one, and multiple owned copies; confirm visibility/selectability, no count change, normal recipient response/support, and no global inventory effect.
- Confirm special relationship/plot restrictions and non-gift item use are unchanged.
- Save and reload; remove the plugin and reload a test save to verify no custom IDs or schema dependency was introduced.
