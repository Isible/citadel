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
    mov rax,dword 3
    imul rax,dword 20
    add rax,rax
    mov [rbp-4],rax
    mov rdi,[rbp-4]
    mov rax,60
    syscall
    pop rbp
    ret
