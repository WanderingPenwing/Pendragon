	.text
	.file	"test_mieux.ll"
	.globl	texte_centaine                  # -- Begin function texte_centaine
	.p2align	4, 0x90
	.type	texte_centaine,@function
texte_centaine:                         # @texte_centaine
	.cfi_startproc
# %bb.0:                                # %entry
	cmpl	$9, %edi
	jg	.LBB0_6
# %bb.1:                                # %pas_infini
	testl	%edi, %edi
	je	.LBB0_6
# %bb.2:                                # %centaine
	pushq	%rax
	.cfi_def_cfa_offset 16
	cmpl	$1, %edi
	jne	.LBB0_4
# %bb.3:                                # %pas_nombre
	leaq	.Lvide(%rip), %rdi
	jmp	.LBB0_5
.LBB0_6:                                # %infini
	.cfi_def_cfa_offset 8
	movq	separateurs@GOTPCREL(%rip), %rax
	movq	48(%rax), %rax
	retq
.LBB0_4:                                # %nombre
	.cfi_def_cfa_offset 16
	movslq	%edi, %rax
	movq	petits_nombres@GOTPCREL(%rip), %rcx
	movq	(%rcx,%rax,8), %rdi
	leaq	.Ltiret(%rip), %rsi
	callq	concat_strings@PLT
	movq	%rax, %rdi
.LBB0_5:                                # %cent
	movq	dizaine@GOTPCREL(%rip), %rax
	movq	(%rax), %rsi
	callq	concat_strings@PLT
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end0:
	.size	texte_centaine, .Lfunc_end0-texte_centaine
	.cfi_endproc
                                        # -- End function
	.globl	texte_unite                     # -- Begin function texte_unite
	.p2align	4, 0x90
	.type	texte_unite,@function
texte_unite:                            # @texte_unite
	.cfi_startproc
# %bb.0:                                # %entry
	cmpl	$19, %edi
	jg	.LBB1_2
# %bb.1:                                # %unite
	movslq	%edi, %rax
	movq	petits_nombres@GOTPCREL(%rip), %rcx
	movq	(%rcx,%rax,8), %rax
	retq
.LBB1_2:                                # %infini
	movq	separateurs@GOTPCREL(%rip), %rax
	movq	48(%rax), %rax
	retq
.Lfunc_end1:
	.size	texte_unite, .Lfunc_end1-texte_unite
	.cfi_endproc
                                        # -- End function
	.globl	texte_dizaine                   # -- Begin function texte_dizaine
	.p2align	4, 0x90
	.type	texte_dizaine,@function
texte_dizaine:                          # @texte_dizaine
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%r15
	.cfi_def_cfa_offset 16
	pushq	%r14
	.cfi_def_cfa_offset 24
	pushq	%rbx
	.cfi_def_cfa_offset 32
	.cfi_offset %rbx, -32
	.cfi_offset %r14, -24
	.cfi_offset %r15, -16
	cmpl	$19, %edi
	jg	.LBB2_2
# %bb.1:                                # %unite
	callq	texte_unite@PLT
	jmp	.LBB2_4
.LBB2_2:                                # %dizaine
	movslq	%edi, %rax
	imulq	$1717986919, %rax, %r15         # imm = 0x66666667
	movq	%r15, %rax
	shrq	$63, %rax
	sarq	$34, %r15
	addl	%eax, %r15d
	leal	(%r15,%r15), %eax
	leal	(%rax,%rax,4), %eax
	subl	%eax, %edi
	movslq	%r15d, %rax
	movq	dizaine@GOTPCREL(%rip), %rcx
	movq	(%rcx,%rax,8), %rcx
	je	.LBB2_3
# %bb.5:                                # %pitet-et
	movl	%edi, %ebx
	cmpl	$1, %edi
	sete	%al
	cmpl	$8, %r15d
	setl	%dl
	mulb	%dl
	cmpb	$1, %al
	jne	.LBB2_7
