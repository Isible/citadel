section .text
global _start
_start:
call main
main:
push rbp
mov rbp,rsp
mov [rbp-4],9
mov [rbp-8],0
mov [rbp-12],100
mov [rbp-16],1000000
mov rdi,0
mov rax,60
syscall
mov rax,0
pop rbp
ret