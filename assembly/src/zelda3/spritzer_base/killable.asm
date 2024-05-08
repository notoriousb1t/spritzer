; Loosely based on killable thief in Enemizer, but is generally 
; usable for making things killable that are invulnerable.
pub_Killable:
    LDA !SPRITE_ID,X
    CMP.b #$FF ; Replaced with $C4 in Rust code when this is enabled.
    BEQ Killable_EyeGore_Green

    CMP.b #!SPRITE_GORIYA
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