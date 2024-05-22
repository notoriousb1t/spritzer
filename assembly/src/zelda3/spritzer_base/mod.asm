; Hooks and Symbols are declared first since they call org and target
; specific areas of the game.

; Symbols
incsrc ../ids.asm
incsrc ../ram.asm
incsrc ../rom.asm

; Hooks (should be limited to direct replacements or labeling addresses)
incsrc overworld_fixes.asm
incsrc ancilla_hooks.asm
incsrc blind_hooks.asm
incsrc killable_hooks.asm
incsrc goriya_hooks.asm
incsrc moldorm_hooks.asm

; Code goes in bank $36. This avoids conflicts with Enemizer and Archipelago/z3randomizer code.
org $B68000
incsrc blind.asm
incsrc killable.asm
incsrc goriya.asm
incsrc moldorm.asm