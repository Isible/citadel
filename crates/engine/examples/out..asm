section .text
global _start
_start:
call main
main:
push rbp
mov rbp,rsp
mov [rbp-4],dword 9
mov [rbp-8],dword 0
mov [rbp-12],dword 100
mov [rbp-16],dword 1000000
mov rax,60
mov rdi,0
syscall
ret