# %bb.6:                                # %affiche-et
	leaq	.Let(%rip), %rsi
	movq	%rcx, %rdi
	callq	concat_strings@PLT
	movq	%rax, %rcx
.LBB2_7:                                # %pitet-special
	leaq	.Ltiret(%rip), %rsi
	movq	%rcx, %rdi
	callq	concat_strings@PLT
	movq	%rax, %r14
	cmpl	$7, %r15d
	sete	%al
	cmpl	$9, %r15d
	sete	%cl
	addb	%al, %cl
	testb	$1, %cl
	je	.LBB2_9
# %bb.8:                                # %unite-special
	addl	$10, %ebx
.LBB2_9:                                # %unite-simple
	movl	%ebx, %edi
	callq	texte_unite@PLT
	movq	%r14, %rdi
	movq	%rax, %rsi
	callq	concat_strings@PLT
	jmp	.LBB2_4
.LBB2_3:                                # %juste_dizaine
	movq	%rcx, %rax
.LBB2_4:                                # %juste_dizaine
	popq	%rbx
	.cfi_def_cfa_offset 24
	popq	%r14
	.cfi_def_cfa_offset 16
	popq	%r15
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end2:
	.size	texte_dizaine, .Lfunc_end2-texte_dizaine
	.cfi_endproc
                                        # -- End function
	.globl	texte_petit_nombre              # -- Begin function texte_petit_nombre
	.p2align	4, 0x90
	.type	texte_petit_nombre,@function
texte_petit_nombre:                     # @texte_petit_nombre
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%r14
	.cfi_def_cfa_offset 16
	pushq	%rbx
	.cfi_def_cfa_offset 24
	pushq	%rax
	.cfi_def_cfa_offset 32
	.cfi_offset %rbx, -24
	.cfi_offset %r14, -16
	movslq	%edi, %rax
	imulq	$1374389535, %rax, %rdi         # imm = 0x51EB851F
	movq	%rdi, %rcx
	shrq	$63, %rcx
	sarq	$37, %rdi
	addl	%ecx, %edi
	imull	$100, %edi, %ecx
	movl	%eax, %ebx
	subl	%ecx, %ebx
	cmpl	$99, %eax
	jg	.LBB3_2
# %bb.1:                                # %pas_centaine
	leaq	.Lvide(%rip), %r14
	jmp	.LBB3_4
.LBB3_2:                                # %centaine
                                        # kill: def $edi killed $edi killed $rdi
	callq	texte_centaine@PLT
	testl	%ebx, %ebx
	jle	.LBB3_5
# %bb.3:                                # %separateur
	leaq	.Ltiret(%rip), %rsi
	movq	%rax, %rdi
	callq	concat_strings@PLT
	movq	%rax, %r14
.LBB3_4:                                # %dizaine
	movl	%ebx, %edi
	callq	texte_dizaine@PLT
	movq	%r14, %rdi
	movq	%rax, %rsi
	callq	concat_strings@PLT
.LBB3_5:                                # %juste_centaine
	addq	$8, %rsp
	.cfi_def_cfa_offset 24
	popq	%rbx
	.cfi_def_cfa_offset 16
	popq	%r14
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end3:
	.size	texte_petit_nombre, .Lfunc_end3-texte_petit_nombre
	.cfi_endproc
                                        # -- End function
	.globl	mille_puissance                 # -- Begin function mille_puissance
	.p2align	4, 0x90
	.type	mille_puissance,@function
mille_puissance:                        # @mille_puissance
	.cfi_startproc
# %bb.0:                                # %entry
	testl	%edi, %edi
	je	.LBB4_1
# %bb.2:                                # %recursion
	pushq	%rax
	.cfi_def_cfa_offset 16
	decl	%edi
	callq	mille_puissance@PLT
	imulq	$1000, %rax, %rax               # imm = 0x3E8
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.LBB4_1:                                # %zero
	movl	$1, %eax
	retq
