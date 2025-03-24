	.text
	.file	"test.ll"
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
	pushq	%rbp
	.cfi_def_cfa_offset 16
	pushq	%r14
	.cfi_def_cfa_offset 24
	pushq	%rbx
	.cfi_def_cfa_offset 32
	.cfi_offset %rbx, -32
	.cfi_offset %r14, -24
	.cfi_offset %rbp, -16
	cmpl	$19, %edi
	jg	.LBB2_2
# %bb.1:                                # %unite
	callq	texte_unite@PLT
	jmp	.LBB2_5
.LBB2_2:                                # %dizaine
	movslq	%edi, %rax
	imulq	$1717986919, %rax, %rax         # imm = 0x66666667
	movq	%rax, %rsi
	shrq	$63, %rsi
	sarq	$34, %rax
	leal	(%rax,%rsi), %edx
	leal	(%rdx,%rdx), %ecx
	leal	(%rcx,%rcx,4), %ecx
	subl	%ecx, %edi
	movslq	%edx, %rcx
	movq	dizaine@GOTPCREL(%rip), %r8
	movq	(%r8,%rcx,8), %rcx
	leal	-7(%rax,%rsi), %eax
	testl	$-3, %eax
	sete	%bpl
	je	.LBB2_6
# %bb.3:                                # %dizaine
	testl	%edi, %edi
	jne	.LBB2_6
# %bb.4:                                # %juste_dizaine
	movq	%rcx, %rax
	jmp	.LBB2_5
.LBB2_6:                                # %pitet-et
	movl	%edi, %ebx
	cmpl	$1, %edi
	sete	%al
	cmpl	$8, %edx
	setl	%dl
	mulb	%dl
	cmpb	$1, %al
	jne	.LBB2_8
# %bb.7:                                # %affiche-et
	leaq	.Let(%rip), %rsi
	movq	%rcx, %rdi
	callq	concat_strings@PLT
	movq	%rax, %rcx
.LBB2_8:                                # %pitet-special
	leaq	.Ltiret(%rip), %rsi
	movq	%rcx, %rdi
	callq	concat_strings@PLT
	movq	%rax, %r14
	testb	%bpl, %bpl
	je	.LBB2_10
# %bb.9:                                # %unite-special
	addl	$10, %ebx
.LBB2_10:                               # %unite-simple
	movl	%ebx, %edi
	callq	texte_unite@PLT
	movq	%r14, %rdi
	movq	%rax, %rsi
	callq	concat_strings@PLT
.LBB2_5:                                # %juste_dizaine
	popq	%rbx
	.cfi_def_cfa_offset 24
	popq	%r14
	.cfi_def_cfa_offset 16
	popq	%rbp
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
	pushq	%r12
	.cfi_def_cfa_offset 40
	pushq	%rbx
	.cfi_def_cfa_offset 48
	.cfi_offset %rbx, -48
	.cfi_offset %r12, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.cfi_offset %rbp, -16
	testq	%rdi, %rdi
	je	.LBB6_1
# %bb.2:                                # %nombre
	js	.LBB6_4
# %bb.3:
	leaq	.Lvide(%rip), %r14
	jmp	.LBB6_5
.LBB6_1:                                # %affiche_zero
	movq	petits_nombres@GOTPCREL(%rip), %rax
	movq	(%rax), %rax
	jmp	.LBB6_15
.LBB6_4:                                # %negatif
	negq	%rdi
	leaq	.Lmoins(%rip), %r14
.LBB6_5:                                # %positif
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
	movq	%r14, %rdi
	movq	%rax, %rsi
	callq	concat_strings@PLT
	testl	%ebp, %ebp
	je	.LBB6_15
# %bb.6:                                # %separateur
	leaq	.Ltiret(%rip), %r14
	movq	%rax, %rdi
	movq	%r14, %rsi
	callq	concat_strings@PLT
	movslq	%ebp, %rcx
	movq	separateurs@GOTPCREL(%rip), %rdx
	movq	(%rdx,%rcx,8), %r12
	movq	%rax, %rdi
	movq	%r12, %rsi
	callq	concat_strings@PLT
	cmpl	$1, %r15d
	jg	.LBB6_8
