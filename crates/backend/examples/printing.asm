section .data
    hello db 'Hello, World!', 0xa

section .text
    global _start

_start:
    ; write syscall
    mov rax, 1
    ; stdout file descriptor
    mov rdi, 1
    ; address of string to output
    mov rsi, hello
    ; length of string to output
    mov rdx, 14
    syscall

    ; exit syscall
    mov rax, 60
    xor rdi, rdi
    syscall
