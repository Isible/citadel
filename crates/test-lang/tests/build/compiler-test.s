section .data
testing db "Hello World"
section .text
global _start
_start:    
call main
main:    
mov rax,1
mov rdi,1
mov rsi,testing
mov rdx,11
syscall 
ret 