# %bb.7:                                # %separateur
	cmpl	$2, %ebp
	jge	.LBB6_8
# %bb.12:                               # %pitet-recursion
	testq	%rbx, %rbx
	je	.LBB6_13
.LBB6_14:                               # %recursion
	movq	%r12, %rdi
	movq	%r14, %rsi
	callq	concat_strings@PLT
	movq	%rax, %r14
	movq	%rbx, %rdi
	callq	texte_nombre@PLT
	movq	%r14, %rdi
	movq	%rax, %rsi
	callq	concat_strings@PLT
	jmp	.LBB6_15
.LBB6_8:                                # %autre
	cmpl	$2, %r15d
	jl	.LBB6_11
# %bb.9:                                # %autre
	cmpl	$2, %ebp
	jl	.LBB6_11
# %bb.10:                               # %pluriel
	leaq	.Ls(%rip), %rsi
	movq	%rax, %rdi
	callq	concat_strings@PLT
.LBB6_11:                               # %pitet-recursion
	movq	%rax, %r12
	testq	%rbx, %rbx
	jne	.LBB6_14
.LBB6_13:                               # %fin
	movq	%r12, %rax
.LBB6_15:                               # %petit
	popq	%rbx
	.cfi_def_cfa_offset 40
	popq	%r12
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
	.globl	compare_texte                   # -- Begin function compare_texte
	.p2align	4, 0x90
	.type	compare_texte,@function
compare_texte:                          # @compare_texte
	.cfi_startproc
# %bb.0:
	pushq	%rbx
	.cfi_def_cfa_offset 16
	.cfi_offset %rbx, -16
	movl	%edx, %ebx
	callq	strcmp@PLT
	testl	%eax, %eax
	setne	%al
	xorb	%bl, %al
	popq	%rbx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end9:
	.size	compare_texte, .Lfunc_end9-compare_texte
	.cfi_endproc
                                        # -- End function
	.globl	read_line                       # -- Begin function read_line
	.p2align	4, 0x90
	.type	read_line,@function
read_line:                              # @read_line
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rbx
	.cfi_def_cfa_offset 16
	.cfi_offset %rbx, -16
	movq	buffer@GOTPCREL(%rip), %rbx
	movq	%rbx, %rdi
	callq	gets@PLT
	testq	%rax, %rax
	je	.LBB10_1
# %bb.2:                                # %return_buffer
	movq	%rbx, %rax
	popq	%rbx
	.cfi_def_cfa_offset 8
	retq
.LBB10_1:                               # %return_null
	.cfi_def_cfa_offset 16
	xorl	%eax, %eax
	popq	%rbx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end10:
	.size	read_line, .Lfunc_end10-read_line
	.cfi_endproc
                                        # -- End function
	.globl	demande_texte                   # -- Begin function demande_texte
	.p2align	4, 0x90
	.type	demande_texte,@function
demande_texte:                          # @demande_texte
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rax
	.cfi_def_cfa_offset 16
	movq	%rdi, %rsi
	leaq	.Ldemande_str(%rip), %rdi
	leaq	.Ltype_texte(%rip), %rdx
	xorl	%eax, %eax
	callq	printf@PLT
	callq	read_line@PLT
	testq	%rax, %rax
	je	.LBB11_2
# %bb.1:                                # %texte
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.LBB11_2:                               # %vide
	.cfi_def_cfa_offset 16
	leaq	.Lvide(%rip), %rax
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end11:
	.size	demande_texte, .Lfunc_end11-demande_texte
	.cfi_endproc
                                        # -- End function
	.globl	demande_booleen                 # -- Begin function demande_booleen
	.p2align	4, 0x90
	.type	demande_booleen,@function
demande_booleen:                        # @demande_booleen
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
	movq	%rdi, %rbx
	leaq	.Ldemande_str(%rip), %rdi
	leaq	.Ltype_booleen(%rip), %rdx
	movq	%rbx, %rsi
	xorl	%eax, %eax
	callq	printf@PLT
	callq	read_line@PLT
	testq	%rax, %rax
	je	.LBB12_1
