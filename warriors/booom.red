;author dora
;strategy scans every few spaces for a non-zero instruction and deploys a limited-time bomb
step equ 700
length equ 4

start:  SEQ -1, 700
        JMP send
        ADD.AB #step, start
        JMP start
        DAT bomb
counter:DAT bomb + length
send:   MOV <-1, @start
        SUB.AB #1, start
        SEQ -3, counter - 1
        JMP send
        ADD.AB #step, start
        SPL @start
        JMP start
bomb:   DAT 10
        DAT 9
        DAT 8
        DAT 7