	.text
	.file	"carre.ll"
	.globl	affiche_centaine                # -- Begin function affiche_centaine
	.p2align	4, 0x90
	.type	affiche_centaine,@function
affiche_centaine:                       # @affiche_centaine
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rax
	.cfi_def_cfa_offset 16
	cmpl	$9, %edi
	jg	.LBB0_5
# %bb.1:                                # %pas_infini
	testl	%edi, %edi
	je	.LBB0_7
# %bb.2:                                # %centaine
	cmpl	$1, %edi
	je	.LBB0_4
# %bb.3:                                # %nombre
	movslq	%edi, %rax
	movq	petits_nombres@GOTPCREL(%rip), %rcx
	movq	(%rcx,%rax,8), %rdi
	callq	printf@PLT
	callq	affiche_tiret@PLT
.LBB0_4:                                # %cent
	movq	dizaine@GOTPCREL(%rip), %rax
	movq	(%rax), %rdi
	jmp	.LBB0_6
.LBB0_5:                                # %infini
	movq	separateurs@GOTPCREL(%rip), %rax
	movq	48(%rax), %rdi
.LBB0_6:                                # %fin
	callq	printf@PLT
.LBB0_7:                                # %fin
	popq	%rax
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end0:
	.size	affiche_centaine, .Lfunc_end0-affiche_centaine
	.cfi_endproc
                                        # -- End function
	.globl	affiche_unite                   # -- Begin function affiche_unite
	.p2align	4, 0x90
	.type	affiche_unite,@function
affiche_unite:                          # @affiche_unite
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rax
	.cfi_def_cfa_offset 16
	cmpl	$19, %edi
	jg	.LBB1_2
# %bb.1:                                # %unite
	movslq	%edi, %rax
	movq	petits_nombres@GOTPCREL(%rip), %rcx
	movq	(%rcx,%rax,8), %rdi
	jmp	.LBB1_3
.LBB1_2:                                # %infini
	movq	separateurs@GOTPCREL(%rip), %rax
	movq	48(%rax), %rdi
.LBB1_3:                                # %fin
	callq	printf@PLT
	popq	%rax
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end1:
	.size	affiche_unite, .Lfunc_end1-affiche_unite
	.cfi_endproc
                                        # -- End function
	.globl	affiche_dizaine                 # -- Begin function affiche_dizaine
	.p2align	4, 0x90
	.type	affiche_dizaine,@function
affiche_dizaine:                        # @affiche_dizaine
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rbp
	.cfi_def_cfa_offset 16
	pushq	%r14
	.cfi_def_cfa_offset 24
	pushq	%rbx
	.cfi_def_cfa_offset 32
	.cfi_offset %rbx, -32
	.cfi_offset %r14, -24
	.cfi_offset %rbp, -16
	movl	%edi, %ebx
	cmpl	$19, %edi
	jle	.LBB2_6
# %bb.1:                                # %dizaine
	movslq	%ebx, %rax
	imulq	$1717986919, %rax, %r14         # imm = 0x66666667
	movq	%r14, %rax
	shrq	$63, %rax
	sarq	$34, %r14
	addl	%eax, %r14d
	leal	(%r14,%r14), %eax
	leal	(%rax,%rax,4), %ebp
	movslq	%r14d, %rax
	movq	dizaine@GOTPCREL(%rip), %rcx
	movq	(%rcx,%rax,8), %rdi
	callq	printf@PLT
	subl	%ebp, %ebx
	je	.LBB2_7
# %bb.2:                                # %pitet-et
	cmpl	$1, %ebx
	sete	%al
	cmpl	$8, %r14d
	setl	%cl
	mulb	%cl
	cmpb	$1, %al
	jne	.LBB2_4
# %bb.3:                                # %affiche-et
	leaq	.Let(%rip), %rdi
	callq	printf@PLT
.LBB2_4:                                # %pitet-special
	callq	affiche_tiret@PLT
	cmpl	$7, %r14d
	sete	%al
	cmpl	$9, %r14d
	sete	%cl
	addb	%al, %cl
	testb	$1, %cl
	je	.LBB2_6
# %bb.5:                                # %unite-special
	addl	$10, %ebx
.LBB2_6:                                # %unite-simple
	movl	%ebx, %edi
	callq	affiche_unite@PLT
.LBB2_7:                                # %fin
	popq	%rbx
	.cfi_def_cfa_offset 24
	popq	%r14
	.cfi_def_cfa_offset 16
	popq	%rbp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end2:
	.size	affiche_dizaine, .Lfunc_end2-affiche_dizaine
	.cfi_endproc
                                        # -- End function
	.globl	affiche_petit_nombre            # -- Begin function affiche_petit_nombre
	.p2align	4, 0x90
	.type	affiche_petit_nombre,@function
affiche_petit_nombre:                   # @affiche_petit_nombre
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rbx
	.cfi_def_cfa_offset 16
	.cfi_offset %rbx, -16
	movslq	%edi, %rax
	imulq	$1374389535, %rax, %rdi         # imm = 0x51EB851F
	movq	%rdi, %rcx
	shrq	$63, %rcx
	sarq	$37, %rdi
	addl	%ecx, %edi
	imull	$100, %edi, %ecx
	movl	%eax, %ebx
	subl	%ecx, %ebx
	cmpl	$100, %eax
	jl	.LBB3_3
# %bb.1:                                # %centaine
                                        # kill: def $edi killed $edi killed $rdi
	callq	affiche_centaine@PLT
	testl	%ebx, %ebx
	jle	.LBB3_4