# %bb.3:                                # %texte
	movq	%rax, %r14
	movq	booleen@GOTPCREL(%rip), %r15
	movq	8(%r15), %rsi
	movq	%rax, %rdi
	movl	$1, %edx
	callq	compare_texte@PLT
	testb	$1, %al
	je	.LBB12_6
# %bb.4:                                # %vrai
	movb	$1, %al
	jmp	.LBB12_5
.LBB12_1:                               # %vide
	leaq	.Lvide(%rip), %r14
	jmp	.LBB12_2
.LBB12_6:                               # %pas-vrai
	movq	(%r15), %rsi
	movq	%r14, %rdi
	movl	$1, %edx
	callq	compare_texte@PLT
	testb	$1, %al
	je	.LBB12_2
# %bb.7:                                # %faux
	xorl	%eax, %eax
	jmp	.LBB12_5
.LBB12_2:                               # %redemande
	leaq	.Lbooleen_invalide(%rip), %rdi
	movq	%r14, %rsi
	xorl	%eax, %eax
	callq	printf@PLT
	movq	%rbx, %rdi
	callq	demande_booleen@PLT
.LBB12_5:                               # %vrai
	popq	%rbx
	.cfi_def_cfa_offset 24
	popq	%r14
	.cfi_def_cfa_offset 16
	popq	%r15
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end12:
	.size	demande_booleen, .Lfunc_end12-demande_booleen
	.cfi_endproc
                                        # -- End function
	.globl	verifie_unite                   # -- Begin function verifie_unite
	.p2align	4, 0x90
	.type	verifie_unite,@function
verifie_unite:                          # @verifie_unite
	.cfi_startproc
# %bb.0:
	pushq	%rax
	.cfi_def_cfa_offset 16
	movslq	%esi, %rax
	movq	petits_nombres@GOTPCREL(%rip), %rcx
	movq	(%rcx,%rax,8), %rsi
	movl	$1, %edx
	callq	compare_texte@PLT
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end13:
	.size	verifie_unite, .Lfunc_end13-verifie_unite
	.cfi_endproc
                                        # -- End function
	.globl	verifie_dizaine                 # -- Begin function verifie_dizaine
	.p2align	4, 0x90
	.type	verifie_dizaine,@function
verifie_dizaine:                        # @verifie_dizaine
	.cfi_startproc
# %bb.0:
	pushq	%rax
	.cfi_def_cfa_offset 16
	movslq	%esi, %rax
	movq	dizaine@GOTPCREL(%rip), %rcx
	movq	(%rcx,%rax,8), %rsi
	movl	$1, %edx
	callq	compare_texte@PLT
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end14:
	.size	verifie_dizaine, .Lfunc_end14-verifie_dizaine
	.cfi_endproc
                                        # -- End function
	.globl	verifie_separateur              # -- Begin function verifie_separateur
	.p2align	4, 0x90
	.type	verifie_separateur,@function
verifie_separateur:                     # @verifie_separateur
	.cfi_startproc
# %bb.0:
	pushq	%rbp
	.cfi_def_cfa_offset 16
	pushq	%r14
	.cfi_def_cfa_offset 24
	pushq	%rbx
	.cfi_def_cfa_offset 32
	.cfi_offset %rbx, -32
	.cfi_offset %r14, -24
	.cfi_offset %rbp, -16
	movq	%rdi, %rbx
	movslq	%esi, %rax
	movq	separateurs@GOTPCREL(%rip), %rcx
	movq	(%rcx,%rax,8), %r14
	movq	%r14, %rsi
	movl	$1, %edx
	callq	compare_texte@PLT
	movl	%eax, %ebp
	leaq	.Ls(%rip), %rsi
	movq	%r14, %rdi
	callq	concat_strings@PLT
	movq	%rbx, %rdi
	movq	%rax, %rsi
	movl	$1, %edx
	callq	compare_texte@PLT
	orb	%bpl, %al
	popq	%rbx
	.cfi_def_cfa_offset 24
	popq	%r14
	.cfi_def_cfa_offset 16
	popq	%rbp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end15:
	.size	verifie_separateur, .Lfunc_end15-verifie_separateur
	.cfi_endproc
                                        # -- End function
	.globl	mot_comme_unite                 # -- Begin function mot_comme_unite
	.p2align	4, 0x90
	.type	mot_comme_unite,@function
