;redcode-94b
;assert 1
;name scanner1
;version 1
;author marcus
;date 2006-Nov-27

MINDISTANCE EQU 100

loop		sub.ab	#8,		+1
ptr		jmz.f	loop,		-MINDISTANCE
attack		add	#33,		ptr
		mov	little,		>ptr
		mov	little+1,	@ptr
		spl	@ptr,		<loop-1
		sub	#33,		ptr
		jmp	loop,		}loop

little		mov	-1,	<-1
		djn	-1,	-2