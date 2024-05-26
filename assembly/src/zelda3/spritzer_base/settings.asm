; This should be used to toggle on and off features.
; There is enough room for 36 bytes. This is the largest
; block in bank 1. If this exceeds 36 bytes, it needs to be moved.
org $01B53C

; <Whether or not Thief is killable.
setting_EnableKillableThief:
 db $00

; Used to configure Spritzer ASM code.
setting_IsHeraPrizeCentered:
 db $01
