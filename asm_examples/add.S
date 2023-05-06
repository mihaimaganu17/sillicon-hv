    .section __TEXT,__text
    .globl _main
    .p2align 2

_main:
    mov x0, #2
    mov x1, #9
    add x0, x0, x1
    ret