.Lfunc_end4:
	.size	mille_puissance, .Lfunc_end4-mille_puissance
	.cfi_endproc
                                        # -- End function
	.globl	log_mille                       # -- Begin function log_mille
	.p2align	4, 0x90
	.type	log_mille,@function
log_mille:                              # @log_mille
	.cfi_startproc
# %bb.0:                                # %entry
	cmpq	$999, %rdi                      # imm = 0x3E7
	jg	.LBB5_2
# %bb.1:                                # %zero
	xorl	%eax, %eax
	retq
.LBB5_2:                                # %recursion
	pushq	%rax
	.cfi_def_cfa_offset 16
	movabsq	$2361183241434822607, %rcx      # imm = 0x20C49BA5E353F7CF
	movq	%rdi, %rax
	imulq	%rcx
	movq	%rdx, %rdi
	shrq	$63, %rdi
	sarq	$7, %rdx
	addq	%rdx, %rdi
	callq	log_mille@PLT
	incl	%eax
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end5:
	.size	log_mille, .Lfunc_end5-log_mille
	.cfi_endproc
                                        # -- End function
	.globl	texte_nombre                    # -- Begin function texte_nombre
	.p2align	4, 0x90
	.type	texte_nombre,@function
texte_nombre:                           # @texte_nombre
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rbp
	.cfi_def_cfa_offset 16
	pushq	%r15
	.cfi_def_cfa_offset 24
	pushq	%r14
	.cfi_def_cfa_offset 32
	pushq	%rbx
	.cfi_def_cfa_offset 40
	pushq	%rax
	.cfi_def_cfa_offset 48
	.cfi_offset %rbx, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.cfi_offset %rbp, -16
	testq	%rdi, %rdi
	je	.LBB6_1
# %bb.2:                                # %nombre
	movq	%rdi, %rbx
	callq	log_mille@PLT
	movl	%eax, %ebp
	movl	%eax, %edi
	callq	mille_puissance@PLT
	movq	%rax, %rcx
	movq	%rbx, %rax
	cqto
	idivq	%rcx
	movq	%rdx, %rbx
	movq	%rax, %r15
	movl	%r15d, %edi
	callq	texte_petit_nombre@PLT
	testl	%ebp, %ebp
	je	.LBB6_7
# %bb.3:                                # %separateur
	leaq	.Ltiret(%rip), %r14
	movq	%rax, %rdi
	movq	%r14, %rsi
	callq	concat_strings@PLT
	movslq	%ebp, %rcx
	movq	separateurs@GOTPCREL(%rip), %rdx
	movq	(%rdx,%rcx,8), %rsi
	movq	%rax, %rdi
	callq	concat_strings@PLT
	cmpl	$2, %r15d
	setl	%cl
	cmpl	$2, %ebp
	setl	%dl
	addb	%cl, %dl
	testb	$1, %dl
	jne	.LBB6_5
# %bb.4:                                # %pluriel
	leaq	.Ls(%rip), %rsi
	movq	%rax, %rdi
	callq	concat_strings@PLT
.LBB6_5:                                # %pitet-recursion
	testq	%rbx, %rbx
	je	.LBB6_7
# %bb.6:                                # %recursion
	movq	%rax, %rdi
	movq	%r14, %rsi
	callq	concat_strings@PLT
	movq	%rax, %r14
	movq	%rbx, %rdi
	callq	texte_nombre@PLT
	movq	%r14, %rdi
	movq	%rax, %rsi
	callq	concat_strings@PLT
	jmp	.LBB6_7
.LBB6_1:                                # %affiche_zero
	movq	petits_nombres@GOTPCREL(%rip), %rax
	movq	(%rax), %rax
