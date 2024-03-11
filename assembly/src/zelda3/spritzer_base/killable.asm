; Loosely based on killable thief in Enemizer, but is generally 
; usable for making things killable that are invulnerable.
pub_Killable:
    LDA !SPRITE_ID,X
    CMP.b #$FF ; Replaced with $C4 in Rust code when this is enabled.
    BEQ Killable_EyeGore

    CMP.b #!SPRITE_GORIYA
    BEQ Killable_EyeGore

    JMP Killable_return

; Load green eyegore sprite id so we can kill the thing.
Killable_EyeGore:
    LDA !SPRITE_GREEN_EYEGORE
    JMP Killable_return

Killable_return:
    ; Restore part of algorithm that was written over.
    REP #$20
    ASL #2
    RTL