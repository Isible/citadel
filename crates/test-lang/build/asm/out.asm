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
    mov [rbp-1],dword 100
    mov [rbp-5],dword 200
    mov rdi,dword 69
    mov rax,60
    syscall
    mov [rbp-6],dword 0
    mov edi,dword 10
    mov esi,dword 20
    mov edx,dword 30
    call deez
    pop rbp
    ret
deez:
    push rbp
    mov rbp,rsp
    mov [rbp-10],edi
    mov [rbp-14],esi
    mov [rbp-15],cl
    mov rsi,dword 1633771873
    mov rdx,8
    call print
    pop rbp
    ret
test1:
    push rbp
    mov rbp,rsp
    pop rbp
    ret
print:
    mov [msg],rsi
    mov rsi,msg
    mov rax,1
    mov rdi,1
    syscall
    ret
