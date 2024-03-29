section .text
    global _start

_start:    
    call main

    mov rax, 60
    call testt

main:    
    mov rax, 1
    mov rdi, 1
    mov rsi, testing
    mov rdx, 10
    syscall
    ret

testt:
    mov rdi, 0
    syscall

section .data
    testing db "Deez nuts", 0xa
