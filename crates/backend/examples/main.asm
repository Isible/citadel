section .text
    global _start

_start:
    mov r8,  60

    ; Exit the program
    mov rax, r8        ; sys_exit syscall number
    xor rdi, rdi       ; exit code 0
    syscall            ; invoke the syscall