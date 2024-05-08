; Misery Mire's northeastern slot should be empty, but it points to its eastern neighbor.
; Fixing this so the rust project doesn't try to load the wrong sprite list.
; Darkworld v1
org $09C9E3
dw pub_OWRoomEmpty

; Darkworld v2
org $09CB03
dw pub_OWRoomEmpty