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

; Code goes in bank $29 (fast lorom location is $69). This seems to be uncontested.
; Start at position 60 since there is a series of $6B at the start of the bank that
; is of unknown use. Maybe it is used for settings or something.
org $698060
incsrc blind.asm
incsrc killable.asm
incsrc goriya.asm
incsrc moldorm.asm