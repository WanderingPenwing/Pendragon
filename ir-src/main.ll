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
@moins = private unnamed_addr constant [7 x i8] c"moins-\00"

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
	%vide_str = getelementptr [1 x i8], [1 x i8]* @vide, i32 0, i32 0
	%est_negatif = icmp slt i64 %x, 0
	br i1 %est_negatif, label %negatif, label %positif
negatif:
	%pos_x = sub i64 0, %x
	%moins_str = getelementptr [7 x i8], [7 x i8]* @moins, i32 0, i32 0
	br label %positif
positif:
	%base_str = phi i8* [%vide_str, %nombre], [%moins_str, %negatif]
	%abs-x = phi i64 [%x, %nombre], [%pos_x, %negatif]
	%puissance = call i32 @log_mille(i64 %abs-x)
	%echelle = call i64 @mille_puissance(i32 %puissance)
	
	%valeur = sdiv i64 %abs-x, %echelle
	%reste = srem i64 %abs-x, %echelle

	%petite-valeur = trunc i64 %valeur to i32

	%valeur_str = call i8* @texte_petit_nombre(i32 %petite-valeur)
	%petit_nombre_str = call i8* @concat_strings(i8* %base_str, i8* %valeur_str)

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
	%un-mille = and i1 %un, %mille
	br i1 %un-mille, label %pas-nombre, label %autre
pas-nombre:
	br label %pitet-recursion
autre:
	%pas-s = or i1 %un, %mille
	br i1 %pas-s, label %pitet-recursion, label %pluriel
pluriel:
	%s_str = bitcast [2 x i8]* @s to i8*
	%nombre_pluriel = call i8* @concat_strings(i8* %nombre_separateur, i8* %s_str)
	br label %pitet-recursion
pitet-recursion:
	%nombre_complet = phi i8* [ %nombre_separateur, %autre ], [ %nombre_pluriel, %pluriel ], [ %separateur_str, %pas-nombre ]
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

define i1 @compare_texte(i8* %str1, i8* %str2, i1 %egalite) {
	%cmp = call i32 @strcmp(i8* %str1, i8* %str2)
	%is_equal = icmp ne i32 %cmp, 0
	%resultat = xor i1 %is_equal, %egalite
	ret i1 %resultat
}

declare i64 @strlen(i8*)
declare i8* @malloc(i64)
declare void @memcpy(i8*, i8*, i64)
declare i32 @strcmp(i8*, i8*)
declare i8* @gets(i8*)

@buffer = common global [100 x i8] zeroinitializer, align 1

define i8* @read_line() {
entry:
    %buffer_ptr = getelementptr inbounds [100 x i8], i8* @buffer, i32 0, i32 0
    %call = call i8* @gets(i8* %buffer_ptr)  ; Read input

    ; Check if input was read successfully
    %is_null = icmp eq i8* %call, null
    br i1 %is_null, label %return_null, label %return_buffer

return_null:
    ret i8* null

return_buffer:
    ret i8* %buffer_ptr
}

@demande_str = private unnamed_addr constant [30 x i8] c"Quelle valeur pour %s (%s) ?\0A\00"
@type_texte = private unnamed_addr constant [6 x i8] c"texte\00"

define i8* @demande_texte(i8* %nom_variable) {
entry:
	%demande_fmt = getelementptr [30 x i8], [30 x i8]* @demande_str, i32 0, i32 0
	%type_texte_str = getelementptr [6 x i8], [6 x i8]* @type_texte, i32 0, i32 0
	call i32 (i8*, ...) @printf(i8* %demande_fmt, i8* %nom_variable, i8* %type_texte_str)
	%reponse = call i8* @read_line()
	%is_null = icmp eq ptr %reponse, null
	br i1 %is_null, label %vide, label %texte
texte:
	ret i8* %reponse
vide:
	%vide_str = getelementptr [1 x i8], [1 x i8]* @vide, i32 0, i32 0
	ret i8* %vide_str
}

@type_booleen = private unnamed_addr constant [9 x i8] c"booléen\00"
@booleen_invalide = private unnamed_addr constant [41 x i8] c"Erreur : Le booléen '%s' est invalide.\0A\00"