.LBB6_7:                                # %petit
	addq	$8, %rsp
	.cfi_def_cfa_offset 40
	popq	%rbx
	.cfi_def_cfa_offset 32
	popq	%r14
	.cfi_def_cfa_offset 24
	popq	%r15
	.cfi_def_cfa_offset 16
	popq	%rbp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end6:
	.size	texte_nombre, .Lfunc_end6-texte_nombre
	.cfi_endproc
                                        # -- End function
	.globl	texte_booleen                   # -- Begin function texte_booleen
	.p2align	4, 0x90
	.type	texte_booleen,@function
texte_booleen:                          # @texte_booleen
	.cfi_startproc
# %bb.0:
                                        # kill: def $edi killed $edi def $rdi
	andl	$1, %edi
	movq	booleen@GOTPCREL(%rip), %rax
	movq	(%rax,%rdi,8), %rax
	retq
.Lfunc_end7:
	.size	texte_booleen, .Lfunc_end7-texte_booleen
	.cfi_endproc
                                        # -- End function
	.globl	concat_strings                  # -- Begin function concat_strings
	.p2align	4, 0x90
	.type	concat_strings,@function
concat_strings:                         # @concat_strings
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rbp
	.cfi_def_cfa_offset 16
	pushq	%r15
	.cfi_def_cfa_offset 24
	pushq	%r14
	.cfi_def_cfa_offset 32
	pushq	%r13
	.cfi_def_cfa_offset 40
	pushq	%r12
	.cfi_def_cfa_offset 48
	pushq	%rbx
	.cfi_def_cfa_offset 56
	pushq	%rax
	.cfi_def_cfa_offset 64
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.cfi_offset %rbp, -16
	movq	%rsi, %rbx
	movq	%rdi, %r14
	callq	strlen@PLT
	movq	%rax, %r15
	movq	%rbx, %rdi
	callq	strlen@PLT
	movq	%rax, %r12
	leaq	(%r15,%rax), %rbp
	leaq	1(%r15,%rax), %rdi
	callq	malloc@PLT
	movq	%rax, %r13
	movq	%rax, %rdi
	movq	%r14, %rsi
	movq	%r15, %rdx
	callq	memcpy@PLT
	addq	%r13, %r15
	movq	%r15, %rdi
	movq	%rbx, %rsi
	movq	%r12, %rdx
	callq	memcpy@PLT
	movb	$0, (%r13,%rbp)
	movq	%r13, %rax
	addq	$8, %rsp
	.cfi_def_cfa_offset 56
	popq	%rbx
	.cfi_def_cfa_offset 48
	popq	%r12
	.cfi_def_cfa_offset 40
	popq	%r13
	.cfi_def_cfa_offset 32
	popq	%r14
	.cfi_def_cfa_offset 24
	popq	%r15
	.cfi_def_cfa_offset 16
	popq	%rbp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end8:
	.size	concat_strings, .Lfunc_end8-concat_strings
	.cfi_endproc
                                        # -- End function
	.globl	main                            # -- Begin function main
	.p2align	4, 0x90
	.type	main,@function
main:                                   # @main
	.cfi_startproc
