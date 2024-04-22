section .text
    global _start
_start:
    call main
    mov rdi,rax
    mov rax,60
    syscall
main:
    push rbp
    mov rbp,rsp
    mov edi,2
    mov esi,3
    mov edx,12
    call addd
    mov rax,rax
    pop rbp
    ret
addd:
    push rbp
    mov rbp,rsp
    mov [rbp-8],al
    mov [rbp-16],bl
    mov [rbp-24],cl
    mov rax,[rbp-8]
    add rax,[rbp-16]
    mov rax,rax
    add rax,[rbp-24]
    mov rax,rax
    pop rbp
    ret
