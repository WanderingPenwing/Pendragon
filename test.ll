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

@format_str = private constant [3 x i8] c"%s\00"

declare i32 @printf(i8*, ...)

define i8* @texte_centaine(i32 %x) {
entry:
	%pas_trop_gros = icmp slt i32 %x, 10
	br i1 %pas_trop_gros, label %pas_infini, label %infini
pas_infini:
	%pas_trop_petit = icmp eq i32 %x, 0
	br i1 %pas_trop_petit, label %infini, label %centaine
centaine:
	%juste_cent = icmp eq i32 %x, 1
	br i1 %juste_cent, label %pas_nombre, label %nombre
pas_nombre:
	%vide_str = getelementptr [1 x i8], [1 x i8]* @vide, i32 0, i32 0
	br label %cent
nombre:
	%nombre_ptr = getelementptr [20 x i8*], [20 x i8*]* @petits_nombres, i32 0, i32 %x
	%petit_nombre_str = load i8*, i8** %nombre_ptr
	%tiret_str = getelementptr [2 x i8], [2 x i8]* @tiret, i32 0, i32 0
	%concat_str = call i8* @concat_strings(i8* %petit_nombre_str, i8* %tiret_str)
	br label %cent
cent:
	%nombre_str = phi i8* [ %vide_str, %pas_nombre ], [ %concat_str, %nombre ]
	%cent_ptr = getelementptr [10 x i8*], [10 x i8*]* @dizaine, i32 0, i32 0
	%cent_str = load i8*, i8** %cent_ptr
	%nombre_complet = call i8* @concat_strings(i8* %nombre_str, i8* %cent_str)
	ret i8* %nombre_complet
infini:
	%infini_ptr = getelementptr [8 x i8*], [8 x i8*]* @separateurs, i32 0, i32 6
	%infini_str = load i8*, i8** %infini_ptr
	ret i8* %infini_str
}

define i8* @texte_unite(i32 %x) {
entry:
	%pas_trop_gros = icmp slt i32 %x, 20
	br i1 %pas_trop_gros, label %unite, label %infini
unite:
	%nombre_ptr = getelementptr [20 x i8*], [20 x i8*]* @petits_nombres, i32 0, i32 %x
	%nombre_str = load i8*, i8** %nombre_ptr
	ret i8* %nombre_str
infini:
	%infini_ptr = getelementptr [8 x i8*], [8 x i8*]* @separateurs, i32 0, i32 6
	%infini_str = load i8*, i8** %infini_ptr
	ret i8* %infini_str
}

define i8* @texte_dizaine(i32 %x) {
entry:
	%petit = icmp slt i32 %x, 20
	br i1 %petit, label %unite, label %dizaine
unite:
	%unite_str = call i8* @texte_unite(i32 %x)
	ret i8* %unite_str
dizaine:
	%chiffre_dizaine = sdiv i32 %x, 10
	%chiffre_unite = srem i32 %x, 10
	%dizaine_ptr = getelementptr [10 x i8*], [10 x i8*]* @dizaine, i32 0, i32 %chiffre_dizaine
	%dizaine_str = load i8*, i8** %dizaine_ptr
	%a_unite = icmp eq i32 %chiffre_unite, 0
	br i1 %a_unite, label %juste_dizaine, label %pitet-et
juste_dizaine:
	ret i8* %dizaine_str
pitet-et:
	%est-un = icmp eq i32 %chiffre_unite, 1
	%petite_dizaine = icmp slt i32 %chiffre_dizaine, 8
	%manque-et = mul i1 %est-un, %petite_dizaine
	br i1 %manque-et, label %affiche-et, label %pitet-special
affiche-et:
	%et_str = bitcast [4 x i8]* @et to i8*
	%concat_str = call i8* @concat_strings(i8* %dizaine_str, i8* %et_str)
	br label %pitet-special
pitet-special:
	%dizaine_finale_str = phi i8* [ %dizaine_str, %pitet-et ], [ %concat_str, %affiche-et ]
	%tiret_str = getelementptr [2 x i8], [2 x i8]* @tiret, i32 0, i32 0
	%dizaine_complete_str = call i8* @concat_strings(i8* %dizaine_finale_str, i8* %tiret_str)
	%sept = icmp eq i32 %chiffre_dizaine, 7
	%neuf = icmp eq i32 %chiffre_dizaine, 9
	%special = add i1 %sept, %neuf
	br i1 %special, label %unite-special, label %unite-simple
unite-special:
	%chiffre_special = add i32 %chiffre_unite, 10
	%unite_speciale_str = call i8* @texte_unite(i32 %chiffre_special)
	%petit_nombre_special = call i8* @concat_strings(i8* %dizaine_complete_str, i8* %unite_speciale_str)
	ret i8* %petit_nombre_special
unite-simple:
	%unite_simple_str = call i8* @texte_unite(i32 %chiffre_unite)
	%petit_nombre_simple = call i8* @concat_strings(i8* %dizaine_complete_str, i8* %unite_simple_str)
	ret i8* %petit_nombre_simple
}