mot_comme_unite:                        # @mot_comme_unite
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rbp
	.cfi_def_cfa_offset 16
	pushq	%rbx
	.cfi_def_cfa_offset 24
	pushq	%rax
	.cfi_def_cfa_offset 32
	.cfi_offset %rbx, -24
	.cfi_offset %rbp, -16
	movq	%rdi, %rbx
	movl	$0, 4(%rsp)
	.p2align	4, 0x90
.LBB16_1:                               # %check
                                        # =>This Inner Loop Header: Depth=1
	movl	4(%rsp), %ebp
	movq	%rbx, %rdi
	movl	%ebp, %esi
	callq	verifie_unite@PLT
	testb	$1, %al
	jne	.LBB16_5
# %bb.2:                                # %next
                                        #   in Loop: Header=BB16_1 Depth=1
	incl	%ebp
	movl	%ebp, 4(%rsp)
	cmpl	$20, %ebp
	jl	.LBB16_1
# %bb.3:                                # %default
	xorl	%eax, %eax
	jmp	.LBB16_4
.LBB16_5:                               # %return
	movl	4(%rsp), %eax
.LBB16_4:                               # %default
	addq	$8, %rsp
	.cfi_def_cfa_offset 24
	popq	%rbx
	.cfi_def_cfa_offset 16
	popq	%rbp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end16:
	.size	mot_comme_unite, .Lfunc_end16-mot_comme_unite
	.cfi_endproc
                                        # -- End function
	.globl	mot_comme_dizaine               # -- Begin function mot_comme_dizaine
	.p2align	4, 0x90
	.type	mot_comme_dizaine,@function
mot_comme_dizaine:                      # @mot_comme_dizaine
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rbp
	.cfi_def_cfa_offset 16
	pushq	%rbx
	.cfi_def_cfa_offset 24
	pushq	%rax
	.cfi_def_cfa_offset 32
	.cfi_offset %rbx, -24
	.cfi_offset %rbp, -16
	movq	%rdi, %rbx
	movl	$0, 4(%rsp)
	.p2align	4, 0x90
.LBB17_1:                               # %check
                                        # =>This Inner Loop Header: Depth=1
	movl	4(%rsp), %ebp
	movq	%rbx, %rdi
	movl	%ebp, %esi
	callq	verifie_dizaine@PLT
	testb	$1, %al
	jne	.LBB17_3
# %bb.2:                                # %next
                                        #   in Loop: Header=BB17_1 Depth=1
	incl	%ebp
	movl	%ebp, 4(%rsp)
	cmpl	$10, %ebp
	jl	.LBB17_1
	jmp	.LBB17_7
.LBB17_3:                               # %return
	movl	4(%rsp), %eax
	testq	%rax, %rax
	je	.LBB17_4
# %bb.6:                                # %pas-cent
	cmpl	$1, %eax
	jne	.LBB17_8
.LBB17_7:                               # %dix
	xorl	%eax, %eax
	jmp	.LBB17_5
.LBB17_4:                               # %cent
	movl	$100, %eax
	jmp	.LBB17_5
.LBB17_8:                               # %pas-dix
	addq	%rax, %rax
	leaq	(%rax,%rax,4), %rax
.LBB17_5:                               # %cent
	addq	$8, %rsp
	.cfi_def_cfa_offset 24
	popq	%rbx
	.cfi_def_cfa_offset 16
	popq	%rbp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end17:
	.size	mot_comme_dizaine, .Lfunc_end17-mot_comme_dizaine
	.cfi_endproc
                                        # -- End function
	.globl	mot_comme_separateur            # -- Begin function mot_comme_separateur
	.p2align	4, 0x90
	.type	mot_comme_separateur,@function
mot_comme_separateur:                   # @mot_comme_separateur
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rbp
	.cfi_def_cfa_offset 16
	pushq	%rbx
	.cfi_def_cfa_offset 24
	pushq	%rax
	.cfi_def_cfa_offset 32
	.cfi_offset %rbx, -24
	.cfi_offset %rbp, -16
	movq	%rdi, %rbx
	movl	$0, 4(%rsp)
	.p2align	4, 0x90
