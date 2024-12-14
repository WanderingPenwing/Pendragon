use std::collections::HashMap;
use crate::sophie;
use crate::pendragon;
use crate::display::*;
use crate::structure::*;

#[test]
fn calcul_texte() {
	let pendragon = pendragon::Pendragon::nouveau();
	let a = 2345678;
	let b = 987654;
	
	let possible_expression = pendragon.elements_texte(&format!("\"hello\" puis {} fois {} puis \"there\" puis vrai ou faux puis trois puis deux puis alinéa puis retour à la ligne",
		sophie::nombre::nombre_comme_texte(a),
		sophie::nombre::nombre_comme_texte(b)));
	match possible_expression {
		Ok(expression) => {
			match sophie::texte::calcule_texte(expression, &HashMap::new()) {
				Ok(texte) => {
					let vrai_texte = format!("hello{}therevraitroisdeux\t\n", sophie::nombre::nombre_comme_texte(a*b));
					assert_eq!(texte, vrai_texte, "Calcul d'expression (texte) donne un mauvais résultat : {}", texte);
				}
				Err(raison) => {
					panic!("Calcul d'expression (texte) échoué, avec l'erreur : {}", raison);
				}
			}
		}
		Err(raison) => {
			panic!("Détermination d'expression (texte) échouée : {}", raison);
		}
	}
}

#[test]
fn conversion_texte() {
	let pendragon = pendragon::Pendragon::nouveau();
	let texte = "\"hello	 aaaa puis AERTY et ou fois six\"";
	match pendragon.elements_texte(texte) {
		Ok(expression) => {
			if expression.len() != 2 {
				panic!("L'expression (texte) devrait contenir deux éléments (texte et puis), contient : {:?}", expression);
			}
			assert_eq!(expression[0], Element::Texte(texte[1..texte.len()-1].into()), "Calcul d'expression (texte) donne un mauvais résultat : {}", texte);
		}
		Err(raison) => {
			panic!("Conversion échouée (texte) : {}", raison);
		}
	}
}

#[test]
fn erreur_conversion_texte() {
	let pendragon = pendragon::Pendragon::nouveau();
	let textes = vec![
		"trois puis puis un",
		"\" test",
		"puis",
		"un puis",
		"puis un",
	];
	for texte in textes {
		let Err(raison) = pendragon.elements_texte(texte) else {
			panic!("Ne devrait pas réussir à convertir le texte '{}'", texte);
		};
		let ErreurPendragon::TexteInvalide(_) = raison else {
			panic!("Erreur imprévue pour convertir le texte '{}' : {}", texte, raison);
		};
	}
}


//----------------------------


#[test]
fn conversion_booleen_texte() {
	for b in [true, false].iter() {
		let texte = sophie::booleen::booleen_comme_texte(*b); // Convert number to text
		match pendragon::booleen::texte_comme_booleen(&texte) { // Convert text back to number
			Ok(booleen) => {
				assert_eq!(Element::Booleen(*b), booleen, "Booleen inexact : {}, texte : {}", b, texte);
			}
			Err(raison) => {
				panic!("Conversion échouée pour : {}, avec l'erreur : {}", b, raison);
			}
		}
	}
}

#[test]
fn calcul_booleen() {
	let pendragon = pendragon::Pendragon::nouveau();
	let mut configurations = Vec::new();
	for b1 in [true, false] {
		for b2 in [true, false] {
			for b3 in [true, false] {
				for b4 in [true, false] {
					for b5 in [true, false] {
						configurations.push((b1, b2, b3, b4, b5));
					}
				}
			}
		}
	}
	for configuration in configurations {
		let possible_expression = pendragon.elements_booleen(&format!("{} et non ouvre la parenthèse {} ou non {} ferme la parenthèse ou non {} et {}",
			sophie::booleen::booleen_comme_texte(configuration.0),
			sophie::booleen::booleen_comme_texte(configuration.1),
			sophie::booleen::booleen_comme_texte(configuration.2),
			sophie::booleen::booleen_comme_texte(configuration.3),
			sophie::booleen::booleen_comme_texte(configuration.4)));
		match possible_expression {
			Ok(expression) => {
				match sophie::booleen::calcule_booleen(expression, &HashMap::new()) {
					Ok(booleen) => {
						let resultat = configuration.0 && !(configuration.1 || !configuration.2) || !configuration.3 && configuration.4;
						assert_eq!(booleen, resultat, "Calcul d'expression (booleen) donne un mauvais résultat : {}", booleen);
					}
					Err(raison) => {
						panic!("Calcul d'expression (booleen) échoué, avec l'erreur : {}", raison);
					}
				}
			}
			Err(raison) => {
				panic!("Détermination d'expression (booleen) échouée : {}", raison);
			}
		}
	}
}

