; Intercept JSL to SpritePrep_Blind_PrepareBattle
org $0688CA
JSL SpritePrep_Blind_CheckMaidenless

; Bank06.asm (4876) - .setThiefKillable in place of not absorbable.
org $06EDA6
JSL Killable

; Replaces a comparison for hera tower and a compare and routes to 
; a new subroutine.
org $098C0D
JSL IsPrizeCentered
NOP

; Adjust oam position after drawing Moldorm eyes.
org $1DD88E
JSL Moldorm_AddEyes
NOP #08

; The number of eyes Moldorm has.
org $1DDBB3
setting_MoldormEyeCount:
db $07

; ; Intercept the normal Load accumular from pointer and load from the 24 bit address instead.
; ; This only needs to happen once per room load, so it shouldn't be intensive.
; org $09C297
; JSL UnderworldSpriteList_LoadPointer
; NOP 
; NOP