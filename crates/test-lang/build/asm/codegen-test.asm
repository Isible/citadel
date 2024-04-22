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
    mov rdi,2
    mov rsi,3
    mov rdx,12
    call add
    mov rax,rax
    pop rbp
    ret
add:
    push rbp
    mov rbp,rsp
    mov [rbp-4],rdi
    mov [rbp-8],rsi
    mov [rbp-12],rdx
    mov rax,[rbp-4]
    add rax,[rbp-8]
    mov rax,rax
    add rax,[rbp-12]
    mov rax,rax
    pop rbp
    ret
