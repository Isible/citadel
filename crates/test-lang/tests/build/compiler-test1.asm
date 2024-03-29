section .text
    global _start

_start:    
    call main

main:    
    mov rax,1
    mov rdi,1
    mov rsi,testing
    mov rdx,11
    syscall 
    ret
section .data
    testing db "Hello World"