#[test]
fn comparaison_booleen() {
	let pendragon = pendragon::Pendragon::nouveau();
	for (a, b) in [(1, 4), (2, 2), (3, 1), (0, 3)] {
		let possible_expressions = vec![
			pendragon.elements_booleen(&format!("non six plus {} est supérieur à deux fois {}", sophie::nombre::nombre_comme_texte(a), sophie::nombre::nombre_comme_texte(b))),
			pendragon.elements_booleen(&format!("six plus {} est inférieur à deux fois {}", sophie::nombre::nombre_comme_texte(a), sophie::nombre::nombre_comme_texte(b))),
			pendragon.elements_booleen(&format!("six plus {} est supérieur ou égal à deux fois {}", sophie::nombre::nombre_comme_texte(a), sophie::nombre::nombre_comme_texte(b))),
			pendragon.elements_booleen(&format!("non six plus {} est inférieur ou égal à deux fois {}", sophie::nombre::nombre_comme_texte(a), sophie::nombre::nombre_comme_texte(b))),
			pendragon.elements_booleen(&format!("non \"deux\" est égal à \"{}\"", sophie::nombre::nombre_comme_texte(a))),
			pendragon.elements_booleen(&format!("\"trois\" est différent de \"{}\"", sophie::nombre::nombre_comme_texte(a))),
		];
		let bonne_reponses = vec![
			!(6+a > 2*b),
			(6+a < 2*b),
			(6+a >= 2*b),
			!(6+a <= 2*b),
			!(a == 2),
			(a != 3),
		];
		for index in 0..possible_expressions.len() {
			match &possible_expressions[index] {
				Ok(expression) => {
					match sophie::booleen::calcule_booleen(expression.clone(), &HashMap::new()) {
						Ok(booleen) => {
							let reponse = bonne_reponses[index];
							assert_eq!(booleen, reponse, "Calcul d'expression (booleen) n°{} ({},{}) donne un mauvais résultat : {}, attendais {}", index, a, b, booleen, reponse);
						}
						Err(raison) => {
							panic!("Calcul d'expression (booleen) échoué, avec l'erreur : {}", raison);
						}
					}
				}
				Err(raison) => {
					panic!("Détermination d'expression (booleen) échouée : {}", raison);
				}
			}
		}
	}
}

#[test]
fn combinaison_booleen() {
	let pendragon = pendragon::Pendragon::nouveau();
	for a in 0..5 {
		for b in 0..5 {
			for c in 0..5 {
				for d in 1..5 {
					for e in 0..5 {
						let possible_expression = pendragon.elements_booleen(&format!("non ouvre la parenthèse six plus {} ferme la parenthèse est supérieur à deux fois {} et ouvre la parenthèse {} divisé par deux est inférieur à ouvre la parenthèse {} moins un ferme la parenthèse ou non \"deux\" est égal à \"{}\" ferme la parenthèse", 
								sophie::nombre::nombre_comme_texte(a),
								sophie::nombre::nombre_comme_texte(b),
								sophie::nombre::nombre_comme_texte(c),
								sophie::nombre::nombre_comme_texte(d),
								sophie::nombre::nombre_comme_texte(e),
						));
						let bonne_reponse = !((6+a) > 2*b) && (c/2 < (d-1) || !(e == 2));
						match possible_expression {
							Ok(expression) => {
								match sophie::booleen::calcule_booleen(expression.clone(), &HashMap::new()) {
									Ok(booleen) => {
										assert_eq!(booleen, bonne_reponse, "Calcul d'expression (booleen) ({},{},{},{},{}) donne un mauvais résultat : {}, attendais {}", a, b, c, d, e, booleen, bonne_reponse);
									}
									Err(raison) => {
										panic!("Calcul d'expression (booleen) échoué, avec l'erreur : {}", raison);
									}
								}
							}
							Err(raison) => {
								panic!("Détermination d'expression (booleen) échouée : {}", raison);
							}
						}
					}
				}
			}
		}
	}
}

