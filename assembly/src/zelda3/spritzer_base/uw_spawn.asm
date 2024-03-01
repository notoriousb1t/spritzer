Underworld_LoadSingleSprite_override: {
    NOP ; Add sprite spawning logic for the underworld.
    ; Continue with .spawn
    .exit
        LDX.b $02
        DEY
        DEY
        RTL
}