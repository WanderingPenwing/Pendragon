use super::*;

impl Pendragon {
	pub fn elements_booleen(&self, arguments: &str) -> Result<Vec<Element>, ErreurPendragon> {
		let texte = arguments
					.replace("ouvre la parenthèse", "ouvre-la-parenthese")
					.replace("ferme la parenthèse", "ferme-la-parenthese")
					.replace("est égal à", "est-egal-a")
					.replace("est différent de", "est-different-de")
					.replace("est supérieur ou égal à", "est-superieur-ou-egal-a")
					.replace("est inférieur ou égal à", "est-inferieur-ou-egal-a")
					.replace("est supérieur à", "est-superieur-a")
					.replace("est inférieur à", "est-inferieur-a")
					.replace("divisé par", "divise-par");
		let elements_texte: Vec<&str> = texte.split(" ").collect();
		let mut expression: Vec<Element> = Vec::new();
		let mut pile_operateurs: Vec<Operateur> = Vec::new();
		
		let mut pile_inconnu: Vec<String> = Vec::new();
		let mut possible_comparaison: Option<Comparaison> = None;
		let mut precede_par_operation: bool = true;
	
		for element in elements_texte {
			match element {
				"vrai" => {
					self.fin_comparaison("vrai", &mut pile_inconnu, &mut pile_operateurs, &mut expression, &mut possible_comparaison)?;
					expression.push(Element::Booleen(true));
					if !precede_par_operation {
						return Err(ErreurPendragon::CalculBooleen("il manque un opérateur avant le booléen 'vrai'".into()))
					}
					precede_par_operation = false;
					continue;
				},
				"faux" => {
					self.fin_comparaison("faux", &mut pile_inconnu, &mut pile_operateurs, &mut expression, &mut possible_comparaison)?;
					expression.push(Element::Booleen(false));
					if !precede_par_operation {
						return Err(ErreurPendragon::CalculBooleen("il manque un opérateur avant le booléen 'faux'".into()))
					}
					precede_par_operation = false;
					continue;
				}
				"non" => {
					self.fin_comparaison("non", &mut pile_inconnu, &mut pile_operateurs, &mut expression, &mut possible_comparaison)?;
					pile_operateurs.push(Operateur::Non);
					if !precede_par_operation {
						return Err(ErreurPendragon::CalculBooleen("il manque un opérateur avant l'opérateur 'non'".into()))
					}
					continue;
				}
				"et" => {
					self.fin_comparaison("et", &mut pile_inconnu, &mut pile_operateurs, &mut expression, &mut possible_comparaison)?;
					while let Some(operateur) = pile_operateurs.last() {
						if *operateur == Operateur::Non || *operateur == Operateur::Et {
							expression.push(Element::Operateur(pile_operateurs.pop().unwrap()));
						} else {
							break;
						}
					}
					pile_operateurs.push(Operateur::Et);
				}
				"ou" => {
					self.fin_comparaison("ou", &mut pile_inconnu, &mut pile_operateurs, &mut expression, &mut possible_comparaison)?;
					while let Some(operateur) = pile_operateurs.last() {
						if *operateur == Operateur::Non || *operateur == Operateur::Et || *operateur == Operateur::Ou {
							expression.push(Element::Operateur(pile_operateurs.pop().unwrap()));
						} else {
							break;
						}
					}
					pile_operateurs.push(Operateur::Ou);
				}
				"ouvre-la-parenthese" => {
					if !precede_par_operation && pile_inconnu.len() > 0 {
						return Err(ErreurPendragon::CalculBooleen("il manque un opérateur avant l'ouverture de la parenthèse".into()))
					}
					pile_inconnu.push("ouvre-la-parenthese".into());
					continue;
				}
				"ferme-la-parenthese" => {
					if precede_par_operation {
						return Err(ErreurPendragon::CalculBooleen("il manque un booleen avant la fermeture de la parenthèse".into()))
					}
					let nombre_parenthese = compare_parentheses(&pile_inconnu);
					if nombre_parenthese.0 > nombre_parenthese.1 {
						pile_inconnu.push("ferme-la-parenthese".into());
						continue
					}
					self.fin_comparaison("ferme-la-parenthese", &mut pile_inconnu, &mut pile_operateurs, &mut expression, &mut possible_comparaison)?;
					while let Some(operateur) = pile_operateurs.pop() {
						if operateur == Operateur::ParentheseBooleen {
							break;
						}
						expression.push(Element::Operateur(operateur));
					}
					continue;
				}
				autre => {
					if format_de_variable(autre) {
						if let Ok(_) = self.programme.variable_est_de_type(autre, TypeElement::Booleen) {
							self.fin_comparaison(autre, &mut pile_inconnu, &mut pile_operateurs, &mut expression, &mut possible_comparaison)?;
							expression.push(Element::Variable(autre.into(), TypeElement::Booleen));
						} else {
							pile_inconnu.push(autre.into());
						}
					} else if let Ok(type_comparaison) = texte_comme_comparaison(autre) {
						if let Some(comparaison) = possible_comparaison {
							return Err(ErreurPendragon::BooleenInvalide(format!("besoin d'un operateur booleen entre {:?} et {:?}", comparaison, type_comparaison)))
						}
						let mut comparaison = Comparaison::nouvelle();
						let nombre_parenthese = compare_parentheses(&pile_inconnu); 
						if pile_inconnu[0] == "ouvre-la-parenthese" && nombre_parenthese.0 > nombre_parenthese.1 {
							pile_inconnu.remove(0);
							pile_operateurs.push(Operateur::ParentheseBooleen);
						}
						self.ajoute_comparaison_membre(&mut comparaison, &pile_inconnu.join(" "))?;
						comparaison.ajoute_type(type_comparaison)?;
						possible_comparaison = Some(comparaison);
						pile_inconnu = Vec::new();
					} else {
						pile_inconnu.push(autre.into());
					}
//					if !precede_par_operation {
//						return Err(ErreurPendragon::CalculBooleen(format!("il manque un opérateur avant le booléen '{}'", autre)))
//					}
					precede_par_operation = false;
					continue;
				}
			}
			if precede_par_operation {
				return Err(ErreurPendragon::CalculBooleen(format!("il manque un booleen avant l'opérateur '{}'", element)))
			}
			precede_par_operation = true;
		}
		if !pile_inconnu.is_empty() {
			let Some(mut comparaison) = possible_comparaison else {
				return Err(ErreurPendragon::BooleenInvalide(format!("{:?}", pile_inconnu)))
			};
			self.ajoute_comparaison_membre(&mut comparaison, &pile_inconnu.join(" "))?;
			expression.push(Element::Comparaison(comparaison.clone()));
		}
		
		while let Some(operateur) = pile_operateurs.pop() {
			expression.push(Element::Operateur(operateur));
		}
		
		Ok(expression)
	}
	