#[test]
fn erreur_calcul_booleen() {
	let pendragon = pendragon::Pendragon::nouveau();
	let textes_invalide = vec![
		"et faux",
		"vrai et et faux",
		"vrai ou ou faux",
		"vrai et vrai faux",
		"vrai et faux vrai",
		"vrai et faux ouvre la parenthèse vrai ou faux ferme la parenthèse",
		"vrai et ouvre la parenthèse et vrai ou faux ferme la parenthèse",
		"vrai et ouvre la parenthèse vrai ou faux et ferme la parenthèse",
		"vrai et ouvre la parenthèse vrai ou faux ferme la parenthèse vrai",
	];
	for texte in textes_invalide {
		let Err(raison) = pendragon.elements_booleen(texte) else {
			panic!("Devrait détecter une erreur pour '{}'", texte);
		};
		let ErreurPendragon::OrdreCalculBooleen(_,_,_) = raison else {
			panic!("Devrait détecter une erreur de calcul booléen pour '{}', a déclenché : {}", texte, raison);
		};
	}
}


// ---------------------


#[test]
fn conversion_nombres_texte() {
	for i in [0, 1, 42, 70, 123, 999, 1031, 1_001_091, 72_036_854_775_807usize, 2345678*987654].iter() {
		let texte = sophie::nombre::nombre_comme_texte(*i); // Convert number to text
		if texte.contains("--") {
			panic!("Il y a deux tirets pour {} : {}", i, texte);
		}
		match pendragon::nombre::texte_comme_nombre(&texte) { // Convert text back to number
			Ok(nombre) => {
				assert_eq!(Element::Entier(*i), nombre, "Nombre inexact : {}, texte : {}", i, texte);
			}
			Err(raison) => {
				panic!("Conversion échouée pour : {}, avec l'erreur : {}", i, raison);
			}
		}
	}
}

#[test]
fn calcul_nombre() {
	let pendragon = pendragon::Pendragon::nouveau();
	let a = 2345678;
	let b = 987654;
	let c = 34523456;
	let d = 45678;
	let e = 2;
	let possible_expression = pendragon.elements_nombre(&format!("{} fois {} plus ouvre la parenthèse {} moins {} ferme la parenthèse divisé par {}",
		sophie::nombre::nombre_comme_texte(a),
		sophie::nombre::nombre_comme_texte(b),
		sophie::nombre::nombre_comme_texte(c),
		sophie::nombre::nombre_comme_texte(d),
		sophie::nombre::nombre_comme_texte(e)));
	match possible_expression {
		Ok(expression) => {
			match sophie::nombre::calcule_nombre(expression, &HashMap::new()) {
				Ok(nombre) => {
					assert_eq!(nombre, a*b+(c-d)/e, "Calcul d'expression (entier) donne un mauvais résultat : {}", nombre);
				}
				Err(raison) => {
					panic!("Calcul d'expression (entier) échoué, avec l'erreur : {}", raison);
				}
			}
		}
		Err(raison) => {
			panic!("Détermination d'expression (entier) échouée : {}", raison);
		}
	}
}

#[test]
fn erreur_calcul_nombre() {
	let pendragon = pendragon::Pendragon::nouveau();
	let textes_invalide = vec![
		"un un fois un",
		"un plus fois un",
		"un moins divisé par un",
		"un fois un ouvre la parenthèse un plus un ferme la parenthèse",
		"un fois ouvre la parenthèse plus un plus un ferme la parenthèse",
		"un fois ouvre la parenthèse un plus un fois ferme la parenthèse",
		"un fois ouvre la parenthèse un plus un ferme la parenthèse un",
	];
	for texte in textes_invalide {
		let Err(raison) = pendragon.elements_nombre(texte) else {
			panic!("Devrait détecter une erreur pour '{}'", texte);
		};
		let ErreurPendragon::OrdreCalculEntier(_,_,_) = raison else {
			panic!("Devrait détecter une erreur de calcul entier pour '{}', a déclenché : {}", texte, raison);
		};
	}
}

#[test]
fn nombre_invalide_et() {
	let pendragon = pendragon::Pendragon::nouveau();
	let Err(raison) = pendragon.elements_nombre("et") else {
		panic!("Devrait détecter une erreur pour 'et'");
	};
	let ErreurPendragon::NombreInvalide(_) = raison else {
		panic!("Devrait détecter une erreur de nombre invalide pour 'et', a déclenché : {}", raison);
	};
}