section .text
    global _start
_start:
    call main
    mov rdi,rax
    mov rax,60
    syscall
main:
    push rbp
    mov rbp,rsp
    mov rax,8
    imul rax,2
    add rax,rax
    mov [rbp-4],rax
    mov rax,[rbp-4]
    pop rbp
    ret