	pub fn fin_comparaison(&self, _element: &str, pile_inconnu: &mut Vec<String>, pile_operateurs: &mut Vec<Operateur>, expression: &mut Vec<Element>, possible_comparaison: &mut Option<Comparaison>) -> Result<(), ErreurPendragon> {
		if pile_inconnu.len() == 1 && pile_inconnu[0] == "ouvre-la-parenthese" {
			pile_operateurs.push(Operateur::ParentheseBooleen);
			*pile_inconnu = Vec::new();
		}
		if pile_inconnu.is_empty() {
			return Ok(());
		}
		let Some(ancienne_comparaison) = possible_comparaison else {
			return Err(ErreurPendragon::BooleenInvalide(format!("{:?}", pile_inconnu)))
		};
		let mut comparaison = ancienne_comparaison.clone();
		self.ajoute_comparaison_membre(&mut comparaison, &pile_inconnu.join(" "))?;
		expression.push(Element::Comparaison(comparaison.clone()));
		*pile_inconnu = Vec::new();
		*possible_comparaison = None;
		Ok(())
	}
	
	pub fn ajoute_comparaison_membre(&self, comparaison: &mut Comparaison, texte: &str) -> Result<(), ErreurPendragon> {
		let membre = if let Ok(elements_nombre) = self.elements_nombre(texte) {
			elements_nombre
		} else if let Ok(elements_booleen) = self.elements_booleen(texte) {
			elements_booleen
		} else if let Ok(elements_texte) = self.elements_texte(texte) {
			elements_texte
		} else {
			return Err(ErreurPendragon::MauvaisArgument(texte.to_string()));
		};
		let Some(element) = membre.first() else {
			return Err(ErreurPendragon::ComparaisonInvalide("il n'y a pas de d'élément dans le membre ajouté".into()))
		};
		if comparaison.type_comparaison.is_none() {
			comparaison.membre_a = membre;
			return Ok(());
		}
		let Some(element_de_comparaison) = comparaison.membre_a.first() else {
			return Err(ErreurPendragon::ComparaisonInvalide("il n'y a pas de premier membre".into()))
		};
		if element_de_comparaison.type_element() != element.type_element() {
			return Err(ErreurPendragon::MauvaisType(
				format!("{:?}", element), element.type_element().nom(), 
				element_de_comparaison.type_element().nom()))
		}
		comparaison.membre_b = membre;
		Ok(())
	}
}

