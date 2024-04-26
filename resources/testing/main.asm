section .rodata
    LC0 db "Hello, world!", 10

section .text
    global _start

_start:
    ; write syscall
    mov rax, 1
    mov rdi, 1
    mov rsi, LC0
    mov rdx, 14
    syscall

    mov eax, 60
    xor edi, edi
    syscall
 