define i1 @demande_booleen(i8* %nom_variable) {
entry:
	%demande_fmt = getelementptr [30 x i8], [30 x i8]* @demande_str, i32 0, i32 0
	%type_booleen_str = getelementptr [9 x i8], [9 x i8]* @type_booleen, i32 0, i32 0
	call i32 (i8*, ...) @printf(i8* %demande_fmt, i8* %nom_variable, i8* %type_booleen_str)
	%reponse = call i8* @read_line()
	%is_null = icmp eq ptr %reponse, null
	br i1 %is_null, label %vide, label %texte
vide:
	%vide_str = getelementptr [1 x i8], [1 x i8]* @vide, i32 0, i32 0
	br label %redemande
texte:
	%vrai_ptr = getelementptr [2 x i8*], [2 x i8*]* @booleen, i32 0, i32 1
	%vrai_str = load i8*, i8** %vrai_ptr
	%est_vrai = call i1 @compare_texte(i8* %reponse, i8* %vrai_str, i1 1)
	br i1 %est_vrai, label %vrai, label %pas-vrai
vrai:
	ret i1 1
pas-vrai:
	%faux_ptr = getelementptr [2 x i8*], [2 x i8*]* @booleen, i32 0, i32 0
	%faux_str = load i8*, i8** %faux_ptr
	%est_faux = call i1 @compare_texte(i8* %reponse, i8* %faux_str, i1 1)
	br i1 %est_faux, label %faux, label %redemande
faux:
	ret i1 0
redemande:
	%erreur = phi i8* [%vide_str, %vide], [%reponse, %pas-vrai]
	%erreur_fmt = getelementptr [41 x i8], [41 x i8]* @booleen_invalide, i32 0, i32 0
	call i32 (i8*, ...) @printf(i8* %erreur_fmt, i8* %erreur)
	%nouvelle-reponse = call i1 @demande_booleen(i8* %nom_variable)
	ret i1 %nouvelle-reponse
}

@vingts = private constant [7 x i8] c"vingts\00"

define i1 @verifie_unite(i8* %mot, i32 %x) {
	%nombre_ptr = getelementptr [20 x i8*], [20 x i8*]* @petits_nombres, i32 0, i32 %x
	%nombre_str = load i8*, i8** %nombre_ptr
	%resultat = call i1 @compare_texte(i8* %mot, i8* %nombre_str, i1 1)
	ret i1 %resultat
}

define i1 @verifie_dizaine(i8* %mot, i32 %x) {
	%nombre_ptr = getelementptr [10 x i8*], [10 x i8*]* @dizaine, i32 0, i32 %x
	%nombre_str = load i8*, i8** %nombre_ptr
	%resultat = call i1 @compare_texte(i8* %mot, i8* %nombre_str, i1 1)
	ret i1 %resultat
}

define i1 @verifie_separateur(i8* %mot, i32 %x) {
	%nombre_ptr = getelementptr [8 x i8*], [8 x i8*]* @separateurs, i32 0, i32 %x
	%nombre_str = load i8*, i8** %nombre_ptr
	%resultat = call i1 @compare_texte(i8* %mot, i8* %nombre_str, i1 1)
	ret i1 %resultat
}

define i64 @mot_comme_unite(i8* %mot) {
entry:
	%val = alloca i32
	store i32 0, i32* %val
	br label %check

check:
	%idx = load i32, i32* %val
	%est_valide = call i1 @verifie_unite(i8* %mot, i32 %idx)
	br i1 %est_valide, label %return, label %next

next:
	%new_idx = add i32 %idx, 1
	store i32 %new_idx, i32* %val
	%cond = icmp slt i32 %new_idx, 20   ; Replace N with your max expected value
	br i1 %cond, label %check, label %default

return:
	%result = load i32, i32* %val
	%result_ext = zext i32 %result to i64
	ret i64 %result_ext

default:
	ret i64 0 
}

define i64 @mot_comme_dizaine(i8* %mot) {
entry:
	%val = alloca i32
	store i32 0, i32* %val
	br label %check
check:
	%idx = load i32, i32* %val
	%est_valide = call i1 @verifie_dizaine(i8* %mot, i32 %idx)
	br i1 %est_valide, label %return, label %next

next:
	%new_idx = add i32 %idx, 1
	store i32 %new_idx, i32* %val
	%cond = icmp slt i32 %new_idx, 10
	br i1 %cond, label %check, label %default

return:
	%result = load i32, i32* %val
	%result_ext = zext i32 %result to i64
	%est-cent = icmp eq i64 %result_ext, 0
	br i1 %est-cent, label %cent, label %pas-cent
cent:
	ret i64 100
pas-cent:
	%nombre_dizaine = mul i64 %result_ext, 10
	ret i64 %nombre_dizaine
default:
	ret i64 0
}

