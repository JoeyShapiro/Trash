.global _asm_write
.global _asm_read
.global _bzero

_asm_write:
    mov x16, #4         // write syscall
    svc #0x80           
    ret

_asm_read:
    mov x16, #3         // read syscall
    svc #0x80
    ret

_bzero:
    cbz x1, 2f      // if length is 0, return
1:  strb wzr, [x0], #1  // store 0 and increment
    subs x1, x1, #1     // decrement length
    b.ne 1b        // loop if not zero
2:  ret