# %bb.0:
	pushq	%r15
	.cfi_def_cfa_offset 16
	pushq	%r14
	.cfi_def_cfa_offset 24
	pushq	%r12
	.cfi_def_cfa_offset 32
	pushq	%rbx
	.cfi_def_cfa_offset 40
	pushq	%rax
	.cfi_def_cfa_offset 48
	.cfi_offset %rbx, -40
	.cfi_offset %r12, -32
	.cfi_offset %r14, -24
	.cfi_offset %r15, -16
	leaq	.Lformat_str(%rip), %rbx
	leaq	.Lnewline(%rip), %r14
	leaq	.Lvide(%rip), %r15
	movl	$9, %edi
	callq	texte_nombre@PLT
	movq	%r15, %rdi
	movq	%rax, %rsi
	callq	concat_strings@PLT
	movq	%rbx, %rdi
	movq	%rax, %rsi
	xorl	%eax, %eax
	callq	printf@PLT
	movq	%rbx, %rdi
	movq	%r14, %rsi
	xorl	%eax, %eax
	callq	printf@PLT
	movl	$29, %edi
	callq	texte_nombre@PLT
	movq	%r15, %rdi
	movq	%rax, %rsi
	callq	concat_strings@PLT
	movq	%rbx, %rdi
	movq	%rax, %rsi
	xorl	%eax, %eax
	callq	printf@PLT
	movq	%rbx, %rdi
	movq	%r14, %rsi
	xorl	%eax, %eax
	callq	printf@PLT
	movl	$9126, %edi                     # imm = 0x23A6
	callq	texte_nombre@PLT
	movq	%r15, %rdi
	movq	%rax, %rsi
	callq	concat_strings@PLT
	movq	%rbx, %rdi
	movq	%rax, %rsi
	xorl	%eax, %eax
	callq	printf@PLT
	movq	%rbx, %rdi
	movq	%r14, %rsi
	xorl	%eax, %eax
	callq	printf@PLT
	leaq	".Ltexte_global-0"(%rip), %rsi
	movq	%r15, %rdi
	callq	concat_strings@PLT
	movq	%rax, %r12
	xorl	%edi, %edi
	callq	texte_booleen@PLT
	movq	%r12, %rdi
	movq	%rax, %rsi
	callq	concat_strings@PLT
	movq	%rbx, %rdi
	movq	%rax, %rsi
	xorl	%eax, %eax
	callq	printf@PLT
	movq	%rbx, %rdi
	movq	%r14, %rsi
	xorl	%eax, %eax
	callq	printf@PLT
	leaq	".Ltexte_global-1"(%rip), %rsi
	movq	%r15, %rdi
	callq	concat_strings@PLT
	movq	%rbx, %rdi
	movq	%rax, %rsi
	xorl	%eax, %eax
	callq	printf@PLT
	movq	%rbx, %rdi
	movq	%r14, %rsi
	xorl	%eax, %eax
	callq	printf@PLT
	movq	$-1, %rdi
	callq	texte_nombre@PLT
	movq	%r15, %rdi
	movq	%rax, %rsi
	callq	concat_strings@PLT
	movq	%rbx, %rdi
	movq	%rax, %rsi
	xorl	%eax, %eax
	callq	printf@PLT
	movq	%rbx, %rdi
	movq	%r14, %rsi
	xorl	%eax, %eax
	callq	printf@PLT
	callq	texte_nombre@PLT
	movq	%r15, %rdi
	movq	%rax, %rsi
	callq	concat_strings@PLT
	movq	%rbx, %rdi
	movq	%rax, %rsi
	xorl	%eax, %eax
	callq	printf@PLT
	movq	%rbx, %rdi
	movq	%r14, %rsi
	xorl	%eax, %eax
	callq	printf@PLT
	movl	$1, %edi
	callq	texte_nombre@PLT
	movq	%r15, %rdi
	movq	%rax, %rsi
	callq	concat_strings@PLT
	movq	%rbx, %rdi
	movq	%rax, %rsi
	xorl	%eax, %eax
	callq	printf@PLT
	movq	%rbx, %rdi
	movq	%r14, %rsi
	xorl	%eax, %eax
	callq	printf@PLT
	xorl	%eax, %eax
	addq	$8, %rsp
	.cfi_def_cfa_offset 40
	popq	%rbx
	.cfi_def_cfa_offset 32
	popq	%r12
	.cfi_def_cfa_offset 24
	popq	%r14
	.cfi_def_cfa_offset 16
	popq	%r15
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end9:
	.size	main, .Lfunc_end9-main
	.cfi_endproc
                                        # -- End function
	.type	.Lzero,@object                  # @zero
	.section	.rodata,"a",@progbits
.Lzero:
	.asciz	"z\303\251ro"
	.size	.Lzero, 6

	.type	.Lun,@object                    # @un
.Lun:
	.asciz	"un"
	.size	.Lun, 3

	.type	.Ldeux,@object                  # @deux
.Ldeux:
	.asciz	"deux"
	.size	.Ldeux, 5

	.type	.Ltrois,@object                 # @trois
