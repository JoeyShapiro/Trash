.global _asm_write
_asm_write:
    mov x16, #4         // write syscall
    svc #0x80           
    ret