.LBB18_1:                               # %check
                                        # =>This Inner Loop Header: Depth=1
	movl	4(%rsp), %ebp
	movq	%rbx, %rdi
	movl	%ebp, %esi
	callq	verifie_separateur@PLT
	testb	$1, %al
	jne	.LBB18_5
# %bb.2:                                # %next
                                        #   in Loop: Header=BB18_1 Depth=1
	incl	%ebp
	movl	%ebp, 4(%rsp)
	cmpl	$8, %ebp
	jl	.LBB18_1
# %bb.3:                                # %default
	xorl	%eax, %eax
	jmp	.LBB18_4
.LBB18_5:                               # %return
	movl	4(%rsp), %edi
	callq	mille_puissance@PLT
.LBB18_4:                               # %default
	addq	$8, %rsp
	.cfi_def_cfa_offset 24
	popq	%rbx
	.cfi_def_cfa_offset 16
	popq	%rbp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end18:
	.size	mot_comme_separateur, .Lfunc_end18-mot_comme_separateur
	.cfi_endproc
                                        # -- End function
	.globl	mot_comme_entier                # -- Begin function mot_comme_entier
	.p2align	4, 0x90
	.type	mot_comme_entier,@function
mot_comme_entier:                       # @mot_comme_entier
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
	movq	%rdi, %rbx
	leaq	.Lvingts(%rip), %rsi
	movl	$1, %edx
	callq	compare_texte@PLT
	testb	$1, %al
	je	.LBB19_3
# %bb.1:                                # %quatre-vingts
	movl	$76, %eax
	jmp	.LBB19_2
.LBB19_3:                               # %normal
	leaq	".Ljuste-et"(%rip), %rsi
	movq	%rbx, %rdi
	movl	$1, %edx
	callq	compare_texte@PLT
	testb	$1, %al
	je	.LBB19_5
# %bb.4:                                # %et
	xorl	%eax, %eax
	jmp	.LBB19_2
.LBB19_5:                               # %pas-et
	movq	%rbx, %rdi
	callq	mot_comme_unite@PLT
	movq	%rax, %r15
	movq	%rbx, %rdi
	callq	mot_comme_dizaine@PLT
	movq	%rax, %r14
	movq	%rbx, %rdi
	callq	mot_comme_separateur@PLT
	addq	%r15, %r14
	addq	%rax, %r14
	movq	%rbx, %rdi
	xorl	%esi, %esi
	callq	verifie_unite@PLT
	testb	$1, %al
	jne	.LBB19_8
# %bb.6:                                # %pas-et
	testq	%r14, %r14
	jne	.LBB19_8
# %bb.7:                                # %erreur
	movq	$-1, %rax
	jmp	.LBB19_2
.LBB19_8:                               # %bon
	movq	%r14, %rax
.LBB19_2:                               # %quatre-vingts
	popq	%rbx
	.cfi_def_cfa_offset 24
	popq	%r14
	.cfi_def_cfa_offset 16
	popq	%r15
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end19:
	.size	mot_comme_entier, .Lfunc_end19-mot_comme_entier
	.cfi_endproc
                                        # -- End function
	.globl	texte_comme_entier              # -- Begin function texte_comme_entier
	.p2align	4, 0x90
	.type	texte_comme_entier,@function
texte_comme_entier:                     # @texte_comme_entier
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%r15
	.cfi_def_cfa_offset 16
	pushq	%r14
	.cfi_def_cfa_offset 24
	pushq	%rbx
	.cfi_def_cfa_offset 32
	subq	$96, %rsp
	.cfi_def_cfa_offset 128
	.cfi_offset %rbx, -32
	.cfi_offset %r14, -24
	.cfi_offset %r15, -16
	movq	%rdi, %rbx
	callq	strlen@PLT
	movq	%rax, %r14
	movq	$0, (%rsp)
	movq	$0, 16(%rsp)
	movl	$0, 28(%rsp)
	movq	%rbx, 32(%rsp)
	movl	$0, 12(%rsp)
	leaq	46(%rsp), %r15
	jmp	.LBB20_4
	.p2align	4, 0x90