# %bb.2:                                # %separateur
	callq	affiche_tiret@PLT
.LBB3_3:                                # %dizaine
	movl	%ebx, %edi
	callq	affiche_dizaine@PLT
.LBB3_4:                                # %fin
	popq	%rbx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end3:
	.size	affiche_petit_nombre, .Lfunc_end3-affiche_petit_nombre
	.cfi_endproc
                                        # -- End function
	.globl	affiche_tiret                   # -- Begin function affiche_tiret
	.p2align	4, 0x90
	.type	affiche_tiret,@function
affiche_tiret:                          # @affiche_tiret
	.cfi_startproc
# %bb.0:
	pushq	%rax
	.cfi_def_cfa_offset 16
	leaq	.Ltiret(%rip), %rdi
	callq	printf@PLT
	popq	%rax
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end4:
	.size	affiche_tiret, .Lfunc_end4-affiche_tiret
	.cfi_endproc
                                        # -- End function
	.globl	nouvelle_ligne                  # -- Begin function nouvelle_ligne
	.p2align	4, 0x90
	.type	nouvelle_ligne,@function
nouvelle_ligne:                         # @nouvelle_ligne
	.cfi_startproc
# %bb.0:
	pushq	%rax
	.cfi_def_cfa_offset 16
	leaq	.Lnewline(%rip), %rdi
	callq	printf@PLT
	popq	%rax
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end5:
	.size	nouvelle_ligne, .Lfunc_end5-nouvelle_ligne
	.cfi_endproc
                                        # -- End function
	.globl	mille_puissance                 # -- Begin function mille_puissance
	.p2align	4, 0x90
	.type	mille_puissance,@function
mille_puissance:                        # @mille_puissance
	.cfi_startproc
# %bb.0:                                # %entry
	testl	%edi, %edi
	je	.LBB6_1
# %bb.2:                                # %recursion
	pushq	%rax
	.cfi_def_cfa_offset 16
	decl	%edi
	callq	mille_puissance@PLT
	imulq	$1000, %rax, %rax               # imm = 0x3E8
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.LBB6_1:                                # %zero
	movl	$1, %eax
	retq
.Lfunc_end6:
	.size	mille_puissance, .Lfunc_end6-mille_puissance
	.cfi_endproc
                                        # -- End function
	.globl	log_mille                       # -- Begin function log_mille
	.p2align	4, 0x90
	.type	log_mille,@function
log_mille:                              # @log_mille
	.cfi_startproc
# %bb.0:                                # %entry
	cmpq	$999, %rdi                      # imm = 0x3E7
	jg	.LBB7_2
# %bb.1:                                # %zero
	xorl	%eax, %eax
	retq
.LBB7_2:                                # %recursion
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
.Lfunc_end7:
	.size	log_mille, .Lfunc_end7-log_mille
	.cfi_endproc
                                        # -- End function
	.globl	affiche_nombre                  # -- Begin function affiche_nombre
	.p2align	4, 0x90
	.type	affiche_nombre,@function
affiche_nombre:                         # @affiche_nombre
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rbp
	.cfi_def_cfa_offset 16
	pushq	%r14
	.cfi_def_cfa_offset 24
	pushq	%rbx
	.cfi_def_cfa_offset 32
	.cfi_offset %rbx, -32
	.cfi_offset %r14, -24
	.cfi_offset %rbp, -16
	testq	%rdi, %rdi
	je	.LBB8_1
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
	movq	%rax, %r14
	movl	%r14d, %edi
	callq	affiche_petit_nombre@PLT
	testl	%ebp, %ebp
	je	.LBB8_7
# %bb.3:                                # %separateur
	callq	affiche_tiret@PLT
	movslq	%ebp, %rax
	movq	separateurs@GOTPCREL(%rip), %rcx
	movq	(%rcx,%rax,8), %rdi
	callq	printf@PLT
	cmpl	$2, %r14d
	setl	%al
	cmpl	$2, %ebp
	setl	%cl
	addb	%al, %cl
	testb	$1, %cl
	jne	.LBB8_5
# %bb.4:                                # %affiche-s
	leaq	.Ls(%rip), %rdi
	callq	printf@PLT
.LBB8_5:                                # %pitet-recursion
	testq	%rbx, %rbx
	je	.LBB8_7
# %bb.6:                                # %recursion
	callq	affiche_tiret@PLT
	movq	%rbx, %rdi
	callq	affiche_nombre@PLT
	jmp	.LBB8_7
.LBB8_1:                                # %affiche_zero
	movq	petits_nombres@GOTPCREL(%rip), %rax
	movq	(%rax), %rdi
	callq	printf@PLT
.LBB8_7:                                # %fin
	popq	%rbx
	.cfi_def_cfa_offset 24
	popq	%r14
	.cfi_def_cfa_offset 16
	popq	%rbp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end8:
	.size	affiche_nombre, .Lfunc_end8-affiche_nombre
	.cfi_endproc
                                        # -- End function
	.globl	main                            # -- Begin function main
	.p2align	4, 0x90
	.type	main,@function
main:                                   # @main
	.cfi_startproc
# %bb.0:
	pushq	%rax
	.cfi_def_cfa_offset 16
	movl	$987, %edi                      # imm = 0x3DB
	callq	affiche_nombre@PLT
	xorl	%eax, %eax
	popq	%rcx
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

	.section	".note.GNU-stack","",@progbits
