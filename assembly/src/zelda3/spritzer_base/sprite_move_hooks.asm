; ; Intercept the normal Load accumular from pointer and load from the 24 bit address instead.
; ; This only needs to happen once per room load, so it shouldn't be intensive.
; org $09C297
; JSL UnderworldSpriteList_LoadPointer
; NOP 
; NOP