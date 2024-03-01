; Adjust oam position after drawing eyes.
org $1DD88E
JSL Moldorm_AddEyes
NOP #08

org $1DDBB3
pub_Moldorm_EyeCount:
db $07