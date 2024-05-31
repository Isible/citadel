section .data
    msg db 8
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
    mov rax,751942187145258344
    mov [rbp-8],rax
    mov rsi,[rbp-8]
    mov rdx,8
    call print
    mov rax,0
    pop rbp
    ret
print:
    mov [msg],rsi
    mov rsi,msg
    mov rax,1
    mov rdi,1
    syscall
    ret
