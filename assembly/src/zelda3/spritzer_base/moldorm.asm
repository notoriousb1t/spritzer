; Based on Enemizer code.

Moldorm_AddEyes: {
	PHX
	LDX.b pub_Moldorm_EyeCount
	TAX

	; Draw eyes in a circle. Decrement from number of eyes - 1.
	.next_eye
		LDA !OAM_POS_LOW
		CLC
		ADC.w #$0004
		STA !OAM_POS_LOW

		LDA !OAM_POS_HIGH
		CLC
		ADC.w #$0001
		STA !OAM_POS_HIGH

		DEX
		BPL .next_eye
	
	PLX
	RTL
}