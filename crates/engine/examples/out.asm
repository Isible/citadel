section .text
global _start
_start:
mov rdi,dword 1009
call main
mov rdi, rax
mov rax, 60
syscall
main:
push rbp
mov rbp,rsp
mov [rbp-4],rdi
mov [rbp-8],dword 9
mov [rbp-12],dword 0
mov [rbp-16],dword 100
mov [rbp-20],dword 1000000
mov rax,[rbp-16]
pop rbp
ret 