define i64 @mot_comme_separateur(i8* %mot) {
entry:
	%val = alloca i32
	store i32 0, i32* %val
	br label %check
check:
	%idx = load i32, i32* %val
	%est_valide = call i1 @verifie_separateur(i8* %mot, i32 %idx)
	br i1 %est_valide, label %return, label %next
next:
	%new_idx = add i32 %idx, 1
	store i32 %new_idx, i32* %val
	%cond = icmp slt i32 %new_idx, 8
	br i1 %cond, label %check, label %default
return:
	%result = load i32, i32* %val
	%puissance = call i64 @mille_puissance(i32 %result)
	ret i64 %puissance
default:
	ret i64 0
}

define i64 @mot_comme_entier(i8* %str) {
entry:
	%vingts = getelementptr [7 x i8], [7 x i8]* @vingts, i32 0, i32 0
	%est-vingts = call i1 @compare_texte(i8* %str, i8* %vingts, i1 1)
	br i1 %est-vingts, label %quatre-vingts, label %normal
quatre-vingts:
	ret i64 76
normal:
	%unite = call i64 @mot_comme_unite(i8* %str)
	%dizaine = call i64 @mot_comme_dizaine(i8* %str)
	%separateur = call i64 @mot_comme_separateur(i8* %str)
	%unite-dizaine = add i64 %unite, %dizaine
	%total = add i64 %unite-dizaine, %separateur
	%est-zero = call i1 @verifie_unite(i8* %str, i32 0)
	%pas-zero = xor i1 %est-zero, true
	%total-zero = icmp eq i64 %total, 0
	%probleme = and i1 %pas-zero, %total-zero
	br i1 %probleme, label %erreur, label %bon
erreur:
	ret i64 -1
bon:
	ret i64 %total
}

; External function declarations
declare i8 @tolower(i8)
declare i1 @isalpha(i8)

