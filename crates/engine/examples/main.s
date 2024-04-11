section .rodata
LC0 db "l{"Hello World"}", 10
section .text
global _start
_start:
mov rsi,LC0
mov rdx,17
call print
print:
mov rax,1
mov rdi,1
syscall 
ret 