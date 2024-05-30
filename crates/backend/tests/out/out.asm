section .data
    msg db 9
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
    mov rax,8029759184975979880
    mov [rbp-8],rax
    mov [msg], rax
    mov [msg+8], byte 0xA
    mov rax, 1
    mov rdi, 1
    mov rsi, msg
    mov rdx, 9
    syscall
    mov rax,[rbp-8]
    pop rbp
    ret
