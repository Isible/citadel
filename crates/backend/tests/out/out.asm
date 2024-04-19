section .text
global _start
_start:
mov rdi,dword 9
call main
main:
push rbp
mov rbp,rsp
mov [rbp-4],rdi
mov [rbp-8],dword 100
mov rax,[rbp-4]
pop rbp
ret 