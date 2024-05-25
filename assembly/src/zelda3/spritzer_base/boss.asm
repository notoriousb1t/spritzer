; Checks if the prize should be centered. The return of this subroutine should
; expects a BNE call indicating if the prize is center dropped.
IsPrizeCentered:
    LDA.b pub_IsHeraPrizeCentered
    CMP.b #$01
    BEQ IsPrizeCentered_HeraCheck
    RTL

; If prize is centered for Hera, check that the room is Tower of Hera boss room.
IsPrizeCentered_HeraCheck:
    LDA.w $040C
    CMP.b #$14
    RTL