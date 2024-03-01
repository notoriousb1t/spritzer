; Sourced from Enemizer, but with significant changes.
;
; TODO: Switch this to use subtypes instead of depending on mimic cave room as signal.
; Hypothetically there are up to 16 subtypes that can be set in !SPRITE_SUBTYPE1, so at least
; to start, setting a value of $1 could help distinquish between red and green and might be
; the best way to do variants for other types of enemies/creatures.


SpritePrep_EyegoreOrMimicNew: {
    ; Load sprite at x index and perform special loading if it is a a mimic code.
    LDA !SPRITE_ID,X
    CMP.b #!SPRITE_MIMIC 
    BEQ .then_load_mimic
    RTL

    .then_load_mimic
        LDA.w !ROOM_ID
        CMP.b #$0C ; TODO: This should probably do a CMP.w #10C instead.
        BEQ .then_its_not_easy_being_green

        ; Use Red Eyegore for all properties.
        LDA #!SPRITE_RED_EYEGORE 
        JMP .then_continue

    ; Use Green Eyegore props in Mimic Cave.
    .then_its_not_easy_being_green
        LDA #!SPRITE_GREEN_EYEGORE ; Temporarily set to Green Eyegore during property loading.

    .then_continue
        STA !SPRITE_ID,X
        JSL SpritePrep_LoadProperties
        
        LDA #!SPRITE_MIMIC
        STA !SPRITE_ID,X 
        
        LDA !SPRITE_SETTINGS_3,X
        AND #$FB
        ORA #$80
        STZ.w !SPRITE_SETTINGS_3,X
        
        INC.w !SPRITE_STATE,X
        LDA.w !SPRITE_ID,X
        STZ.w !SPRITE_SETTINGS_3,X

        RTL
}

resetSprite_Mimic: {
    LDA !SPRITE_ID,X
    CMP.b #!SPRITE_MIMIC
    BNE .then_continue

    ; Preserve green mimics in mimic cave.
    LDA.w !ROOM_ID
    CMP.b #$0C ; TODO: This should probably do a CMP.w #10C instead.
    BEQ .then_its_not_easy_being_green

    ; Use Red Eyegore for collisions.
    LDA #!SPRITE_RED_EYEGORE
    JMP .then_continue

    ; Use Green Eyegore for collisions in Mimic Cave.
    .then_its_not_easy_being_green
        LDA #!SPRITE_GREEN_EYEGORE

    ; restore comparison that this JSL wrote over.
    .then_continue
        ; Store the temporary sprite value.
        STA !SPRITE_ID,X
        ; Maybe this is redundant?
        LDA !SPRITE_ID,X
        CMP.b #!SPRITE_AGAHNIM
        RTL
}

