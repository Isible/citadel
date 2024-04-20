section .rodata
    LC0 db "Hellow orld", 10
section .text
    global _start
_start:
    mov rdi,dword 9
    call main
    mov rax,60
    mov rdi,0
    syscall
main:
    push rbp
    mov rbp,rsp
    mov [rbp-4],rdi
    mov [rbp-8],dword 100
    mov [rbp-12],dword 890
    mov rsi,LC0
    mov rdx,12
    call print
    mov rax,[rbp-12]
    pop rbp
    ret
print:
    mov rax,1
    mov rdi,1
    syscall
    ret