fn compare_parentheses(strings: &Vec<String>) -> (usize, usize) {
	let ouvre_count = strings.iter().filter(|s| *s == "ouvre-la-parenthese").count();
	let ferme_count = strings.iter().filter(|s| *s == "ferme-la-parenthese").count();

	(ouvre_count, ferme_count)
}

pub fn affiche_booleen(expression: Vec<Element>, variables: &HashMap<String, Element>) -> Result<String, ErreurPendragon> {
	let booleen = calcule_booleen(expression.clone(), variables)?;
	Ok(booleen_comme_texte(booleen))
}

pub fn calcule_booleen(expression: Vec<Element>, variables: &HashMap<String, Element>) -> Result<bool, ErreurPendragon> {
	let mut pile: Vec<bool> = Vec::new();
	
	for element in expression {
		if let Element::Booleen(booleen) = element {
			pile.push(booleen);
			continue;
		}
		if let Element::Variable(nom, _) = element {
			let Some(variable) = variables.get(&nom) else {
				return Err(ErreurPendragon::VariableInconnue(nom.into()))
			};
			if let Element::Booleen(booleen) = variable {
				pile.push(*booleen);
				continue
			} else {
				return Err(ErreurPendragon::MauvaisType(nom.into(), variable.type_element().nom(), "booleen".into()))
			}
		}
		if let Element::Comparaison(comparaison) = element {
			pile.push(comparaison.calcule(variables)?);
			continue
		}
		let Element::Operateur(ref operateur) = element else {
			return Err(ErreurPendragon::MauvaisArgument(format!("{:?}, attendais un opérateur", element)))
		};
		let Some(booleen_a) = pile.pop() else {
			return Err(ErreurPendragon::CalculBooleen("la pile est vide".into()))
		};
		match operateur {
			Operateur::Non => {
				pile.push(!booleen_a);
			}
			Operateur::Et => {
				let Some(booleen_b) = pile.pop() else {
					return Err(ErreurPendragon::CalculBooleen("la pile est vide".into()))
				};
				pile.push(booleen_a && booleen_b);
			}
			Operateur::Ou => {
				let Some(booleen_b) = pile.pop() else {
					return Err(ErreurPendragon::CalculBooleen("la pile est vide".into()))
				};
				pile.push(booleen_a || booleen_b);
			}
			_ => return Err(ErreurPendragon::MauvaisArgument(format!("{:?}, attendais un opérateur booléen", element)))
		}
	}
	if pile.len() > 1 {
		return Err(ErreurPendragon::CalculBooleen("il reste plusieurs éléments dans la pile".into()))
	}
	Ok(pile[0])
}

pub fn booleen_comme_texte(booleen: bool) -> String {
	if booleen {
		"vrai".into()
	} else {
		"faux".into()
	}
}

pub fn texte_comme_booleen(texte: &str) -> Result<Element, ErreurPendragon> {
	match texte {
		"vrai" => Ok(Element::Booleen(true)),
		"faux" => Ok(Element::Booleen(false)),
		_ => Err(ErreurPendragon::BooleenInvalide(texte.into())),
	}
}

