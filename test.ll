@zero = private constant [6 x i8] c"zéro\00"
@un = private constant [3 x i8] c"un\00"
@deux = private constant [5 x i8] c"deux\00"
@trois = private constant [6 x i8] c"trois\00"
@quatre = private constant [7 x i8] c"quatre\00"
@cinq = private constant [5 x i8] c"cinq\00"
@six = private constant [4 x i8] c"six\00"
@sept = private constant [5 x i8] c"sept\00"
@huit = private constant [5 x i8] c"huit\00"
@neuf = private constant [5 x i8] c"neuf\00"
@dix = private constant [4 x i8] c"dix\00"
@onze = private constant [5 x i8] c"onze\00"
@douze = private constant [6 x i8] c"douze\00"
@treize = private constant [7 x i8] c"treize\00"
@quatorze = private constant [9 x i8] c"quatorze\00"
@quinze = private constant [7 x i8] c"quinze\00"
@seize = private constant [6 x i8] c"seize\00"
@dix-sept = private constant [9 x i8] c"dix-sept\00"
@dix-huit = private constant [9 x i8] c"dix-huit\00"
@dix-neuf = private constant [9 x i8] c"dix-neuf\00"

@vingt = private constant [6 x i8] c"vingt\00"
@trente = private constant [7 x i8] c"trente\00"
@quarante = private constant [9 x i8] c"quarante\00"
@cinquante = private constant [10 x i8] c"cinquante\00"
@soixante = private constant [9 x i8] c"soixante\00"
@quatre-vingts = private constant [14 x i8] c"quatre-vingts\00"
@cent = private constant [5 x i8] c"cent\00"

@vide = private constant [1 x i8] c"\00"
@mille = private constant [6 x i8] c"mille\00"
@million = private constant [8 x i8] c"million\00"
@milliard = private constant [9 x i8] c"milliard\00"
@billion = private constant [8 x i8] c"billion\00"
@billiard = private constant [9 x i8] c"billiard\00"
@trillion = private constant [9 x i8] c"trillion\00"
@infini = private constant [7 x i8] c"infini\00"

@vrai = private constant [5 x i8] c"vrai\00"
@faux = private constant [5 x i8] c"faux\00"

@newline = private unnamed_addr constant [2 x i8] c"\0A\00"
@tiret = private unnamed_addr constant [2 x i8] c"-\00"
@et = private unnamed_addr constant [4 x i8] c"-et\00"
@s = private unnamed_addr constant [2 x i8] c"s\00"
@espace = private unnamed_addr constant [2 x i8] c" \00"

; Create an array of i8* pointers, where each points to one of the strings
@petits_nombres = global [20 x i8*] [
    i8* getelementptr inbounds ([6 x i8], [6 x i8]* @zero, i32 0, i32 0),
    i8* getelementptr inbounds ([3 x i8], [3 x i8]* @un, i32 0, i32 0),
    i8* getelementptr inbounds ([5 x i8], [5 x i8]* @deux, i32 0, i32 0),
    i8* getelementptr inbounds ([6 x i8], [6 x i8]* @trois, i32 0, i32 0),
    i8* getelementptr inbounds ([7 x i8], [7 x i8]* @quatre, i32 0, i32 0),
    i8* getelementptr inbounds ([5 x i8], [5 x i8]* @cinq, i32 0, i32 0),
    i8* getelementptr inbounds ([4 x i8], [4 x i8]* @six, i32 0, i32 0),
    i8* getelementptr inbounds ([5 x i8], [5 x i8]* @sept, i32 0, i32 0),
    i8* getelementptr inbounds ([5 x i8], [5 x i8]* @huit, i32 0, i32 0),
    i8* getelementptr inbounds ([5 x i8], [5 x i8]* @neuf, i32 0, i32 0),
    i8* getelementptr inbounds ([4 x i8], [4 x i8]* @dix, i32 0, i32 0),
    i8* getelementptr inbounds ([5 x i8], [5 x i8]* @onze, i32 0, i32 0),
    i8* getelementptr inbounds ([6 x i8], [6 x i8]* @douze, i32 0, i32 0),
    i8* getelementptr inbounds ([7 x i8], [7 x i8]* @treize, i32 0, i32 0),
    i8* getelementptr inbounds ([9 x i8], [9 x i8]* @quatorze, i32 0, i32 0),
    i8* getelementptr inbounds ([7 x i8], [7 x i8]* @quinze, i32 0, i32 0),
    i8* getelementptr inbounds ([6 x i8], [6 x i8]* @seize, i32 0, i32 0),
    i8* getelementptr inbounds ([9 x i8], [9 x i8]* @dix-sept, i32 0, i32 0),
    i8* getelementptr inbounds ([9 x i8], [9 x i8]* @dix-huit, i32 0, i32 0),
    i8* getelementptr inbounds ([9 x i8], [9 x i8]* @dix-neuf, i32 0, i32 0)
]

