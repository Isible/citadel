section .text
    global _start

_start:
    ; Exit the program
    mov rax, 60        ; sys_exit syscall number
    xor rdi, rdi       ; exit code 0
    syscall            ; invoke the syscall