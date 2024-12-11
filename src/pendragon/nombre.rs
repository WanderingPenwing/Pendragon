use super::*;

const NOMS_UNITES: [&str; 10] = ["", "un", "deux", "trois", "quatre", "cinq", "six", "sept", "huit", "neuf"];
const NOMS_UNITES_DIX: [&str; 10] = ["dix", "onze", "douze", "treize", "quatorze", "quinze", "seize", "dix-sept", "dix-huit", "dix-neuf"];
const NOMS_DIZAINES: [&str; 9] = ["", "dix", "vingt", "trente", "quarante", "cinquante", "soixante", "x", "quatre-vingts"];
const NOMS_SEPARATEURS: [&str; 7] = ["", "mille", "million", "milliard", "billion", "billiard", "trillion"];
const UNION: &str = "-";

impl Pendragon {
	pub fn elements_nombre(&self, arguments: &str) -> Result<Vec<Element>, ErreurPendragon> {
		let texte = arguments
					.replace("ouvre la parenthèse", "ouvre-la-parenthese")
					.replace("ferme la parenthèse", "ferme-la-parenthese")
					.replace("divisé par", "divise-par");
		let elements_texte: Vec<&str> = texte.split(" ").collect();
		let mut expression: Vec<Element> = Vec::new();
		let mut pile_operateurs: Vec<Operateur> = Vec::new();
	
		for element in elements_texte {
			match element {
				"plus" => {
					while let Some(operateur) = pile_operateurs.last() {
						if *operateur == Operateur::Plus || *operateur == Operateur::Moins || *operateur == Operateur::Fois || *operateur == Operateur::Divise {
							expression.push(Element::Operateur(pile_operateurs.pop().unwrap()));
						} else {
							break;
						}
					}
					pile_operateurs.push(Operateur::Plus);
				}
				"moins" => {
					while let Some(operateur) = pile_operateurs.last() {
						if *operateur == Operateur::Plus || *operateur == Operateur::Moins || *operateur == Operateur::Fois || *operateur == Operateur::Divise {
							expression.push(Element::Operateur(pile_operateurs.pop().unwrap()));
						} else {
							break;
						}
					}
					pile_operateurs.push(Operateur::Moins);
				}
				"fois" => {
					while let Some(operateur) = pile_operateurs.last() {
						if *operateur == Operateur::Fois || *operateur == Operateur::Divise {
							expression.push(Element::Operateur(pile_operateurs.pop().unwrap()));
						} else {
							break;
						}
					}
					pile_operateurs.push(Operateur::Fois);
				}
				"divise-par" => {
					while let Some(operateur) = pile_operateurs.last() {
						if *operateur == Operateur::Fois || *operateur == Operateur::Divise {
							expression.push(Element::Operateur(pile_operateurs.pop().unwrap()));
						} else {
							break;
						}
					}
					pile_operateurs.push(Operateur::Divise);
				}
				"ouvre-la-parenthese" => pile_operateurs.push(Operateur::ParentheseEntier),
				"ferme-la-parenthese" => {
					while let Some(operateur) = pile_operateurs.pop() {
						if operateur == Operateur::ParentheseEntier {
							break;
						}
						expression.push(Element::Operateur(operateur));
					}
				}
				autre => {
					if format_de_variable(autre) {
						self.programme.variable_est_de_type(autre, TypeElement::Entier)?;
						expression.push(Element::Variable(autre.into(), TypeElement::Entier));
					} else {
						expression.push(texte_comme_nombre(autre)?);
					}
				}
			}
		}
		
		while let Some(operateur) = pile_operateurs.pop() {
			expression.push(Element::Operateur(operateur));
		}
	
		Ok(expression)
	}
}

pub fn affiche_nombre(expression: Vec<Element>, variables: &HashMap<String, Element>) -> Result<String, ErreurPendragon> {
	let nombre = calcule_nombre(expression.clone(), variables)?;
	Ok(nombre_comme_texte(nombre))
}

