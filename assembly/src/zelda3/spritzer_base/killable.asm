; Loosely based on killable thief in Enemizer, but is generally 
; usable for making things killable that are invulnerable.
Killable:
    LDA !SPRITE_ID,X
    CMP.b #$C4
    BEQ Killable_Thief

    CMP.b #!SPRITE_GORIYA
    BEQ Killable_EyeGore_Green

    JMP Killable_return

Killable_Thief:
    LDA.b setting_EnableKillableThief
    CMP.b #$01
    BEQ Killable_EyeGore_Green
    JMP Killable_return

Killable_EyeGore_Green:
    LDA #!SPRITE_GREEN_EYEGORE
    JMP Killable_return

Killable_return:
    ; Restore what was written over.
    REP #$20
    ASL #2
    RTL