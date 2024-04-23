section .rodata
LC0 db "Hello World", 10
section .text
global _start
_start:
mov rsi,LC0
mov rdx,12
call print
test:
push rbp
mov rbp,rsp
mov [rbp-4],edi
mov [rbp-5],bl
mov [rbp-7],cx
mov rax,0
pop rbp
ret
print:
mov rax,1
mov rdi,1
syscall
ret