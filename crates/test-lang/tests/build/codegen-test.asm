section .text
global _start
_start:
call main
main:
push rbp
mov rbp,rsp
mov [rbp-4],dword 420
mov [rbp-8],dword 69
mov rax,[rbp-8]
pop rbp
ret 