section .data
    msg db 8
test:
    push rbp
    mov rbp,rsp
    mov [rbp-1],dword 100
    mov [rbp-5],dword 200
    mov edi,dword 10
    mov esi,dword 20
    mov edx,dword 30
    call deez
    pop rbp
    ret
deez:
    push rbp
    mov rbp,rsp
    mov [rbp-9],edi
    mov [rbp-13],esi
    mov [rbp-14],cl
    mov rsi,dword 1633771873
    mov rdx,8
    call print
    pop rbp
    ret
test:
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
