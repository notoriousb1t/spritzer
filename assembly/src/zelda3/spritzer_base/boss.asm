SpritePrep_Blind_PrepareBattle = $1DA081
SpritePrep_Blind_Maidenless = $1DA090

; Checks if the prize should be centered. The return of this subroutine should
; expects a BNE call indicating if the prize is center dropped.
IsPrizeCentered:
    LDA.b setting_IsHeraPrizeCentered
    CMP.b #$01
    BEQ IsPrizeCentered_HeraCheck
    RTL

; If prize is centered for Hera, check that the room is Tower of Hera boss room.
IsPrizeCentered_HeraCheck:
    LDA.w $040C
    CMP.b #$14
    RTL

SpritePrep_Blind_CheckMaidenless:
    LDA !ROOM_ID
    CMP.b #$AC
    BNE SpritePrep_Blind_IsTarnished
    JML SpritePrep_Blind_PrepareBattle

SpritePrep_Blind_IsTarnished:
    JSL SpritePrep_Blind_Maidenless
    RTL