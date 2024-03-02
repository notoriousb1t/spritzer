; Hooks and Symbols are declared first since they call org and target
; specific areas of the game.

; Symbols
incsrc ../ids.asm
incsrc ../ram.asm
incsrc ../rom.asm

; Hooks (should be limited to direct replacements or labeling addresses)
incsrc ancilla_hooks.asm
incsrc blind_hooks.asm
incsrc killable_hooks.asm
incsrc mimic_hooks.asm
incsrc moldorm_hooks.asm
incsrc uw_spawn_hooks.asm

; Code
org $368000
incsrc blind.asm
incsrc killable.asm
incsrc mimic.asm
incsrc moldorm.asm
incsrc uw_spawn.asm