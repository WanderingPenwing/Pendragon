# Pendragon

Un language de programmation avec la syntaxe d'un texte français.

Avec de la [Documentation](https://doc.penwing.org) !

## Pour l'instant
- La partie pendragon/ transforme le fichier '.dr' en AST (arbre de syntaxe)
- La parte sophie/ interprète l'AST
- La partie morgan/ convertit l'AST en IR de LLVM
- Compilation avec LLVM

## Exécution

Dans le codespace github, utilisez Ctrl+Shift+B, ou la commande ```pendragon <fichier>```

## Exemple
Voici un exemple de code :

```
Définis A comme entier. Définis B comme entier.
Modifie B avec un.

Définis N comme entier.
Modifie N avec trente.

Tant que N est supérieur à zéro,
	Modifie N avec N moins un.
	
	Affiche A.
	Affiche B.
	Modifie A avec A plus B.
	Modifie B avec A plus B.

Affiche "Fin".

Nota Bene : Ceci est un programme qui affiche deux fois N nombres de la suite de Fibonacci. 
```
