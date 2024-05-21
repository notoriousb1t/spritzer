; Underworld sprites are moved to 2A/6A, This function 
UnderworldSpriteList_LoadPointer:
    ; This routine loads a 24 bit pointer into the accumulator,
    ; so that the vanilla routine can load from a different bank other than bank 9.
    LDA pub_UnderworldSpriteList_Pointers,Y
    STA $00
    LDA pub_UnderworldSpriteList_Pointers+1, Y
    STA $01
    LDA pub_UnderworldSpriteList_Pointers+2, Y
    STA $02
    RTL

; Placeholder for where the new pointer table is added.
pub_UnderworldSpriteList_Pointers:
    NOP