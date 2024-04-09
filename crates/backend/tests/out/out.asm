section .text
    global _start

_start:
    call main
    mov rdi, rax
    mov rax, 60
    syscall

main:
    push rbp
    mov rbp,rsp
    mov [rbp-4],dword 19
    mov [rbp-8],dword 69
    mov rax,[rbp-8]
    pop rbp
    ret 