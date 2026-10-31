# Implementation

Target: Engage 2.0.0, Build ID `8C08B9719E085F91847B5E0F935D9488`, Cobalt 1.31.0. All addresses are main-module relative.

## Retained scope

Draw/meld transactions preserve normal costs, RNG, acquisition records, equipped-copy exclusions and native capacity rules. Draw keeps one/ten plus the configured batch. Presentation is bypassed before header/background changes; original outer settlement and cleanup remain.

Collection runs synchronously from the original exploration menu and uses CommonRewardSequence.CreateBindForWell to load, grant, display and unload results. It processes pending generated sources only, without extra inventory budgets. Result disposal restores the same menu and selection.

Facilities retain one outdoor entry, a native category side panel, and separate native destination pages. Destination pages hide both ancestors; B resumes them with retained selections. Only the outdoor Ring Chamber shortcut hides RingCleaningItem. The actual room is untouched.

## Preferences

Fourteen ConfigBasicMenuItem classes, in one flat list. Native OnSelect, OnDeselect, BCall and InitContent survive both the plugin clone and Cobalt's second class clone. All footer help is empty. Batch count uses the native left/right control with step 10; other controls keep their existing steps. The independent SD settings file retains existing values; removed multiplier, rewind, save and outfit keys are accepted but not serialized.

There are no reward-multiplier, resource-editor, battle-save, rewind-limit, outfit, arena-presentation or well-preview/reroll hooks in this plugin. Standard-training quota logic remains independent from the removed battle-presentation code. The Talk tick hook is removed; manually started support and Emblem bond conversations use native playback.

## Automatic polishing

The generic singleton getter at `0x001FD360`, using metadata `0x0628DD48`, returns GodPool, **not List<GodUnit>**. Native GodPool.GetEnumerator at `0x03207830` loads the used list from `pool + 0x28`. The previous implementation misread the pool header as a list and silently found no entries.

Use this used list, deduplicate shared GodUnits, and retain native availability/dirt checks. Clear dirt through SetDirty, grant the native three-point finish reward through GodUnit's bond API, and record global and per-Emblem polishing completion. The actor comes from the joined-ally roster and is stored by PID.

The automatic scan runs once on idle Somniel arrival and again on exploration-menu opening, independently of automatic ordinary supports. It does not bind a polishing scene or change the manual selected God/actor.

The synthetic Polish All row, its cloned GodItem class, list reordering, row-count changes, selection session and bulk cleaner callback are removed. Manual one-action polishing remains the existing scoped activity instruction patch.

## Other retained behavior

- Native ordinary supports: CanLevelUp, UnitReliance.LevelUp and MyRoomRelianceCallSequence.UpdateAchive, only in Somniel.
- Shops: scoped stock reads/decrements for unlocked armory, item-shop and flea-market entries.
- Cooking: joined-chef selection, native taste definitions and shortened preparation. Load Cook messages before resolving taste display names, then release the text reference.
- Fishing: independent speed patches and native use-decrement control.
- Strength training and Wyvern Ride: native result/reward flow, with current-difficulty SSS threshold for Wyvern.
- Sommie: original interaction choices with shortened waits; ordinary-gift stock changes stay scoped to gift settlement.

## Inventory-free food

Sommie list inclusion already bypasses the stock branch at `0x01E546B4`. The valid-food BuildAttribute result at `0x02038900` now remains enabled at zero stock; its missing-data branch is untouched. NOP only the feeding coroutine's AddStock(-1) call at `0x01E54158`, then continue through the original scene/reaction/reward path.

Cooking's FoodstuffMenu creates normal recipe ingredients and rare additions without stock filtering. Its BuildAttribute stock branch at `0x01E63640` is bypassed; missing recipe slots and the native one-addition selection behavior remain unchanged. PushA and GetSelectedFoodstuffs retain those selected definitions. NOP only SubInventory at `0x02D813D4` in the settlement loop, after the native Cook call has already calculated the result using the selected array. The special-ingredient flag test at `0x02610EB0` is not a consumption call and stays intact.

The default-on Items Without Consumption setting controls ordinary gifts and both food conveniences together. Retain the existing `gift_stock` storage key and field to preserve previously saved on/off values, without separate food preferences. Disabling it restores all eight original gift/food stock gates and consumption calls. Repeat Cooking's activity quota remains separate. No global inventory method, inventory display, food effect, reward or save field is replaced.