@dizaine = global [10 x i8*] [
    i8* getelementptr inbounds ([5 x i8], [5 x i8]* @cent, i32 0, i32 0),
    i8* getelementptr inbounds ([4 x i8], [4 x i8]* @dix, i32 0, i32 0),
    i8* getelementptr inbounds ([6 x i8], [6 x i8]* @vingt, i32 0, i32 0),
    i8* getelementptr inbounds ([7 x i8], [7 x i8]* @trente, i32 0, i32 0),
    i8* getelementptr inbounds ([9 x i8], [9 x i8]* @quarante, i32 0, i32 0),
    i8* getelementptr inbounds ([10 x i8], [10 x i8]* @cinquante, i32 0, i32 0),
    i8* getelementptr inbounds ([9 x i8], [9 x i8]* @soixante, i32 0, i32 0),
    i8* getelementptr inbounds ([9 x i8], [9 x i8]* @soixante, i32 0, i32 0),
    i8* getelementptr inbounds ([14 x i8], [14 x i8]* @quatre-vingts, i32 0, i32 0),
    i8* getelementptr inbounds ([14 x i8], [14 x i8]* @quatre-vingts, i32 0, i32 0)
]

@separateurs = global [8 x i8*] [
	i8* getelementptr inbounds ([1 x i8], [1 x i8]* @vide, i32 0, i32 0),
    i8* getelementptr inbounds ([6 x i8], [6 x i8]* @mille, i32 0, i32 0),
    i8* getelementptr inbounds ([8 x i8], [8 x i8]* @million, i32 0, i32 0),
    i8* getelementptr inbounds ([9 x i8], [9 x i8]* @milliard, i32 0, i32 0),
    i8* getelementptr inbounds ([8 x i8], [8 x i8]* @billion, i32 0, i32 0),
    i8* getelementptr inbounds ([9 x i8], [9 x i8]* @billiard, i32 0, i32 0),
    i8* getelementptr inbounds ([9 x i8], [9 x i8]* @trillion, i32 0, i32 0),
    i8* getelementptr inbounds ([7 x i8], [7 x i8]* @infini, i32 0, i32 0)
]

@booleen = global [2 x i8*] [
	i8* getelementptr inbounds ([5 x i8], [5 x i8]* @faux, i32 0, i32 0),
    i8* getelementptr inbounds ([5 x i8], [5 x i8]* @vrai, i32 0, i32 0)
]

declare i32 @printf(i8*, ...)

define void @affiche_centaine(i32 %x) {
entry:
	%pas_trop_gros = icmp slt i32 %x, 10
	br i1 %pas_trop_gros, label %pas_infini, label %infini
pas_infini:
	%pas_trop_petit = icmp eq i32 %x, 0
	br i1 %pas_trop_petit, label %fin, label %centaine
centaine:
	%juste_cent = icmp eq i32 %x, 1
	br i1 %juste_cent, label %cent, label %nombre
nombre:
	%nombre_ptr = getelementptr [10 x i8*], [10 x i8*]* @petits_nombres, i32 0, i32 %x
	%nombre_str = load i8*, i8** %nombre_ptr
	call i32 @printf(i8* %nombre_str)
	call void @affiche_tiret()
	br label %cent
cent:
	%cent_ptr = getelementptr [7 x i8*], [7 x i8*]* @dizaine, i32 0, i32 0
	%cent_str = load i8*, i8** %cent_ptr
	call i32 @printf(i8* %cent_str)
	br label %fin
infini:
	%infini_ptr = getelementptr [7 x i8*], [7 x i8*]* @separateurs, i32 0, i32 6
	%infini_str = load i8*, i8** %infini_ptr
	call i32 @printf(i8* %infini_str)
	br label %fin
fin:
	ret void
}

