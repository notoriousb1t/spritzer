
; Originally sourced from Enemizer, but with a lot of changes. This converts all goriyas to red goriyas
; except for mimic cave which must be completable without requiring bow.
; Additionally, all goriyas shoot fireballs now and are identified by $B8.
SpritePrep_LoadProperties = $0DB818

; Replace SpritePrep_Eyegore JSL call.
org $0691B6
JSL SpritePrep_EyegoreOrGoriya 

; Change jump table that was pointing to Dialogue Tester to point to Eyegore.
org $0687CB
dw #$91B6 

; Bank1E.asm (140) -> dw Sprite_DialogueTester   ; 0xB8 - debug artifact, dialogue tester
org $1E8BB1 
dw #$C79D

; Update all references to $84 (red goriya) to $B8 in logic.
org $1EC7C5
db !SPRITE_GORIYA
org $1EC801
db !SPRITE_GORIYA

; Set damage type for $B8.
org $0DB9A9 
db $00

; Bank06.asm (4593) - damage calcs
org $06EC08 
JSL DamageSprite_Goriya
NOP ; Required to cancel out an otherwise invalid instructions.

; Replace Eyegore Sprites in Room 0x10C (Mimic Cave) with new Goriya.
org $09EBCD
db $14, $07, !SPRITE_GORIYA
org $09EBD0
db $14, $08, !SPRITE_GORIYA
org $09EBD3
db $14, $0C, !SPRITE_GORIYA
org $09EBD6
db $1A, $0C, !SPRITE_GORIYA

; Replace Eyegore Sprites in Room 0x1B (POD) with Goriya.
org $09DB19
db $14, $07, !SPRITE_GORIYA
org $09DB1C
db $1C, $03, !SPRITE_GREEN_EYEGORE
org $09DB1F
db $1C, $0C, !SPRITE_GORIYA

; Replace Eyegore Sprites in Room 0x4B (POD) with Goriya.
org $09DEEB
db $04, $07, !SPRITE_GORIYA
org $09DEF4
db $08, $04, !SPRITE_GREEN_EYEGORE
org $09DEF7
db $08, $0B, !SPRITE_GORIYA

; Replace Eyegore Sprites in Room 0x6B (GT) with Goriya.
org $09E199
db $06, $0A, !SPRITE_GORIYA
org $09E19C
db $09, $06, !SPRITE_GORIYA
org $09E1A5
db $18, $03, !SPRITE_GORIYA
org $09E1AE
db $1B, $0C, !SPRITE_GORIYA
org $09E1B1
db $15, $17, !SPRITE_GORIYA
org $09E1BA
db $1B, $18, !SPRITE_GORIYA