.LBB20_2:                               # %handle_centaine
                                        #   in Loop: Header=BB20_4 Depth=1
	imulq	$100, %rcx, %rax
	movq	%rax, (%rsp)
.LBB20_3:                               # %reset_token
                                        #   in Loop: Header=BB20_4 Depth=1
	movl	$0, 12(%rsp)
	incl	28(%rsp)
.LBB20_4:                               # %loop_start
                                        # =>This Inner Loop Header: Depth=1
	movl	28(%rsp), %eax
	cmpl	%r14d, %eax
	jge	.LBB20_12
# %bb.5:                                # %loop_body
                                        #   in Loop: Header=BB20_4 Depth=1
	cltq
	movzbl	(%rbx,%rax), %eax
	cmpb	$45, %al
	jne	.LBB20_17
# %bb.6:                                # %process_token
                                        #   in Loop: Header=BB20_4 Depth=1
	movl	12(%rsp), %eax
	testl	%eax, %eax
	jle	.LBB20_3
# %bb.7:                                # %do_process_token
                                        #   in Loop: Header=BB20_4 Depth=1
	cltq
	movb	$0, 46(%rsp,%rax)
	movq	%r15, %rdi
	callq	mot_comme_entier@PLT
	cmpq	$-1, %rax
	je	.LBB20_20
# %bb.8:                                # %update_sum
                                        #   in Loop: Header=BB20_4 Depth=1
	movq	(%rsp), %rcx
	cmpq	$100, %rax
	je	.LBB20_1
# %bb.9:                                # %handle_pas_cent
                                        #   in Loop: Header=BB20_4 Depth=1
	jle	.LBB20_18
# %bb.10:                               # %handle_separator
                                        #   in Loop: Header=BB20_4 Depth=1
	movq	16(%rsp), %rdx
	testq	%rcx, %rcx
	je	.LBB20_19
# %bb.11:                               # %sep_with_value
                                        #   in Loop: Header=BB20_4 Depth=1
	imulq	%rax, %rcx
	addq	%rcx, %rdx
	movq	%rdx, 16(%rsp)
	movq	$0, (%rsp)
	jmp	.LBB20_3
	.p2align	4, 0x90
.LBB20_12:                              # %process_last_token
                                        #   in Loop: Header=BB20_4 Depth=1
	movl	12(%rsp), %eax
	testl	%eax, %eax
	jle	.LBB20_26
# %bb.13:                               # %process_final
                                        #   in Loop: Header=BB20_4 Depth=1
	cltq
	movb	$0, 46(%rsp,%rax)
	movq	%r15, %rdi
	callq	mot_comme_entier@PLT
	cmpq	$-1, %rax
	je	.LBB20_20
# %bb.14:                               # %update_final_sum
                                        #   in Loop: Header=BB20_4 Depth=1
	movq	(%rsp), %rcx
	cmpq	$100, %rax
	jne	.LBB20_21
.LBB20_1:                               # %handle_cent
                                        #   in Loop: Header=BB20_4 Depth=1
	testq	%rcx, %rcx
	jne	.LBB20_2
# %bb.16:                               # %handle_cent_alone
                                        #   in Loop: Header=BB20_4 Depth=1
	movq	$100, (%rsp)
	jmp	.LBB20_3
	.p2align	4, 0x90
.LBB20_17:                              # %continue_token
                                        #   in Loop: Header=BB20_4 Depth=1
	movslq	12(%rsp), %rcx
	movb	%al, 46(%rsp,%rcx)
	leal	1(%rcx), %eax
	movl	%eax, 12(%rsp)
	incl	28(%rsp)
	jmp	.LBB20_4
.LBB20_18:                              # %handle_unit
                                        #   in Loop: Header=BB20_4 Depth=1
	addq	%rax, %rcx
	movq	%rcx, (%rsp)
	jmp	.LBB20_3
.LBB20_19:                              # %sep_alone
                                        #   in Loop: Header=BB20_4 Depth=1
	addq	%rax, %rdx
	movq	%rdx, 16(%rsp)
	jmp	.LBB20_3
