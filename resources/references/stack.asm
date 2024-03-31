section .text
    global _start

_start:
    call myfunc
    mov rax, 60
    syscall

myfunc:
    ; setup stack frame
    push rbp
    mov rbp, rsp

    ; do stuff
    mov [rbp-4], dword 10
    mov rdi, [rbp-4]

    ; restore stack frame and return
    pop rbp
    ret