define i64 @texte_comme_entier(i8* %str) {
entry:
  %strlen = call i32 @strlen(i8* %str)
  %sum = alloca i64, align 8
  %total = alloca i64, align 8
  %i = alloca i32, align 4
  %token_start = alloca i8*, align 8
  %token_buf = alloca [50 x i8], align 1  ; Buffer for current token
  %token_len = alloca i32, align 4

  ; Initialize variables
  store i64 0, ptr %sum
  store i64 0, ptr %total
  store i32 0, ptr %i
  store i8* %str, ptr %token_start
  store i32 0, ptr %token_len

  br label %loop_start

loop_start:
  %current_i = load i32, ptr %i
  %cmp = icmp slt i32 %current_i, %strlen
  br i1 %cmp, label %loop_body, label %process_last_token

loop_body:
  ; Get current character
  %idx_ptr = getelementptr i8, i8* %str, i32 %current_i
  %current_char = load i8, i8* %idx_ptr

  ; Check if this is a hyphen (delimiter)
  %is_delim = icmp eq i8 %current_char, 45  ; 45 is ASCII for '-'
  br i1 %is_delim, label %process_token, label %continue_token

continue_token:
  ; Add character to token buffer
  %token_len_val = load i32, ptr %token_len
  %buf_idx = getelementptr [50 x i8], [50 x i8]* %token_buf, i32 0, i32 %token_len_val
  store i8 %current_char, i8* %buf_idx
  
  ; Increment token length
  %token_len_inc = add i32 %token_len_val, 1
  store i32 %token_len_inc, ptr %token_len
  
  ; Move to next character
  br label %next_char

process_token:
  ; Only process if we have a token
  %token_len_check = load i32, ptr %token_len
  %has_token = icmp sgt i32 %token_len_check, 0
  br i1 %has_token, label %do_process_token, label %reset_token

do_process_token:
  ; Null-terminate the token
  %term_idx = getelementptr [50 x i8], [50 x i8]* %token_buf, i32 0, i32 %token_len_check
  store i8 0, i8* %term_idx
  
  ; Process the token
  %token_ptr = getelementptr [50 x i8], [50 x i8]* %token_buf, i32 0, i32 0
  %token_value = call i64 @mot_comme_entier(i8* %token_ptr)
  %token_erreur = icmp eq i64 %token_value, -1
  br i1 %token_erreur, label %erreur, label %update_sum
update_sum:
  ; Update sum or total based on token value
  %current_sum = load i64, ptr %sum
  %is_separator = icmp sge i64 %token_value, 100
  br i1 %is_separator, label %handle_separator, label %handle_unit

handle_separator:
  %cur_total = load i64, ptr %total
  %is_sep_alone = icmp eq i64 %current_sum, 0
  br i1 %is_sep_alone, label %sep_alone, label %sep_with_value

sep_alone:
  %sep_val = mul i64 1, %token_value
  %new_total_sep = add i64 %cur_total, %sep_val
  store i64 %new_total_sep, ptr %total
  br label %reset_token

sep_with_value:
  %combined = mul i64 %current_sum, %token_value
  %new_total_comb = add i64 %cur_total, %combined
  store i64 %new_total_comb, ptr %total
  store i64 0, ptr %sum
  br label %reset_token

handle_unit:
  %new_sum = add i64 %current_sum, %token_value
  store i64 %new_sum, ptr %sum
  br label %reset_token

reset_token:
  ; Reset token buffer
  store i32 0, ptr %token_len
  br label %next_char

next_char:
  ; Increment index
  %i_val = load i32, ptr %i
  %i_inc = add i32 %i_val, 1
  store i32 %i_inc, ptr %i
  br label %loop_start

process_last_token:
  ; Check if we have a final token to process
  %final_token_len = load i32, ptr %token_len
  %has_final_token = icmp sgt i32 %final_token_len, 0
  br i1 %has_final_token, label %process_final, label %finish

process_final:
  ; Null-terminate the token
  %final_term_idx = getelementptr [50 x i8], [50 x i8]* %token_buf, i32 0, i32 %final_token_len
  store i8 0, i8* %final_term_idx
  
  ; Process the token
  %final_token_ptr = getelementptr [50 x i8], [50 x i8]* %token_buf, i32 0, i32 0
  %final_token_value = call i64 @mot_comme_entier(i8* %final_token_ptr)
  %final_erreur = icmp eq i64 %final_token_value, -1
  br i1 %final_erreur, label %erreur, label %update_final_sum
erreur:
  ret i64 -1
update_final_sum:
  ; Update sum or total based on token value
  %final_sum = load i64, ptr %sum
  %final_is_separator = icmp sge i64 %final_token_value, 100
  br i1 %final_is_separator, label %final_handle_separator, label %final_handle_unit

final_handle_separator:
  %final_cur_total = load i64, ptr %total
  %final_is_sep_alone = icmp eq i64 %final_sum, 0
  br i1 %final_is_sep_alone, label %final_sep_alone, label %final_sep_with_value

final_sep_alone:
  %final_sep_val = mul i64 1, %final_token_value
  %final_new_total_sep = add i64 %final_cur_total, %final_sep_val
  store i64 %final_new_total_sep, ptr %total
  br label %finish

final_sep_with_value:
  %final_combined = mul i64 %final_sum, %final_token_value
  %final_new_total_comb = add i64 %final_cur_total, %final_combined
  store i64 %final_new_total_comb, ptr %total
  store i64 0, ptr %sum
  br label %finish

final_handle_unit:
  %final_new_sum = add i64 %final_sum, %final_token_value
  store i64 %final_new_sum, ptr %sum
  br label %finish

finish:
  ; Add any remaining sum to the total
  %remaining_sum = load i64, ptr %sum
  %final_total = load i64, ptr %total
  %result = add i64 %final_total, %remaining_sum
  ret i64 %result
}

@type_entier = private unnamed_addr constant [7 x i8] c"entier\00"
@entier_invalide = private unnamed_addr constant [38 x i8] c"Erreur : L'entier '%s' est invalide.\0A\00"

define i64 @demande_entier(i8* %nom_variable) {
entry:
	%demande_fmt = getelementptr [30 x i8], [30 x i8]* @demande_str, i32 0, i32 0
	%type_entier_str = getelementptr [7 x i8], [7 x i8]* @type_entier, i32 0, i32 0
	call i32 (i8*, ...) @printf(i8* %demande_fmt, i8* %nom_variable, i8* %type_entier_str)
	%reponse = call i8* @read_line()
	%is_null = icmp eq ptr %reponse, null
	br i1 %is_null, label %vide, label %texte
vide:
	%vide_str = getelementptr [1 x i8], [1 x i8]* @vide, i32 0, i32 0
	br label %redemande
texte:
	%resultat = call i64 @texte_comme_entier(i8* %reponse)
	%pas-bon = icmp eq i64 %resultat, -1
	br i1 %pas-bon, label %redemande, label %bon-nombre
bon-nombre:
	ret i64 %resultat
redemande:
	%erreur = phi i8* [%vide_str, %vide], [%reponse, %texte]
	%erreur_fmt = getelementptr [38 x i8], [38 x i8]* @entier_invalide, i32 0, i32 0
	call i32 (i8*, ...) @printf(i8* %erreur_fmt, i8* %erreur)
	%nouvelle-reponse = call i64 @demande_entier(i8* %nom_variable)
	ret i64 %nouvelle-reponse
}

