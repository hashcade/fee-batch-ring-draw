# Validation

## Implemented behavior

- Replace the ten-result menu entry with the configured count (default 300); retain single creation.
- Use the native constructor for price calculation and affordability.
- Localize the configured-count label for all twelve game text-language IDs.
- Read `fee-batch-ring-draw.cfg` once at startup from a private, temporary SD mount. Accept `draw_count=2–1000`, defaulting to 300 for missing or invalid configuration. Close opened files and unmount on all successful-mount paths. Do not write files.
- Snapshot the initial fragment budget and cap execution at the configured count.
- Predict the next native result using a **copy** of the four-word RNG state, the game's current rank parameters, and the original RingData list order.
- Check that exact result with `UnitRingPool.CanAdd`, total stock including equipped copies, and current funds before executing one native transaction.
- Do not consume or restore live RNG state during prediction. If a result is full, stop before drawing it; no alternative result is selected.
- Reuse `RefineRingExecute.Do(god, 1, out flags)` for every acquired ring. Validate actual output identity, RNG state, inventory growth, and equipment ownership afterward. Never retry a failed transaction.
- Aggregate actual result references in preallocated, managed lists rooted by `GodRoomPedestalSequence`. The original outer achievement pass receives every acquired result, not merely the last one.
- Bypass `CreateRingGachaDemoSequence` before it hides the header/footer or unbinds the background. The method's inspected calls only prepare/create the child presentation. Do not create a presentation child, switch scenes, or invoke its scene-restoration logic. The original outer achievement and resource-cleanup steps remain intact.

## Native profile

All addresses are relative to the supported main module.

| Entry | Address |
| --- | --- |
| RefineRingTimesSelectMenuItem constructor | `0x02957110` |
| RefineRingTimesSelectMenuItemContent.BuildText | `0x02957B20` |
| GodRoomPedestalSequence.ExecuteRingGacha | `0x0233A170` |
| RefineRingExecute.Do | `0x02952990` |
| GodRoomPedestalSequence.CreateRingGachaDemoSequence | `0x0233A380` |
| GodRoomPedestalSequence.UnloadResourcesForRefineRing | `0x0233A660` |
| Random.get_Game | `0x02374C70` |
| Random.GetValue(max) | `0x02375170` |

The native label builder explicitly handles only 1 and 10; unsupported counts otherwise produce blank text. The plugin replaces that label after the original builder runs, keeping native cost formatting and colors.

The native transaction consumes two `Random.GetValue` results: rank modulo 100, then character index modulo the selected rank's candidate count. No preview transaction is run against the save. Native RNG instructions and relevant transaction call sites are pinned in the profile.

## Automated coverage

- Host tests: bounded execution, normal cost, insufficient funds, full stacks, equipped counts, pool capacity, no rerolls, fragment-refill cheats, failure/no-retry, RNG/rank boundaries, localization, parent-scoped one-shot dispatch, and unload cleanup.
- Python tests: package paths, checksums, release versions, NSO parsing, NRO structure, and unavailable Cobalt-import rejection.
- Compiled NRO initialization: supported game registers six hooks; wrong version, Build ID, instruction, or layout registers none. No unsupported `get_program_id` or NRO-loader interface is used.
- Compiled configuration paths: missing file/mount, empty file, valid low/custom/high counts, invalid UTF-8/value, oversized input, and short reads. Exercise the compiled menu constructor to verify the effective count and unchanged single creation. Unsupported executables do not read configuration.
- Read-only supplied NSO verification: Build ID, segment checksums, and native instruction signatures.

These tests do not simulate Unity scenes or certify in-game behavior.

## Pending in-game acceptance

Use a disposable test save, not a primary save.

- Confirm the menu shows Create One / Create 300 with correct prices in the selected language.
- Configure a different count, restart the game, and confirm both the localized label and normal total price change with it.
- Run 300 creation operations with sufficient funds and free capacity; verify 300 actual inventory additions and the normal total cost.
- Confirm single creation remains unchanged and batches return promptly without result windows, a hidden header/footer, or a missing background.
- Repeat batches, switch Emblems, cancel selection, leave and re-enter the room.
- Test a predicted full stack, equipped copies, pool capacity, and insufficient funds. Confirm no rejected result is charged or rerolled.
- Validate acquisition records, total-created counters, and rank-specific achievements after saving and reloading.
- Confirm coexistence with FEE Batch Ring Meld. Independently verify costs with fragment-refill cheats disabled.

No game installation or user save was modified during development.