.Ltrois:
	.asciz	"trois"
	.size	.Ltrois, 6

	.type	.Lquatre,@object                # @quatre
.Lquatre:
	.asciz	"quatre"
	.size	.Lquatre, 7

	.type	.Lcinq,@object                  # @cinq
.Lcinq:
	.asciz	"cinq"
	.size	.Lcinq, 5

	.type	.Lsix,@object                   # @six
.Lsix:
	.asciz	"six"
	.size	.Lsix, 4

	.type	.Lsept,@object                  # @sept
.Lsept:
	.asciz	"sept"
	.size	.Lsept, 5

	.type	.Lhuit,@object                  # @huit
.Lhuit:
	.asciz	"huit"
	.size	.Lhuit, 5

	.type	.Lneuf,@object                  # @neuf
.Lneuf:
	.asciz	"neuf"
	.size	.Lneuf, 5

	.type	.Ldix,@object                   # @dix
.Ldix:
	.asciz	"dix"
	.size	.Ldix, 4

	.type	.Lonze,@object                  # @onze
.Lonze:
	.asciz	"onze"
	.size	.Lonze, 5

	.type	.Ldouze,@object                 # @douze
.Ldouze:
	.asciz	"douze"
	.size	.Ldouze, 6

	.type	.Ltreize,@object                # @treize
.Ltreize:
	.asciz	"treize"
	.size	.Ltreize, 7

	.type	.Lquatorze,@object              # @quatorze
.Lquatorze:
	.asciz	"quatorze"
	.size	.Lquatorze, 9

	.type	.Lquinze,@object                # @quinze
.Lquinze:
	.asciz	"quinze"
	.size	.Lquinze, 7

	.type	.Lseize,@object                 # @seize
.Lseize:
	.asciz	"seize"
	.size	.Lseize, 6

	.type	".Ldix-sept",@object            # @dix-sept
".Ldix-sept":
	.asciz	"dix-sept"
	.size	".Ldix-sept", 9

	.type	".Ldix-huit",@object            # @dix-huit
".Ldix-huit":
	.asciz	"dix-huit"
	.size	".Ldix-huit", 9

	.type	".Ldix-neuf",@object            # @dix-neuf
".Ldix-neuf":
	.asciz	"dix-neuf"
	.size	".Ldix-neuf", 9

	.type	.Lvingt,@object                 # @vingt
.Lvingt:
	.asciz	"vingt"
	.size	.Lvingt, 6

	.type	.Ltrente,@object                # @trente
.Ltrente:
	.asciz	"trente"
	.size	.Ltrente, 7

	.type	.Lquarante,@object              # @quarante
.Lquarante:
	.asciz	"quarante"
	.size	.Lquarante, 9

	.type	.Lcinquante,@object             # @cinquante
.Lcinquante:
	.asciz	"cinquante"
	.size	.Lcinquante, 10

	.type	.Lsoixante,@object              # @soixante
.Lsoixante:
	.asciz	"soixante"
	.size	.Lsoixante, 9

	.type	".Lquatre-vingts",@object       # @quatre-vingts
".Lquatre-vingts":
	.asciz	"quatre-vingts"
	.size	".Lquatre-vingts", 14

	.type	.Lcent,@object                  # @cent
.Lcent:
	.asciz	"cent"
	.size	.Lcent, 5

	.type	.Lvide,@object                  # @vide
.Lvide:
	.zero	1
	.size	.Lvide, 1

	.type	.Lmille,@object                 # @mille
.Lmille:
	.asciz	"mille"
	.size	.Lmille, 6

	.type	.Lmillion,@object               # @million
.Lmillion:
	.asciz	"million"
	.size	.Lmillion, 8

	.type	.Lmilliard,@object              # @milliard
.Lmilliard:
	.asciz	"milliard"
	.size	.Lmilliard, 9

	.type	.Lbillion,@object               # @billion
