section .text
    global _start
_start:
    mov rdi,12
    mov rax,60
    syscall