.LBB20_20:                              # %erreur
	movq	$-1, %rax
	jmp	.LBB20_27
.LBB20_21:                              # %final_handle_pas_cent
	jle	.LBB20_24
# %bb.22:                               # %final_handle_separator
	movq	16(%rsp), %rdx
	testq	%rcx, %rcx
	je	.LBB20_25
# %bb.23:                               # %final_sep_with_value
	imulq	%rax, %rcx
	addq	%rcx, %rdx
	movq	%rdx, 16(%rsp)
	movq	$0, (%rsp)
	jmp	.LBB20_26
.LBB20_24:                              # %final_handle_unit
	addq	%rax, %rcx
	movq	%rcx, (%rsp)
	jmp	.LBB20_26
.LBB20_25:                              # %final_sep_alone
	addq	%rax, %rdx
	movq	%rdx, 16(%rsp)
.LBB20_26:                              # %finish
	movq	16(%rsp), %rax
	addq	(%rsp), %rax
.LBB20_27:                              # %erreur
	addq	$96, %rsp
	.cfi_def_cfa_offset 32
	popq	%rbx
	.cfi_def_cfa_offset 24
	popq	%r14
	.cfi_def_cfa_offset 16
	popq	%r15
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end20:
	.size	texte_comme_entier, .Lfunc_end20-texte_comme_entier
	.cfi_endproc
                                        # -- End function
	.globl	demande_entier                  # -- Begin function demande_entier
	.p2align	4, 0x90
	.type	demande_entier,@function
demande_entier:                         # @demande_entier
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
	movq	%rdi, %rbx
	leaq	.Ldemande_str(%rip), %rdi
	leaq	.Ltype_entier(%rip), %rdx
	movq	%rbx, %rsi
	xorl	%eax, %eax
	callq	printf@PLT
	callq	read_line@PLT
	testq	%rax, %rax
	je	.LBB21_1
# %bb.2:                                # %texte
	movq	%rax, %r14
	movq	%rax, %rdi
	callq	texte_comme_entier@PLT
	cmpq	$-1, %rax
	jne	.LBB21_4
	jmp	.LBB21_3
.LBB21_1:                               # %vide
	leaq	.Lvide(%rip), %r14
.LBB21_3:                               # %redemande
	leaq	.Lentier_invalide(%rip), %rdi
	movq	%r14, %rsi
	xorl	%eax, %eax
	callq	printf@PLT
	movq	%rbx, %rdi
	callq	demande_entier@PLT
.LBB21_4:                               # %bon-nombre
	addq	$8, %rsp
	.cfi_def_cfa_offset 24
	popq	%rbx
	.cfi_def_cfa_offset 16
	popq	%r14
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end21:
	.size	demande_entier, .Lfunc_end21-demande_entier
	.cfi_endproc
                                        # -- End function
	.globl	"bloc-0"                        # -- Begin function bloc-0
	.p2align	4, 0x90
	.type	"bloc-0",@function
"bloc-0":                               # @bloc-0
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
	movq	%rdi, %rbx
	cmpq	$3, %rdi
	jle	.LBB22_2
# %bb.1:                                # %continue
	leaq	.Lformat_str(%rip), %r14
	leaq	.Lnewline(%rip), %r15
	leaq	.Lvide(%rip), %rdi
	leaq	".Ltexte_global-0"(%rip), %rsi
	callq	concat_strings@PLT
	movq	%r14, %rdi
	movq	%rax, %rsi
	xorl	%eax, %eax
	callq	printf@PLT
	movq	%r14, %rdi
	movq	%r15, %rsi
	xorl	%eax, %eax
	callq	printf@PLT
.LBB22_2:                               # %stop
	movq	%rbx, %rax
	popq	%rbx
	.cfi_def_cfa_offset 24
	popq	%r14
	.cfi_def_cfa_offset 16
	popq	%r15
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end22:
	.size	"bloc-0", .Lfunc_end22-"bloc-0"
	.cfi_endproc
                                        # -- End function
	.globl	"bloc-1"                        # -- Begin function bloc-1
	.p2align	4, 0x90
	.type	"bloc-1",@function