.Lbillion:
	.asciz	"billion"
	.size	.Lbillion, 8

	.type	.Lbilliard,@object              # @billiard
.Lbilliard:
	.asciz	"billiard"
	.size	.Lbilliard, 9

	.type	.Ltrillion,@object              # @trillion
.Ltrillion:
	.asciz	"trillion"
	.size	.Ltrillion, 9

	.type	.Linfini,@object                # @infini
.Linfini:
	.asciz	"infini"
	.size	.Linfini, 7

	.type	.Lvrai,@object                  # @vrai
.Lvrai:
	.asciz	"vrai"
	.size	.Lvrai, 5

	.type	.Lfaux,@object                  # @faux
.Lfaux:
	.asciz	"faux"
	.size	.Lfaux, 5

	.type	.Lnewline,@object               # @newline
	.section	.rodata.str1.1,"aMS",@progbits,1
.Lnewline:
	.asciz	"\n"
	.size	.Lnewline, 2

	.type	.Ltiret,@object                 # @tiret
.Ltiret:
	.asciz	"-"
	.size	.Ltiret, 2

	.type	.Let,@object                    # @et
.Let:
	.asciz	"-et"
	.size	.Let, 4

	.type	.Ls,@object                     # @s
.Ls:
	.asciz	"s"
	.size	.Ls, 2

	.type	.Lespace,@object                # @espace
.Lespace:
	.asciz	" "
	.size	.Lespace, 2

	.type	petits_nombres,@object          # @petits_nombres
	.data
	.globl	petits_nombres
	.p2align	4, 0x0
petits_nombres:
	.quad	.Lzero
	.quad	.Lun
	.quad	.Ldeux
	.quad	.Ltrois
	.quad	.Lquatre
	.quad	.Lcinq
	.quad	.Lsix
	.quad	.Lsept
	.quad	.Lhuit
	.quad	.Lneuf
	.quad	.Ldix
	.quad	.Lonze
	.quad	.Ldouze
	.quad	.Ltreize
	.quad	.Lquatorze
	.quad	.Lquinze
	.quad	.Lseize
	.quad	".Ldix-sept"
	.quad	".Ldix-huit"
	.quad	".Ldix-neuf"
	.size	petits_nombres, 160

	.type	dizaine,@object                 # @dizaine
	.globl	dizaine
	.p2align	4, 0x0
dizaine:
	.quad	.Lcent
	.quad	.Ldix
	.quad	.Lvingt
	.quad	.Ltrente
	.quad	.Lquarante
	.quad	.Lcinquante
	.quad	.Lsoixante
	.quad	.Lsoixante
	.quad	".Lquatre-vingts"
	.quad	".Lquatre-vingts"
	.size	dizaine, 80

	.type	separateurs,@object             # @separateurs
	.globl	separateurs
	.p2align	4, 0x0
separateurs:
	.quad	.Lvide
	.quad	.Lmille
	.quad	.Lmillion
	.quad	.Lmilliard
	.quad	.Lbillion
	.quad	.Lbilliard
	.quad	.Ltrillion
	.quad	.Linfini
	.size	separateurs, 64

	.type	booleen,@object                 # @booleen
	.globl	booleen
	.p2align	3, 0x0
booleen:
	.quad	.Lfaux
	.quad	.Lvrai
	.size	booleen, 16

	.type	.Lformat_str,@object            # @format_str
	.section	.rodata,"a",@progbits
.Lformat_str:
	.asciz	"%s"
	.size	.Lformat_str, 3

	.type	".Ltexte_global-0",@object      # @texte_global-0
	.section	.rodata.str1.1,"aMS",@progbits,1
".Ltexte_global-0":
	.asciz	"test : "
	.size	".Ltexte_global-0", 8

	.type	".Ltexte_global-1",@object      # @texte_global-1
".Ltexte_global-1":
	.asciz	"---------"
	.size	".Ltexte_global-1", 10

	.section	".note.GNU-stack","",@progbits
