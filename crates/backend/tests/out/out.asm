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
    mov [rbp-4],dword 1819042114
    mov rax,0
    pop rbp
    ret
