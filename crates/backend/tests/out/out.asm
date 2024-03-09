section .text
global _start
_start:
    call main
    syscall

main:
    mov rax, 60         ; syscall number for exit
    mov rdi, 0        ; exit code 0
    ret