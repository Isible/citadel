section .rodata
LC0 db "hello world", 10
section .text
global _start
_start:
call main
main:
push rbp
mov rbp,rsp
mov [rbp-4],dword 0
mov rsi,LC0
mov rdx,12
call print
mov rax,[rbp-4]
pop rbp
ret 
print:
mov rax,1
mov rdi,1
syscall 
ret 