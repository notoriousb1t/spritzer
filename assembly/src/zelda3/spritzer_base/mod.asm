; Hooks and Symbols are declared first since they call org and target
; specific areas of the game.

; Symbols
incsrc ../ids.asm
incsrc ../ram.asm

; Settings are fit into bank $1 in some free space.
incsrc settings.asm

; Hooks (should be limited to direct replacements or labeling addresses)
incsrc hooks.asm
incsrc goriya_hooks.asm
incsrc fixes.asm

; New Code goes in bank $36. This avoids conflicts with Archipelago and ALTTPR.
org $B68000
incsrc boss.asm
incsrc killable.asm
incsrc goriya.asm
incsrc moldorm.asm