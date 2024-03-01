SpritePrep_Blind_CheckMaidenless:
    LDA.w !ROOM_ID
    CMP.b #!UW_ROOM_THIEVES_TOWN_BLIND_THE_THIEF_BOSS
    
    BNE SpritePrep_Blind_IsTarnished
    JML SpritePrep_Blind_PrepareBattle

SpritePrep_Blind_IsTarnished:
    JSL SpritePrep_Blind_Maidenless
    RTL