pub fn calcule_nombre(expression: Vec<Element>, variables: &HashMap<String, Element>) -> Result<usize, ErreurPendragon> {
	let mut pile: Vec<usize> = Vec::new();
	
	for element in expression {
		if let Element::Entier(nombre) = element {
			pile.push(nombre);
			continue;
		}
		if let Element::Variable(nom, _) = element {
			let Some(variable) = variables.get(&nom) else {
				return Err(ErreurPendragon::VariableInconnue(nom.into()))
			};
			if let Element::Entier(nombre) = variable {
				pile.push(*nombre);
				continue
			} else {
				return Err(ErreurPendragon::MauvaisType(nom.into(), variable.type_element().nom(), "entier".into()))
			}
		}
		let Element::Operateur(ref operateur) = element else {
			return Err(ErreurPendragon::MauvaisArgument(format!("{:?}, attendais un opérateur", element)))
		};
		let Some(nombre_a) = pile.pop() else {
			return Err(ErreurPendragon::CalculEntier("la pile est vide".into()))
		};
		let Some(nombre_b) = pile.pop() else {
			return Err(ErreurPendragon::CalculEntier("la pile est vide".into()))
		};
		match operateur {
			Operateur::Plus => {
				pile.push(nombre_b + nombre_a);
			}
			Operateur::Moins => {
				pile.push(nombre_b - nombre_a);
			}
			Operateur::Fois => {
				pile.push(nombre_b * nombre_a);
			}
			Operateur::Divise => {
				pile.push(nombre_b / nombre_a);
			}
			_ => return Err(ErreurPendragon::MauvaisArgument(format!("{:?}, attendais un opérateur d'entiers", element)))
		}
	}
	if pile.len() > 1 {
		return Err(ErreurPendragon::CalculEntier("la pile n'est pas vide".into()))
	}
	Ok(pile[0])
}

pub fn nombre_comme_texte(nombre: usize) -> String {
	if nombre == 0 {
		return "zéro".to_string()
	}
	if nombre >= 10usize.pow(18) {
		return "infini".to_string()
	}
	let mut groupes: Vec<usize> = vec![];
	let mut nombre = nombre;
	while nombre > 0 {
		groupes.insert(0, nombre % 1000);
		nombre /= 1000;
	}
	let mut chaine: String = "".to_string();

	for index in 0..groupes.len() {
		if groupes[index] == 0 {
			continue
		}
		let pluriel: &str = if (groupes.len() - index - 1 > 1) && groupes[index] > 1 {
			"s"
		} else {
			""
		};
		if index < groupes.len() - 1 {
			let union = if index > 0 {UNION} else {""};
			let chiffre = if groupes.len() - index - 1 == 1 && groupes[index] == 1 { // un mille
				"".to_string()
			} else {
				petit_nombre_comme_texte(groupes[index]) + UNION
			};
			chaine += &format!("{}{}{}{}",
				union,
				chiffre,
				NOMS_SEPARATEURS[groupes.len() - index - 1],
				pluriel,
			);
		} else {
			let union = if index > 0 {UNION} else {""};
			chaine += union;
			chaine += &petit_nombre_comme_texte(groupes[index]);
		}
	}
	chaine
}

fn petit_nombre_comme_texte(nombre: usize) -> String {
	let nombre = nombre.clamp(0, 999);
	let centaine = nombre / 100;
	let dizaine = (nombre % 100) / 10;
	let unité = nombre % 10;
	
	let décalage_dizaine = if [1, 7, 9].contains(&dizaine) {1} else {0};
	
	let centaine_texte = if centaine > 1 {
		format!("{}{}cent", NOMS_UNITES[centaine], UNION)
	} else if centaine > 0 {
		"cent".to_string()
	} else {
		"".to_string()
	};

	let dizaine_union = if centaine > 0 && dizaine > 0 {
		UNION.to_string()
	} else {
		"".to_string()
	};

	let dizaine_texte = NOMS_DIZAINES[dizaine - décalage_dizaine];

	let séparation = if unité == 1 && ![0, 1, 8, 9].contains(&dizaine) {UNION.to_string() + "et"} else {"".to_string()};

	let unité_union = if (nombre - unité > 0 && unité > 0 && (nombre%100 > 16 || nombre%100 < 10)) || (unité == 0 && dizaine == 7) {
		UNION.to_string()
	} else {
		"".to_string()
	};
	let unité_texte = if [1, 7, 9].contains(&dizaine) {
		unité_union + NOMS_UNITES_DIX[unité]
	} else {
		unité_union + NOMS_UNITES[unité]
	};
	
	format!("{}{}{}{}{}", centaine_texte, dizaine_union, dizaine_texte, séparation, unité_texte)
}

