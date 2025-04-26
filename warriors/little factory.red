;redcode-94b
;name little factory
;kill little factory
;author marcus
;strategy creates lots of littles
;assert 1

;version 2
;date 2004-02-05
		

DISTANCE	EQU	CORESIZE/17+2

		ORG	factory

BOMB	lptr	DAT	$0,		$0
little		MOV	BOMB,		<lptr
		DJN	little,		lptr

fptr		DAT	$0,		$0
factory		SUB	#DISTANCE,	fptr
		MOV	BOMB,		>fptr
		MOV	little,		>fptr
		MOV	little+1,	@fptr
		SPL	<fptr
		JMP	factory