define void @affiche_unite(i32 %x) {
entry:
	%pas_trop_gros = icmp slt i32 %x, 20
	br i1 %pas_trop_gros, label %unite, label %infini

unite:
	%nombre_ptr = getelementptr [10 x i8*], [10 x i8*]* @petits_nombres, i32 0, i32 %x
	%nombre_str = load i8*, i8** %nombre_ptr
	call i32 @printf(i8* %nombre_str)
	br label %fin

infini:
	%infini_ptr = getelementptr [7 x i8*], [7 x i8*]* @separateurs, i32 0, i32 6
	%infini_str = load i8*, i8** %infini_ptr
	call i32 @printf(i8* %infini_str)
	br label %fin

fin:
	ret void
}

define void @affiche_dizaine(i32 %x) {
entry:
	%petit = icmp slt i32 %x, 20
	br i1 %petit, label %unite, label %dizaine
unite:
	call void @affiche_unite(i32 %x)
	br label %fin
dizaine:
	%chiffre_dizaine = sdiv i32 %x, 10
	%chiffre_unite = srem i32 %x, 10
	%dizaine_ptr = getelementptr [10 x i8*], [10 x i8*]* @dizaine, i32 0, i32 %chiffre_dizaine
	%dizaine_str = load i8*, i8** %dizaine_ptr
	call i32 @printf(i8* %dizaine_str)
	%a_unite = icmp eq i32 %chiffre_unite, 0
	br i1 %a_unite, label %fin, label %pitet-et
pitet-et:
	%est-un = icmp eq i32 %chiffre_unite, 1
	%petite_dizaine = icmp slt i32 %chiffre_dizaine, 8
	%manque-et = mul i1 %est-un, %petite_dizaine
	br i1 %manque-et, label %affiche-et, label %pitet-special
affiche-et:
	%et_str = bitcast [4 x i8]* @et to i8*
	call i32 @printf(i8* %et_str)
	br label %pitet-special
pitet-special:
	call void @affiche_tiret()
	%sept = icmp eq i32 %chiffre_dizaine, 7
	%neuf = icmp eq i32 %chiffre_dizaine, 9
	%special = add i1 %sept, %neuf
	br i1 %special, label %unite-special, label %unite-simple
unite-special:
	%chiffre_special = add i32 %chiffre_unite, 10
	call void @affiche_unite(i32 %chiffre_special)
	br label %fin
unite-simple:
	call void @affiche_unite(i32 %chiffre_unite)
	br label %fin
fin:
	ret void
}

define void @affiche_petit_nombre(i32 %x) {
entry:
	%a_centaine = icmp slt i32 %x, 100
	%chiffre_centaine = sdiv i32 %x, 100
	%nombre_dizaine = srem i32 %x, 100
	br i1 %a_centaine, label %dizaine, label %centaine

centaine:
	call void @affiche_centaine(i32 %chiffre_centaine)
	%a_dizaine = icmp slt i32 0, %nombre_dizaine
	br i1 %a_dizaine, label %separateur, label %fin

separateur:
	call void @affiche_tiret()
	br label %dizaine

dizaine:
	call void @affiche_dizaine(i32 %nombre_dizaine)
	br label %fin

fin:
	ret void
}

define void @affiche_tiret() {
	%tiret_str = bitcast [2 x i8]* @tiret to i8*
	call i32 @printf(i8* %tiret_str)
	ret void
}

define void @nouvelle_ligne() {
	%newline_str = bitcast [2 x i8]* @newline to i8*
	call i32 @printf(i8* %newline_str)
	ret void
}

define void @affiche_espace() {
	%espace_str = bitcast [2 x i8]* @espace to i8*
	call i32 @printf(i8* %espace_str)
	ret void
}

define i64 @mille_puissance(i32 %x) {
entry:
	%est-zero = icmp eq i32 %x, 0
	br i1 %est-zero, label %zero, label %recursion
zero:
	ret i64 1
recursion:
	%x_suivant = sub i32 %x, 1
	%resultat = call i64 @mille_puissance(i32 %x_suivant)
	%fois_mille = mul i64 %resultat, 1000
	ret i64 %fois_mille
}

define i32 @log_mille(i64 %x) {
entry:
	%est-zero = icmp slt i64 %x, 1000
	br i1 %est-zero, label %zero, label %recursion
zero:
	ret i32 0
recursion:
	%x_suivant = sdiv i64 %x, 1000
	%resultat = call i32 @log_mille(i64 %x_suivant)
	%plus_un = add i32 %resultat, 1
	ret i32 %plus_un
}