"bloc-1":                               # @bloc-1
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
	movq	%rdi, %rbx
	cmpq	$4, %rdi
	jge	.LBB23_2
# %bb.1:                                # %continue
	leaq	.Lformat_str(%rip), %r14
	leaq	.Lnewline(%rip), %r15
	leaq	.Lvide(%rip), %rdi
	leaq	".Ltexte_global-1"(%rip), %rsi
	callq	concat_strings@PLT
	movq	%r14, %rdi
	movq	%rax, %rsi
	xorl	%eax, %eax
	callq	printf@PLT
	movq	%r14, %rdi
	movq	%r15, %rsi
	xorl	%eax, %eax
	callq	printf@PLT
.LBB23_2:                               # %stop
	movq	%rbx, %rax
	popq	%rbx
	.cfi_def_cfa_offset 24
	popq	%r14
	.cfi_def_cfa_offset 16
	popq	%r15
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end23:
	.size	"bloc-1", .Lfunc_end23-"bloc-1"
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
	leaq	".LA-1-nom"(%rip), %rdi
	callq	demande_entier@PLT
	movq	%rax, %rdi
	callq	"bloc-0"@PLT
	movq	%rax, %rdi
	callq	"bloc-1"@PLT
	xorl	%eax, %eax
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end24:
	.size	main, .Lfunc_end24-main
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

	.type	.Lmoins,@object                 # @moins
.Lmoins:
	.asciz	"moins-"
	.size	.Lmoins, 7

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

	.type	buffer,@object                  # @buffer
	.comm	buffer,100,1
	.type	.Ldemande_str,@object           # @demande_str
	.section	.rodata.str1.16,"aMS",@progbits,1
	.p2align	4, 0x0
.Ldemande_str:
	.asciz	"Quelle valeur pour %s (%s) ?\n"
	.size	.Ldemande_str, 30

	.type	.Ltype_texte,@object            # @type_texte
	.section	.rodata.str1.1,"aMS",@progbits,1
.Ltype_texte:
	.asciz	"texte"
	.size	.Ltype_texte, 6

	.type	.Ltype_booleen,@object          # @type_booleen
.Ltype_booleen:
	.asciz	"bool\303\251en"
	.size	.Ltype_booleen, 9

	.type	.Lbooleen_invalide,@object      # @booleen_invalide
	.section	.rodata.str1.16,"aMS",@progbits,1
	.p2align	4, 0x0
.Lbooleen_invalide:
	.asciz	"Erreur : Le bool\303\251en '%s' est invalide.\n"
	.size	.Lbooleen_invalide, 41

	.type	.Lvingts,@object                # @vingts
	.section	.rodata,"a",@progbits
.Lvingts:
	.asciz	"vingts"
	.size	.Lvingts, 7

	.type	".Ljuste-et",@object            # @juste-et
	.section	.rodata.str1.1,"aMS",@progbits,1
".Ljuste-et":
	.asciz	"et"
	.size	".Ljuste-et", 3

	.type	.Ltype_entier,@object           # @type_entier
.Ltype_entier:
	.asciz	"entier"
	.size	.Ltype_entier, 7

	.type	.Lentier_invalide,@object       # @entier_invalide
	.section	.rodata.str1.16,"aMS",@progbits,1
	.p2align	4, 0x0
.Lentier_invalide:
	.asciz	"Erreur : L'entier '%s' est invalide.\n"
	.size	.Lentier_invalide, 38

	.type	".LA-1-nom",@object             # @A-1-nom
	.section	.rodata.str1.1,"aMS",@progbits,1
".LA-1-nom":
	.asciz	"A"
	.size	".LA-1-nom", 2

	.type	".Ltexte_global-0",@object      # @texte_global-0
".Ltexte_global-0":
	.asciz	"General"
	.size	".Ltexte_global-0", 8

	.type	".Ltexte_global-1",@object      # @texte_global-1
".Ltexte_global-1":
	.asciz	"kenobi"
	.size	".Ltexte_global-1", 7

	.section	".note.GNU-stack","",@progbits