define i8* @texte_petit_nombre(i32 %x) {
entry:
	%a_centaine = icmp slt i32 %x, 100
	%chiffre_centaine = sdiv i32 %x, 100
	%nombre_dizaine = srem i32 %x, 100
	br i1 %a_centaine, label %pas_centaine, label %centaine
pas_centaine:
	%vide_str = getelementptr [1 x i8], [1 x i8]* @vide, i32 0, i32 0
	br label %dizaine
centaine:
	%chiffre_centaine_str = call i8* @texte_centaine(i32 %chiffre_centaine)
	%a_dizaine = icmp slt i32 0, %nombre_dizaine
	br i1 %a_dizaine, label %separateur, label %juste_centaine
juste_centaine:
	ret i8* %chiffre_centaine_str
separateur:
	%tiret_str = getelementptr [2 x i8], [2 x i8]* @tiret, i32 0, i32 0
	%centaine_str = call i8* @concat_strings(i8* %chiffre_centaine_str, i8* %tiret_str)
	br label %dizaine
dizaine:
	%start_str = phi i8* [ %vide_str, %pas_centaine ], [ %centaine_str, %separateur ]
	%dizaine_str = call i8* @texte_dizaine(i32 %nombre_dizaine)
	%nombre_complet = call i8* @concat_strings(i8* %start_str, i8* %dizaine_str)
	ret i8* %nombre_complet
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

define i8* @texte_nombre(i64 %x) {
entry:
	%est_zero = icmp eq i64 %x, 0
	br i1 %est_zero, label %affiche_zero, label %nombre
affiche_zero:
	%zero_ptr = getelementptr [20 x i8*], [20 x i8*]* @petits_nombres, i32 0, i32 0
	%zero_str = load i8*, i8** %zero_ptr
	ret i8* %zero_str
nombre:
	%puissance = call i32 @log_mille(i64 %x)
	%echelle = call i64 @mille_puissance(i32 %puissance)
	
	%valeur = sdiv i64 %x, %echelle
	%reste = srem i64 %x, %echelle

	%petite-valeur = trunc i64 %valeur to i32
	
	%petit_nombre_str = call i8* @texte_petit_nombre(i32 %petite-valeur)

	%petit-nombre = icmp eq i32 %puissance, 0
	br i1 %petit-nombre, label %petit, label %separateur
petit:
	ret i8* %petit_nombre_str
separateur:
	%tiret_str = getelementptr [2 x i8], [2 x i8]* @tiret, i32 0, i32 0
	%nombre_tiret = call i8* @concat_strings(i8* %petit_nombre_str, i8* %tiret_str)
	%separateur_ptr = getelementptr [8 x i8*], [8 x i8*]* @separateurs, i32 0, i32 %puissance
	%separateur_str = load i8*, i8** %separateur_ptr
	%nombre_separateur = call i8* @concat_strings(i8* %nombre_tiret, i8* %separateur_str)

	%un = icmp slt i32 %petite-valeur, 2
	%mille = icmp slt i32 %puissance, 2
	%pas-s = add i1 %un, %mille
	br i1 %pas-s, label %pitet-recursion, label %pluriel
pluriel:
	%s_str = bitcast [2 x i8]* @s to i8*
	%nombre_pluriel = call i8* @concat_strings(i8* %nombre_separateur, i8* %s_str)
	br label %pitet-recursion
pitet-recursion:
	%nombre_complet = phi i8* [ %nombre_separateur, %separateur ], [ %nombre_pluriel, %pluriel ]
	%reste_zero = icmp eq i64 %reste, 0
	br i1 %reste_zero, label %fin, label %recursion
fin:
	ret i8* %nombre_complet
recursion:
	%nombre_complet_tiret = call i8* @concat_strings(i8* %nombre_complet, i8* %tiret_str)
	%suite_str = call i8* @texte_nombre(i64 %reste)
	%nombre_avec_suite = call i8* @concat_strings(i8* %nombre_complet_tiret, i8* %suite_str)
	ret i8* %nombre_avec_suite
}

define i8* @texte_booleen(i1 %x) {
	%bool = zext i1 %x to i32
	%bool_ptr = getelementptr [2 x i8*], [2 x i8*]* @booleen, i32 0, i32 %bool
	%bool_str = load i8*, i8** %bool_ptr
	ret i8* %bool_str
}

; Definition of main function
;define i32 @main() { ; i32()*
;	call void @affiche_nombre(i64 25781)
;	call void @nouvelle_ligne()
;	ret i32 0
;}

define i8* @concat_strings(i8* %str1, i8* %str2) {
entry:
  %len1 = call i64 @strlen(i8* %str1)
  %len2 = call i64 @strlen(i8* %str2)
  %total_len = add i64 %len1, %len2
  %alloc_size = add i64 %total_len, 1
  %result = call i8* @malloc(i64 %alloc_size)

  call void @memcpy(i8* %result, i8* %str1, i64 %len1)
  %result_ptr = getelementptr i8, i8* %result, i64 %len1
  call void @memcpy(i8* %result_ptr, i8* %str2, i64 %len2)
  
  %null_pos = getelementptr i8, i8* %result, i64 %total_len
  store i8 0, i8* %null_pos
  
  ret i8* %result
}

declare i64 @strlen(i8*)
declare i8* @malloc(i64)
declare void @memcpy(i8*, i8*, i64)

define i32 @main() {
%fmt_ptr = getelementptr [3 x i8], [3 x i8]* @format_str, i32 0, i32 0
%nouvelle_ligne = getelementptr [3 x i8], [3 x i8]* @newline, i32 0, i32 0
%expression_texte-0-0 = getelementptr [1 x i8], [1 x i8]* @vide, i32 0, i32 0
%expression_nombre-0-1 = add i64 36004, 0
%expression_nombre-0-fin = add i64 %expression_nombre-0-1, 0
%expression_texte-0-1 = call i8* @texte_nombre(i64 %expression_nombre-0-fin)
%expression_texte-0-2 = call i8* @concat_strings(i8* %expression_texte-0-0, i8* %expression_texte-0-1)
%expression_nombre-1-1 = add i64 4, 0
%expression_nombre-1-2 = add i64 17, 0
%expression_nombre-1-3 = mul i64 %expression_nombre-1-1, %expression_nombre-1-2
%expression_nombre-1-4 = add i64 7, 0
%expression_nombre-1-5 = sub i64 %expression_nombre-1-3, %expression_nombre-1-4
%expression_nombre-1-fin = add i64 %expression_nombre-1-5, 0
%expression_texte-0-3 = call i8* @texte_nombre(i64 %expression_nombre-1-fin)
%expression_texte-0-4 = call i8* @concat_strings(i8* %expression_texte-0-2, i8* %expression_texte-0-3)
%expression_texte-0-fin = getelementptr i8, i8* %expression_texte-0-4, i32 0
call i32 (i8*, ...) @printf(i8* %fmt_ptr, i8* %expression_texte-0-fin)
call i32 (i8*, ...) @printf(i8* %fmt_ptr, i8* %nouvelle_ligne)
%expression_texte-1-0 = getelementptr [1 x i8], [1 x i8]* @vide, i32 0, i32 0
%expression_booleen-0-1 = add i1 1, 0
%expression_booleen-0-fin = add i1 %expression_booleen-0-1, 0
%expression_texte-1-1 = call i8* @texte_booleen(i1 %expression_booleen-0-fin)
%expression_texte-1-2 = call i8* @concat_strings(i8* %expression_texte-1-0, i8* %expression_texte-1-1)
%expression_texte-1-fin = getelementptr i8, i8* %expression_texte-1-2, i32 0
call i32 (i8*, ...) @printf(i8* %fmt_ptr, i8* %expression_texte-1-fin)
call i32 (i8*, ...) @printf(i8* %fmt_ptr, i8* %nouvelle_ligne)
%expression_texte-2-0 = getelementptr [1 x i8], [1 x i8]* @vide, i32 0, i32 0
%expression_booleen-1-1 = add i1 0, 0
%expression_booleen-1-2 = add i1 0, 0
%expression_booleen-1-3 = xor i1 %expression_booleen-1-2, true
%expression_booleen-1-4 = or i1 %expression_booleen-1-3, %expression_booleen-1-2
%expression_booleen-1-fin = add i1 %expression_booleen-1-4, 0
%expression_texte-2-1 = call i8* @texte_booleen(i1 %expression_booleen-1-fin)
%expression_texte-2-2 = call i8* @concat_strings(i8* %expression_texte-2-0, i8* %expression_texte-2-1)
%expression_texte-2-fin = getelementptr i8, i8* %expression_texte-2-2, i32 0
call i32 (i8*, ...) @printf(i8* %fmt_ptr, i8* %expression_texte-2-fin)
call i32 (i8*, ...) @printf(i8* %fmt_ptr, i8* %nouvelle_ligne)
%A-0 = add i64 0, 0
%expression_nombre-2-1 = add i64 1, 0
%expression_nombre-2-fin = add i64 %expression_nombre-2-1, 0
%A-1 = add i64 %expression_nombre-2-fin, 0
%expression_texte-3-0 = getelementptr [1 x i8], [1 x i8]* @vide, i32 0, i32 0
%expression_nombre-3-1 = add i64 %A-1, 0
%expression_nombre-3-2 = add i64 7, 0
%expression_nombre-3-3 = mul i64 %expression_nombre-3-1, %expression_nombre-3-2
%expression_nombre-3-fin = add i64 %expression_nombre-3-3, 0
%expression_texte-3-1 = call i8* @texte_nombre(i64 %expression_nombre-3-fin)
%expression_texte-3-2 = call i8* @concat_strings(i8* %expression_texte-3-0, i8* %expression_texte-3-1)
%expression_texte-3-fin = getelementptr i8, i8* %expression_texte-3-2, i32 0
call i32 (i8*, ...) @printf(i8* %fmt_ptr, i8* %expression_texte-3-fin)
call i32 (i8*, ...) @printf(i8* %fmt_ptr, i8* %nouvelle_ligne)
%expression_nombre-4-1 = add i64 %A-1, 0
%expression_nombre-4-2 = add i64 2, 0
%expression_nombre-4-3 = add i64 %expression_nombre-4-1, %expression_nombre-4-2
%expression_nombre-4-fin = add i64 %expression_nombre-4-3, 0
%A-2 = add i64 %expression_nombre-4-fin, 0
%expression_texte-4-0 = getelementptr [1 x i8], [1 x i8]* @vide, i32 0, i32 0
%expression_nombre-5-1 = add i64 %A-2, 0
%expression_nombre-5-2 = add i64 1, 0
%expression_nombre-5-3 = add i64 %expression_nombre-5-1, %expression_nombre-5-2
%expression_nombre-5-fin = add i64 %expression_nombre-5-3, 0
%expression_texte-4-1 = call i8* @texte_nombre(i64 %expression_nombre-5-fin)
%expression_texte-4-2 = call i8* @concat_strings(i8* %expression_texte-4-0, i8* %expression_texte-4-1)
%expression_texte-4-fin = getelementptr i8, i8* %expression_texte-4-2, i32 0
call i32 (i8*, ...) @printf(i8* %fmt_ptr, i8* %expression_texte-4-fin)
call i32 (i8*, ...) @printf(i8* %fmt_ptr, i8* %nouvelle_ligne)
%B-0 = add i1 0, 0
%expression_texte-5-0 = getelementptr [1 x i8], [1 x i8]* @vide, i32 0, i32 0
%expression_booleen-2-1 = add i1 %B-0, 0
%expression_booleen-2-fin = add i1 %expression_booleen-2-1, 0
%expression_texte-5-1 = call i8* @texte_booleen(i1 %expression_booleen-2-fin)
%expression_texte-5-2 = call i8* @concat_strings(i8* %expression_texte-5-0, i8* %expression_texte-5-1)
%expression_texte-5-fin = getelementptr i8, i8* %expression_texte-5-2, i32 0
call i32 (i8*, ...) @printf(i8* %fmt_ptr, i8* %expression_texte-5-fin)
call i32 (i8*, ...) @printf(i8* %fmt_ptr, i8* %nouvelle_ligne)
%expression_booleen-3-1 = add i1 1, 0
%expression_booleen-3-2 = add i1 0, 0
%expression_booleen-3-3 = or i1 %expression_booleen-3-2, %expression_booleen-3-1
%expression_booleen-3-fin = add i1 %expression_booleen-3-3, 0
%B-1 = add i1 %expression_booleen-3-fin, 0
%expression_texte-6-0 = getelementptr [1 x i8], [1 x i8]* @vide, i32 0, i32 0
%expression_booleen-4-1 = add i1 1, 0
%expression_booleen-4-2 = add i1 %B-1, 0
%expression_booleen-4-3 = and i1 %expression_booleen-4-2, %expression_booleen-4-1
%expression_booleen-4-fin = add i1 %expression_booleen-4-3, 0
%expression_texte-6-1 = call i8* @texte_booleen(i1 %expression_booleen-4-fin)
%expression_texte-6-2 = call i8* @concat_strings(i8* %expression_texte-6-0, i8* %expression_texte-6-1)
%expression_texte-6-fin = getelementptr i8, i8* %expression_texte-6-2, i32 0
call i32 (i8*, ...) @printf(i8* %fmt_ptr, i8* %expression_texte-6-fin)
call i32 (i8*, ...) @printf(i8* %fmt_ptr, i8* %nouvelle_ligne)

ret i32 0
}