define void @affiche_nombre(i64 %x) {
entry:
	%est_zero = icmp eq i64 %x, 0
	br i1 %est_zero, label %affiche_zero, label %nombre
affiche_zero:
	%zero_ptr = getelementptr [10 x i8*], [10 x i8*]* @petits_nombres, i32 0, i32 0
	%zero_str = load i8*, i8** %zero_ptr
	call i32 @printf(i8* %zero_str)
	br label %fin
nombre:
	%puissance = call i32 @log_mille(i64 %x)
	%echelle = call i64 @mille_puissance(i32 %puissance)
	
	%valeur = sdiv i64 %x, %echelle
	%reste = srem i64 %x, %echelle

	%petite-valeur = trunc i64 %valeur to i32
	
	call void @affiche_petit_nombre(i32 %petite-valeur)

	%petit-nombre = icmp eq i32 %puissance, 0
	br i1 %petit-nombre, label %fin, label %separateur
separateur:
	call void @affiche_tiret()
	%separateur_ptr = getelementptr [8 x i8*], [8 x i8*]* @separateurs, i32 0, i32 %puissance
	%separateur_str = load i8*, i8** %separateur_ptr
	call i32 @printf(i8* %separateur_str)
	%un = icmp slt i32 %petite-valeur, 2
	%mille = icmp slt i32 %puissance, 2
	%pas-s = add i1 %un, %mille
	br i1 %pas-s, label %pitet-recursion, label %affiche-s
affiche-s:
	%s_str = bitcast [2 x i8]* @s to i8*
	call i32 @printf(i8* %s_str)
	br label %pitet-recursion
pitet-recursion:
	%reste_zero = icmp eq i64 %reste, 0
	br i1 %reste_zero, label %fin, label %recursion
recursion:
	call void @affiche_tiret()
	call void @affiche_nombre(i64 %reste)
	br label %fin
fin:
	ret void
}

define void @affiche_booleen(i1 %x) {
	%bool = zext i1 %x to i32
	%bool_ptr = getelementptr [2 x i8*], [2 x i8*]* @booleen, i32 0, i32 %bool
	%bool_str = load i8*, i8** %bool_ptr
	call i32 @printf(i8* %bool_str)
	ret void
}

; Definition of main function
; define i32 @main() { ; i32()*
; 	call void @affiche_nombre(i64 25781)
;	call void @nouvelle_ligne()
;	ret i32 0
;}

define i32 @main() {
%expression_nombre-0-1 = add i64 36004, 0
%expression_nombre-0-fin = add i64 %expression_nombre-0-1, 0
call void @affiche_nombre(i64 %expression_nombre-0-fin)
%expression_nombre-1-1 = add i64 4, 0
%expression_nombre-1-2 = add i64 17, 0
%expression_nombre-1-3 = mul i64 %expression_nombre-1-1, %expression_nombre-1-2
%expression_nombre-1-4 = add i64 7, 0
%expression_nombre-1-5 = sub i64 %expression_nombre-1-3, %expression_nombre-1-4
%expression_nombre-1-fin = add i64 %expression_nombre-1-5, 0
call void @affiche_nombre(i64 %expression_nombre-1-fin)
call void @nouvelle_ligne()
%expression_booleen-0-1 = add i1 1, 0
%expression_booleen-0-fin = add i1 %expression_booleen-0-1, 0
call void @affiche_booleen(i1 %expression_booleen-0-fin)
call void @nouvelle_ligne()
%expression_booleen-1-1 = add i1 0, 0
%expression_booleen-1-2 = add i1 0, 0
%expression_booleen-1-3 = xor i1 %expression_booleen-1-2, true
%expression_booleen-1-4 = or i1 %expression_booleen-1-3, %expression_booleen-1-2
%expression_booleen-1-fin = add i1 %expression_booleen-1-4, 0
call void @affiche_booleen(i1 %expression_booleen-1-fin)
call void @nouvelle_ligne()
%A-0 = add i64 0, 0
%expression_nombre-2-1 = add i64 1, 0
%expression_nombre-2-fin = add i64 %expression_nombre-2-1, 0
%A-1 = add i64 %expression_nombre-2-fin, 0

ret i32 0
}