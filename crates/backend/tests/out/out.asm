section .data
    string db 16

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
    mov rax,1819043176
    mov [rbp-8], rax
    mov [string], rax
    mov rax, 1
    mov rdi, 1
    mov rsi, string
    mov rdx, 16
    syscall
    mov rax,0
    pop rbp
    ret