pub fn texte_comme_nombre(texte: &str) -> Result<Element, ErreurPendragon> {
	if texte == "zéro" {
		return Ok(Element::Entier(0))
	}
	let pluriel = format!("s{}", UNION);
	let mut petits_nombres_texte: Vec<&str> = vec![];
	let mut texte_modifie = texte;
	for (index, separateur_texte) in NOMS_SEPARATEURS.iter().rev().enumerate() {
		if index == NOMS_SEPARATEURS.len() - 1 {
			continue
		}
		let mut un_mille: bool = false;
		if texte_modifie.starts_with("mille") && separateur_texte == &"mille" {
			un_mille = true;
		}
		let texte_separe: Vec<&str> = texte_modifie.split(separateur_texte).collect();
		if texte_separe.len() > 2 {
			return Err(ErreurPendragon::NombreInvalide(texte.to_string()))
		}
		if texte_separe.len() > 1 {
			let petit_nombre_texte = texte_separe[0]
				.trim_start_matches(&pluriel)
				.trim_start_matches(UNION)
				.trim_end_matches(UNION);
			if un_mille {
				petits_nombres_texte.push("un");
			} else {
				petits_nombres_texte.push(petit_nombre_texte);
			}
			texte_modifie = texte_separe[1].trim_start_matches(UNION);
		}
	}
	let petit_nombre_texte = texte_modifie
			.trim_start_matches(&pluriel)
			.trim_start_matches(UNION)
			.trim_end_matches(UNION);
	petits_nombres_texte.push(petit_nombre_texte);

	let mut nombre: usize = 0;

	for (index, petit_nombre_texte) in petits_nombres_texte.iter().enumerate() {
		let petit_nombre = texte_comme_petit_nombre(petit_nombre_texte)?;
		nombre += petit_nombre * 1000usize.pow((petits_nombres_texte.len() - index - 1) as u32);
	}
	
	Ok(Element::Entier(nombre))
}

fn texte_comme_petit_nombre(texte: &str) -> Result<usize, ErreurPendragon> {
	let elements: Vec<&str> = texte.split(UNION).collect();

	let mut nombre = 0;
	let mut dernier_chiffre_texte = "";

	for chiffre_texte in elements {
		if chiffre_texte == "cent" {
			if nombre == 0 {
				nombre = 1;
			}
			if nombre >= 100 {
				return Err(ErreurPendragon::NombreInvalide(texte.to_string()))
			}
			nombre *= 100;
			dernier_chiffre_texte = chiffre_texte;
			continue
		}
		if chiffre_texte == "vingts" {
			if dernier_chiffre_texte == "quatre" {
				nombre += 80 - 4;
				dernier_chiffre_texte = chiffre_texte;
				continue
			} else {
				return Err(ErreurPendragon::NombreInvalide(texte.to_string()))
			}
		}
		if let Some(chiffre) = NOMS_UNITES.iter().position(|&s| s == chiffre_texte) {
			if nombre%10 > 0 {
				return Err(ErreurPendragon::NombreInvalide(texte.to_string()))
			}
			nombre += chiffre;
			dernier_chiffre_texte = chiffre_texte;
			continue
		}
		if let Some(chiffre) = NOMS_DIZAINES.iter().position(|&s| s == chiffre_texte) {
			if nombre%100 > 0 && chiffre != 1 {
				return Err(ErreurPendragon::NombreInvalide(texte.to_string()))
			}
			nombre += chiffre*10;
			dernier_chiffre_texte = chiffre_texte;
			continue
		}
		if let Some(chiffre) = NOMS_UNITES_DIX.iter().position(|&s| s == chiffre_texte) {
			if nombre%10 > 0 {
				return Err(ErreurPendragon::NombreInvalide(texte.to_string()))
			}
			nombre += 10 + chiffre;
			dernier_chiffre_texte = chiffre_texte;
			continue
		}
		if chiffre_texte == "et" {
			continue
		}
		return Err(ErreurPendragon::NombreInvalide(texte.to_string()))
	}

	Ok(nombre)
}




// -----------------------------------------------------------------------


#[cfg(test)]
mod test {
	use std::collections::HashMap;
	use super::*;
	#[test]
	fn teste_conversion_nombres_texte() {
		for i in [0, 1, 42, 70, 123, 999, 1031, 1_001_091, 72_036_854_775_807usize].iter() {
			let texte = nombre_comme_texte(*i); // Convert number to text
			match texte_comme_nombre(&texte) { // Convert text back to number
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
	fn teste_calcul_nombre() {
		let pendragon = Pendragon::nouveau();
		let a = 2345678;
		let b = 987654;
		let c = 34523456;
		let d = 45678;
		let e = 2;
		let possible_expression = pendragon.elements_nombre(&format!("{} fois {} plus ouvre la parenthèse {} moins {} ferme la parenthèse divisé par {}",
			nombre_comme_texte(a),
			nombre_comme_texte(b),
			nombre_comme_texte(c),
			nombre_comme_texte(d),
			nombre_comme_texte(e)));
		match possible_expression {
			Ok(expression) => {
				match calcule_nombre(expression, &HashMap::new()) {
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
}
