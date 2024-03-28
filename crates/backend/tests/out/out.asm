section .text
global _start
_start:    
mov rax,1
mov rdi,1
mov rsi,testing
mov rdx,11
syscall 
section .data
testing db "Hello world"