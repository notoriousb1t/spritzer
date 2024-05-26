---
sidebar: "auto"
---

# Guide

Spritzer adds sprite randomization to Zelda: A Link to the Past. This game mod preserves all item logic present in the game on a room by room basis, but will switch the hazards, enemies, dungeon tilesets, and sometimes the NPCs to make the game fresher for fans of Zelda 3.

Currently, ALTTPR, Archipelago, and the base Japanese version of the game are compatible. See below "Enemizer FAQs" to troubleshoot conflicts between Spritzer and Enemizer.

## Options

### Overworld Balancing

Balancing controls how difficult sprites, hazards, and consumables are for the player in the overworld. This affects rerolling of enemies, consumables, hazards, and traps (aka Overlords)

1. **Random** - There is an equal chance of swaps within a category.
1. **Casual** - The game is biased to make the game easier for the player.
1. **Balanced** - This generally should feel similar to the original game.
1. **Hero** - The game is biased to make the game more difficult for the player.

### Overworld Enemy Shuffle

Enemy Shuffle controls which enemies/etc. appear within the overworld

1. **Vanilla** - Changes to the enemies placed are avoided
1. **Full** - Sprites are rerolled in place. Generally enemies that appear together in the original game stay together.
1. **Chaos** - Sprites are regrouped and rerolled randomly. The overworld contains all normal enemies and hazards from both the overworld and underworld.
1. **Insanity** - TBD - Currently the same as Chaos

### Killable Thieves

Makes thieves killable (one or two hits)

### Mushroom Shuffle

Shuffles the location of the overworld Mushroom location in the Lost Woods. The item here may appear at any of the locations of Fake Swords or the original location at random. Additional enemies are added to the Lost Woods.

### Boss Shuffle (Experimental)

> This is in active development and may make the game unwinnable in certain circumstances

Shuffles the bosses while preserving item logic. 


### Overworld Inverted

Dark and light world enemies may be swapped and rerolled. Kakariko NPCs are moved to the dark world except for Bottle Salesman, who is moved to Lumberjack's house to preserve item logic for item randomizers. When this is enabled and 
Overworld Shuffle is set to Vanilla, sprites are shuffled in the overworld as if Full was set.

### Underworld Balancing

Balancing controls how difficult sprites, hazards, and consumables are for the player in the underworld. This affects rerolling of enemies, consumables, hazards, and traps (aka Overlords)

1. **Random** - There is an equal chance of swaps within a category.
1. **Casual** - The game is biased to make the game easier for the player.
1. **Balanced** - This generally should feel similar to the original game.
1. **Hero** - The game is biased to make the game more difficult for the player.

### Underworld Enemy Shuffle

Enemy Shuffle controls which enemies/etc. appear within the underworld (dungeons + caves)

1. **Vanilla** - Changes to the enemy placements are avoided
1. **Full** - Sprites are rerolled in place. Generally enemies that appear together in the original game stay together.
1. **Chaos** - Sprites are regrouped and rerolled randomly. The underworld contains all normal enemies and hazards from both the overworld and underworld.
1. **Insanity** - TBD - Currently the same as Chaos

### Seed

> Some special values are present for debugging purposes, but will be removed. For example, if the word "moldorm4" appears in the seed, all bosses will change to Moldorm and Moldorm will have four eyes. Debug strings are not guaranteed to produce a winnable game. For example, agahnim will replace all bosses with agahnim but they currently do not drop boss prizes.

Controls randomization. Is intended to produce the same result given the same general options.


### Shadow Bees

Changes bees to be more powerful and partially invisible. If using with Archipelago Bee Traps, expect pain.

## Other World Changes

The following lists changes to the base game

- Some enemies have changed color palettes to be more stable across different locations. For example, mini-moldorm and Green Eyegore are blue to improve user experience across environments. Also Moldorm has a random number of eyes.
- Enemies/NPCs will remain the same before and after killing Agahnim unless the room has a specific item logic. i.e. Hyrule Castle, Lumberjack's house, etc.
- Green Goriya/Mimics are replaced with Blue Goriyas. Blue Goriya can be harmed by most damage types and shoot fireballs. Red Goriya/Mimics are replaced with Red Eye Gores

## Patching ALTTPR

1. Generate a game on [ALTTPR](https://alttpr.com/en/randomizer)
1. Download the game file.
1. Generate the file in [Randomize](randomize.html)

## Patching Archipelago

1. Generate a game on [Archipelago](http://https://archipelago.gg)
1. Download the patch file and run the alttp file. Do not close Archipelago or connect via Lua.
1. Generate the file in [Randomize](randomize.html)
1. Replace the .sfc generated by Archipelago
1. Connect via Lua as normal

> TIP: Archipelago may replace the .sfc each time it is launched, so it is a good idea to keep a backup of the generated file if you need to restart.

## FAQs

### Enemizer Troubleshooting

To make the game compatible for Archiepelago and Archipelago, remove these modifications that require Enemizer:

 - Boss Shuffle
 - Enemy Shuffle
 - Killable Thieves
 - Enemy Health & Damage changes
 - Pot Shuffle
 - Bush Shuffle

