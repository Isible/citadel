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
    mov rax, 3
    imul rax, 4
    mov [rbp-4],rax
    mov rdi,[rbp-4]
    mov rax,60
    syscall
    pop rbp
    ret
