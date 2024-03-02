
; Sourced from Enemizer, but with changes. This converts all mimics to red mimics
; except for mimic cave which must be completable without requiring bow.
; Additionally, all mimics shoot fireballs now and are identified by $B8.

; Symbols - Existing named code locations.

org $0DB818
SpritePrep_LoadProperties:

; Hooks - Places where code is intercepted.
; sprite_prep.asm (2466) -> SpritePrep_EyegoreTrampoline:
; 0311B6   1E:C6FA? ; SpriteActive3_Transfer?
org $0691B6
;set eyegore to be only eyegore
JSL SpritePrep_EyegoreOrMimicNew 

; 0307CB replace debugged sprite create by eyegore
; SpritePrep_Eyegore jump table
org $0687CB
dw #$91B6 

; Bank1E.asm (140) -> dw Sprite_DialogueTester   ; 0xB8 - debug artifact, dialogue tester
org $1E8BB1 
dw #$C79D

; Update all references to $84 (red mimic) to $B8
org $1EC7C5
db !SPRITE_MIMIC
org $1EC801
db !SPRITE_MIMIC

; Set damage type for $B8.
org $0DB9A9 
db $00

; Bank06.asm (4593) - damage calcs
org $06EC08 
JSL resetSprite_Mimic
NOP ; Required to cancel out an otherwise invalid instruction.

; Replace Eyegore Sprites in Room 0x10C (Mimic Cave) with Mimic.
org $09EBCD
db $14, $07, !SPRITE_MIMIC
org $09EBD0
db $14, $08, !SPRITE_MIMIC
org $09EBD3
db $14, $0C, !SPRITE_MIMIC
org $09EBD6
db $1A, $0C, !SPRITE_MIMIC

; Replace Eyegore Sprites in Room 0x1B (POD) with Mimic.
org $09DB19
db $14, $07, !SPRITE_MIMIC
org $09DB1C
db $1C, $03, !SPRITE_GREEN_EYEGORE
org $09DB1F
db $1C, $0C, !SPRITE_MIMIC

; Replace Eyegore Sprites in Room 0x4B (POD) with Mimic.
org $09DEEB
db $04, $07, !SPRITE_MIMIC
org $09DEF4
db $08, $04, !SPRITE_GREEN_EYEGORE
org $09DEF7
db $08, $0B, !SPRITE_MIMIC

; Replace Eyegore Sprites in Room 0x6B (GT) with Mimic.
org $09E199
db $06, $0A, !SPRITE_MIMIC
org $09E19C
db $09, $06, !SPRITE_MIMIC
org $09E1A5
db $18, $03, !SPRITE_MIMIC
org $09E1AE
db $1B, $0C, !SPRITE_MIMIC
org $09E1B1
db $15, $17, !SPRITE_MIMIC
org $09E1BA
db $1B, $18, !SPRITE_MIMIC