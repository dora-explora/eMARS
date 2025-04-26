;redcode

;name		DwarfScout
;version	2
;author		Marcus
;date		2004-Feb-05
		
;strategy	waits for dwarfy attack and dodges while dwarfing


	ORG  EntryPoint


FSIZE	EQU	9	; field size
FDIST	EQU	473	; field distance

DSTEP	EQU	5	; dwarf step size


fUptr		DAT	#0,	#0	;watch Pointer to Up-field
dUptr		DAT	#0,	#0	;watch dwarf Up pointer
rlSptr		DAT	#0,	#0	;watch pointers for relocation
rlTptr		DAT	#0,	#0	;watch

	EntryPoint
		MOV.I	dwarfLoop,	dwarfKillSave

FIRST	; relocate begins with the following instruction
initFgenerate	MOV.AB	#-1*FDIST,	fUptr
		MOV.AB	#FDIST,		fDptr

generateFloop	MOV.I	FCODEu,		<fUptr
		MOV.I	FCODEd,		>fDptr
		CMP.AB	#FDIST+FSIZE,	fDptr
		JMP	generateFloop

		SPL	dwarfInit

restartVerify	MOV.AB	#-1*FDIST,	fUptr
		MOV.AB	#FDIST,		fDptr
verifyFloop	CMP.I	FCODEu,		<fUptr
		JMP	alertUfield
		CMP.I	FCODEd,		>fDptr
		JMP	alertDfield
		CMP.AB	#FDIST+FSIZE,	fDptr
		JMP	verifyFloop
		JMP	restartVerify

dwarfKillSave	DAT.I	#0
dwarfInit	MOV.I	dwarfKillSave,	dwarfLoop
		MOV.AB	#-MINDISTANCE,	dUptr
		MOV.AB	#MINDISTANCE,	dDptr
dwarfLoop	MOV.I	BOMBu,		@dUptr
		MOV.I	BOMBd,		@dDptr
		SUB.AB	#DSTEP,		dUptr
		ADD.AB	#DSTEP,		dDptr
		JMP	dwarfLoop

alertUfield	MOV.A	#-FDIST-MINDISTANCE,	rlDelta
		JMP	relocate

alertDfield	MOV.A	#FDIST+MINDISTANCE,	rlDelta

relocate	MOV	BOMBd,		dwarfLoop  ; save CPU time
		MOV.AB	#1+LAST-rlSptr,	rlSptr	; initialize pointers
		MOV.AB	#2+LAST-FIRST,	rlTptr
		ADD.AB	rlDelta,	rlTptr	; add distance param
relocLoop	MOV.I	<rlSptr,	<rlTptr
		CMP.B	rlSptr,		#FIRST-rlSptr
		JMP	relocLoop
		JMP	@rlTptr

BOMBu FCODEu	DAT.X	<667,	$766	;watch recognition code Up-field
LAST	; relocate area ends with next instruction
BOMBd FCODEd	DAT.X	<776,	$677	;watch recognition code Down-field

rlDelta		DAT	#0		;watch relocation distance
dDptr		DAT	#0,	#0	;watch dwarf Down pointer
fDptr		DAT	#0,	#0	;watch pointer to Down-field

		END