pub fn texte_comme_comparaison(texte: &str) -> Result<TypeComparaison, ErreurPendragon> {
	match texte {
		"est-egal-a" => Ok(TypeComparaison::Egal),
		"est-different-de" => Ok(TypeComparaison::Different),
		"est-superieur-ou-egal-a" => Ok(TypeComparaison::SuperieurEgal),
		"est-inferieur-ou-egal-a" => Ok(TypeComparaison::InferieurEgal),
		"est-superieur-a" => Ok(TypeComparaison::Superieur),
		"est-inferieur-a" => Ok(TypeComparaison::Inferieur),
		_ => Err(ErreurPendragon::ComparaisonInvalide(format!("\"{}\" n'est pas un type de comparaison", texte))),
	}
}




// -----------------------------------------------------------------------


#[cfg(test)]
mod test {
	use std::collections::HashMap;
	use super::*;
	
	#[test]
	fn conversion_booleen_texte() {
		for b in [true, false].iter() {
			let texte = booleen_comme_texte(*b); // Convert number to text
			match texte_comme_booleen(&texte) { // Convert text back to number
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
		let pendragon = Pendragon::nouveau();
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
				booleen_comme_texte(configuration.0),
				booleen_comme_texte(configuration.1),
				booleen_comme_texte(configuration.2),
				booleen_comme_texte(configuration.3),
				booleen_comme_texte(configuration.4)));
			match possible_expression {
				Ok(expression) => {
					match calcule_booleen(expression, &HashMap::new()) {
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
		let pendragon = Pendragon::nouveau();
		for (a, b) in [(1, 4), (2, 2), (3, 1), (0, 3)] {
			let possible_expressions = vec![
				pendragon.elements_booleen(&format!("non six plus {} est supérieur à deux fois {}", nombre::nombre_comme_texte(a), nombre::nombre_comme_texte(b))),
				pendragon.elements_booleen(&format!("six plus {} est inférieur à deux fois {}", nombre::nombre_comme_texte(a), nombre::nombre_comme_texte(b))),
				pendragon.elements_booleen(&format!("six plus {} est supérieur ou égal à deux fois {}", nombre::nombre_comme_texte(a), nombre::nombre_comme_texte(b))),
				pendragon.elements_booleen(&format!("non six plus {} est inférieur ou égal à deux fois {}", nombre::nombre_comme_texte(a), nombre::nombre_comme_texte(b))),
				pendragon.elements_booleen(&format!("non \"deux\" est égal à \"{}\"", nombre::nombre_comme_texte(a))),
				pendragon.elements_booleen(&format!("\"trois\" est différent de \"{}\"", nombre::nombre_comme_texte(a))),
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
						match calcule_booleen(expression.clone(), &HashMap::new()) {
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
		let pendragon = Pendragon::nouveau();
		for a in 0..5 {
			for b in 0..5 {
				for c in 0..5 {
					for d in 1..5 {
						for e in 0..5 {
							println!();
							let possible_expression = pendragon.elements_booleen(&format!("non ouvre la parenthèse six plus {} ferme la parenthèse est supérieur à deux fois {} et ouvre la parenthèse {} divisé par deux est inférieur à ouvre la parenthèse {} moins un ferme la parenthèse ou non \"deux\" est égal à \"{}\" ferme la parenthèse", 
									nombre::nombre_comme_texte(a),
									nombre::nombre_comme_texte(b),
									nombre::nombre_comme_texte(c),
									nombre::nombre_comme_texte(d),
									nombre::nombre_comme_texte(e),
							));
							let bonne_reponse = !((6+a) > 2*b) && (c/2 < (d-1) || !(e == 2));
							match possible_expression {
								Ok(expression) => {
									match calcule_booleen(expression.clone(), &HashMap::new()) {
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
		let pendragon = Pendragon::nouveau();
		let textes_invalide = vec![
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
			let ErreurPendragon::CalculBooleen(_) = raison else {
				panic!("Devrait détecter une erreur de calcul booléen pour '{}', a déclenché : {}", texte, raison);
			};
		}
	}
}
