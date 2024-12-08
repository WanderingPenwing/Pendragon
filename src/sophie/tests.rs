use super::*;

#[test]
fn teste_conversion_nombres_texte() {
	for i in [0, 1, 42, 123, 999, 1031, 1_001_091, 72_036_854_775_807usize].iter() {
		let texte = nombres::nombre_comme_texte(*i); // Convert number to text
		match nombres::texte_comme_nombre(&texte) { // Convert text back to number
			Ok(nombre) => {
				assert_eq!(*i, nombre, "Nombre inexact : {}, texte : {}", i, texte);
			}
			Err(raison) => {
				panic!("Conversion échouée pour : {}, avec l'erreur : {}", i, raison);
			}
		}
	}
}

#[test]
fn teste_somme() {
	for (a, b) in [(0, 0), (5, 7), (1467,45678), (1001, 0), (72_036_854_775_807usize, 14_036_567_775_807usize)] {
		let texte_a = nombres::nombre_comme_texte(a);
		let texte_b = nombres::nombre_comme_texte(b);
		let sophie = Sophie::new();
		let resultat = sophie.operation(&format!("{} plus {}", texte_a, texte_b));
		match resultat { // Convert text back to number
			Ok(nombre) => {
				assert_eq!(a+b, nombre, "Résultat inexact pour {}+{} : {}", a, b, nombre);
			}
			Err(raison) => {
				panic!("Conversion échouée pour : ({},{}), avec l'erreur : {}", a, b, raison);
			}
		}
	}
}

#[test]
fn teste_definition_variable() {
	let mut sophie = Sophie::new();
	let resultat = sophie.execute_phrase("Définie Variable comme entier");
	match resultat {
		Ok(_) => {
			assert_eq!(sophie.variables["Variable"], Variable::Entier(0), "Variable mal définie");
		}
		Err(raison) => {
			panic!("Définition de variable échouée : {}", raison);
		}
	}
}

#[test]
fn teste_modification_variable() {
	let mut sophie = Sophie::new();
	if let Err(raison) = sophie.execute_phrase("Définie Variable comme entier") {
		panic!("Définition de variable échouée : {}", raison);
	}
	let a = 2345678;
	let resultat = sophie.execute_phrase(&format!("Modifie Variable avec {} ", nombres::nombre_comme_texte(a)));
	match resultat {
		Ok(_) => {
			assert_eq!(sophie.variables["Variable"], Variable::Entier(a), "Variable mal modifiée");
		}
		Err(raison) => {
			panic!("Modification de variable échouée : {}", raison);
		}
	}
}

#[test]
fn teste_operation_variable() {
	let mut sophie = Sophie::new();
	if let Err(raison) = sophie.execute_phrase("Définie Variable comme entier") {
		panic!("Définition de variable échouée : {}", raison);
	}
	let a = 2345678;
	if let Err(raison) = sophie.execute_phrase(&format!("Modifie Variable avec {} ", nombres::nombre_comme_texte(a))) {
		panic!("Modification de variable échouée : {}", raison);
	}
	let b = 987654;
	let resultat = sophie.operation(&format!("Variable plus {}", nombres::nombre_comme_texte(b)));
	match resultat {
		Ok(nombre) => {
				assert_eq!(nombre, a+b, "Echec de la somme d'un entier et d'une variable, attendais {}, a reçu {}", a+b, nombre);
		}
		Err(raison) => {
			panic!("Opération de variable échouée : {}", raison);
		}
	}
}

#[test]
fn teste_maths() {
	let sophie = Sophie::new();
	let a = 2345678;
	let b = 987654;
	let c = 34523456;
	let d = 45678;
	let e = 2;
	let resultat = sophie.operation(&format!("{} fois {} plus ouvre la parenthèse {} moins {} ferme la parenthèse divisé par {}",
		nombres::nombre_comme_texte(a),
		nombres::nombre_comme_texte(b),
		nombres::nombre_comme_texte(c),
		nombres::nombre_comme_texte(d),
		nombres::nombre_comme_texte(e)));
	match resultat {
		Ok(nombre) => {
			assert_eq!(nombre, a*b+(c-d)/e, "Echec de l'opération mathématique, résultat : {}", nombre);
		}
		Err(raison) => {
			panic!("Execution échouée pour multiplication, avec l'erreur : {}", raison);
		}
	}
}

// --------------------------------------------- anti-test

#[test]
fn teste_echec_modification() {
	let mut sophie = Sophie::new();
	let resultat = sophie.execute_phrase("Modifie Variable avec deux");
	let Err(raison) = resultat else {
		panic!("Ne devrais pas pouvoir modifier une variable non définie");
	};
	if let ErreurSophie::VariableInconnue(nom) = raison {
		assert_eq!(nom, "Variable", "Mauvais nom de variable reconnu : {}", nom);
	} else {
		panic!("Modification échouée avec erreur imprévue : {}", raison);
	}			
}
