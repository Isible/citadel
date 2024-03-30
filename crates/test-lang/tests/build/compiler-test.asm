section .rodata
    LC0 db "Hello World", 10
    LC1 db "I am a dino", 10
    LC2 db "and a cat", 10
    LC3 db "as well as a pig", 10

section .text
    global _start

_start:
    call main
    mov rax, 60
    syscall

main:
    mov rsi,LC0
    mov rdx,12
    call print

    mov rsi,LC1
    mov rdx,12
    call print

    mov rsi,LC2
    mov rdx,10
    call print

    mov rsi,LC3
    mov rdx,17
    call print

    mov rdi, 0
    ret

print:
    mov rax,1
    mov rdi,1
    syscall 
    ret 