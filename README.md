# Spritzer

A The Legend of Zelda: A Link to the Past Sprite + Dungeon Randomizer

The goal of this randomizer is to supplement Item randomizers such as Archipelago with better enemy randomization and total dungeon randomization including changing the general theme of a dungeon. For example, Thieves Town can be converted to Ice Palace theme and enemies, etc.

This mod preserves all item logic present in the game, but will switch the hazards, enemies, and sometimes the NPCs to make the game fresher for veteran players.

There are a few defining characteristics of this randomization mod:

1. Dungeon palettes and tilesets swap to follow Boss Randomization by default. This provides a more cohesive experience in Dungeons.
2. Sprite randomization is divided into categories and controlled via balancing options. The default options provide variety without full chaos to ensure a pleasant journey. You can, however, opt for a more chaotic experience.

Options come in three flavors:

- Configuration for the Overworld
- Configuration for the Underworld (caves + dungeons)
- Configuration for Sprites

## General Options

### Seed

Controls randomization. Will produce the same result given the same general options. Some special values are present for debugging purposes. For example, if the word "moldorm4" appears in the seed, all bosses will change to Moldorm and Moldorm will have four eyes. This is not guaranteed to be winnable and is largely for debugging purposes.

### Shadow Bees

Changes bees to be more powerful and partially invisible.

## Overworld Options

### Overworld Balancing

Balancing controls how difficult sprites, hazards, and consumables are for the player in the overworld. This affects rerolling of enemies, consumables, hazards, and traps (aka Overlords)

1. Random - There is an equal chance of swaps within a category.
1. Casual - The game is biased to make the game easier for the player.
1. Balanced - This generally should feel similar to the original game.
1. Hero - The game is biased to make the game more difficult for the player.

### Overworld Enemy Shuffle

Enemy Shuffle controls which enemies/etc. appear within the overworld

1. Vanilla - Changes to the enemies placed are avoided
1. Simple - Enemies are shuffled, but each screen should contain the same enemies overall.
1. Full - Enemies are rerolled. Enemies that appear together are chosen at random.
1. Inverted - Dark and light world enemies may be swapped and rerolled. Kakariko NPCs are moved to the dark world except for Bottle Salesman, who is moved to Lumberjack's house to preserve item logic.
1. Chaos - TBD - planned to mix light and dark world indiscriminately
1. Insanity - TBD - planned to indescriminately place hazards and enemies from any location.

### Mushroom Shuffle

Shuffles the location of the overworld Mushroom location in the Lost Woods. The item here may appear at any of the locations of Fake Swords or the original location at random.

## Underworld Options

### Dungeon Shuffle

Swap boss, tileset, and palettes as possible. Additional options are planned including Boss Chaos and Advanced Boss Logic.

### Underworld Balancing

Balancing controls how difficult sprites, hazards, and consumables are for the player in the underworld. This affects rerolling of enemies, consumables, hazards, and traps (aka Overlords)

1. Random - There is an equal chance of swaps within a category.
1. Casual - The game is biased to make the game easier for the player.
1. Balanced - This generally should feel similar to the original game.
1. Hero - The game is biased to make the game more difficult for the player.

### Underworld Enemy Shuffle

Enemy Shuffle controls which enemies/etc. appear within the underworld (dungeons + caves)

1. Vanilla - Changes to the enemies placed are avoided
1. Simple - Enemies are shuffled, but each supertile should contain the same enemies overall.
1. Full - Enemies are rerolled. Enemies that appear together are chosen at random.
1. Chaos - TBD - planned to mix enemies between dungeons
1. Insanity - TBD - planned to indescriminately place hazards and enemies from any location

## Other World Changes

Some features are enabled as a result of other enabled features:

1. Killable Thieves - Any value other than Vanilla for Overworld and Underworld Shuffle will enable killable thieves. If there is demand to make this togglable, please file an issue.
2. Overlord Shuffle - Any value above Full for Overworld and Underworld Shuffle will also randomize things like cannon rooms and other minor events. There are hard technical reasons why changes to Overlords are required.
3. Boss Customization - Some bosses may have randomized appearances whe Boss Shuffling is enabled. For example, Moldorm may have up to 8 eyes.
4. Dungeon Shuffle - By default, shuffling bosses also shuffles the appearance and characteristics of the dungeons. In some cases, like Tower of Hera, the boss room may change appearance. Please file an issue, if there is demand to turn this off.
5. Visual Enemy Changes - Some enemies have changed color palettes to be more stable across different locations. For example, mini-moldorm uses the blue palette index since it is more consistent in different areas.
6. Overworld Flattening - Any value other than Vanilla for Overworld and Underworld will remove distinct Pre-Aga and Post Aga sprite configurations. In other words, enemies will remain the same before and after killing Agahnim, unless the room has a specific logic to be preserved. There are really only a few overworld areas such as Hyrule Castle, Lumberjack's house, etc. that have specific Sprites in particular game states.

## Pre-reqs for Building

1. Install Rust
2. Install Visual Studio Code and the Rust Analyzer extension
3. Run `rustup target add wasm32-unknown-unknown` to install the web assembly compiler
4. Install the wasm-bindgen-cli `cargo install wasm-bindgen-cli`
5. Install Maturin -- This adds compiler support for Python bindings

## Running

- Run the task, "Python: Main UI" from the Visual Studio Code debug menu.
- Run `npm run docs:dev` to run the docs site.

> NOTE: The assembly project requires Windows to run and a copy of asar.exe in the directory to rebuild .asm files. But if changes to the asm are not required, this is not required to make contributions. For ASM changes, the

## Architecture

This is a workspace project for Rust. At any time, running `cargo build` will compile all compilable targets. There are a few different projects:

- assembly - Contains direct Assembly code needed to reprogram parts of the game. This automatically generates Rust code that contains the bytes generated from the code. However, this does not use a copy of the game and instead diffs off an "empty" file of 4MB. As a result, it is not possible to `org` 2 addresses that are not at least 2 bytes away from each other. The current marker is 0xFF 0xFF to detect "empty".
- library - This contains the main logic of the randomizer and contains models for working with objects in the game, reader/writers for performing conversions, and logic to perform transformations on dungeons, sprites, overworld, etc. This is the majority of code in the project.
- python - This contains bindings to convert Rust code to python, so this project can be run from python frontends.
- ui - A python UI for running the mod.
