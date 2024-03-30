section .rodata
    LC0 db "Hello world", 10
    LC1 db "Deez nuts", 10
    LC2 db "Amongus", 10
    LC3 db "Lololololol", 10

section .text
    global _start

_start:
    mov rsi,LC0
    mov rdx,12
    call print
    
    mov rsi,LC1
    mov rdx,10
    call print
    
    mov rsi,LC2
    mov rdx,8
    call print
    
    mov rsi,LC3
    mov rdx,12
    call print

    mov rax, 60
    xor rdi, rdi
    syscall

print:
    mov rax,1
    mov rdi,1
    syscall 
    ret 