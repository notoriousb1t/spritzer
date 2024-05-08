SpritePrep_Blind_CheckMaidenless:
    LDA !ROOM_ID
    CMP.b #$AC
    BNE SpritePrep_Blind_IsTarnished
    JML SpritePrep_Blind_PrepareBattle

SpritePrep_Blind_IsTarnished:
    JSL SpritePrep_Blind_Maidenless
    RTL