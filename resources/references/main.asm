	.file	"main.c"
	.text
	.globl	main
	.type	main, @function
main:
.LFB0:
	pushq	%rbp
	movq	%rsp, %rbp

	; return value
	movl	$0, %eax
	
	popq	%rbp
	ret
.LFE0:
	.size	main, .-main
	.ident	"GCC: (GNU) 13.2.1 20240316 (Red Hat 13.2.1-7)"
	.section	.note.GNU-stack,"",@progbits
