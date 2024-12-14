use super::*;

pub const NOMS_UNITES: [&str; 10] = ["", "un", "deux", "trois", "quatre", "cinq", "six", "sept", "huit", "neuf"];
pub const NOMS_UNITES_DIX: [&str; 10] = ["dix", "onze", "douze", "treize", "quatorze", "quinze", "seize", "dix-sept", "dix-huit", "dix-neuf"];
pub const NOMS_DIZAINES: [&str; 9] = ["", "dix", "vingt", "trente", "quarante", "cinquante", "soixante", "x", "quatre-vingts"];
pub const NOMS_SEPARATEURS: [&str; 7] = ["", "mille", "million", "milliard", "billion", "billiard", "trillion"];
pub const UNION: &str = "-";

impl Pendragon {
	pub fn elements_nombre(&self, arguments: &str) -> Result<Vec<Element>, ErreurPendragon> {
		let texte = arguments
					.replace("ouvre la parenthèse", "ouvre-la-parenthese")
					.replace("ferme la parenthèse", "ferme-la-parenthese")
					.replace("divisé par", "divise-par");
		let elements_texte: Vec<&str> = texte.split(" ").collect();
		let mut expression: Vec<Element> = Vec::new();
		let mut pile_operateurs: Vec<Operateur> = Vec::new();
		let mut precede_par_operation: bool = true;
	
		for (index, element) in elements_texte.iter().enumerate() {
			let element_precedent = if index > 0 {
				elements_texte[index-1]
			} else {
				"le début"
			};
			let element: &str = *element;
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
				"ouvre-la-parenthese" => {
					if !precede_par_operation {
						return Err(ErreurPendragon::OrdreCalculEntier("opérateur".into(), element_precedent.into(), "l'ouverture de parenthèse".into()))
					}
					pile_operateurs.push(Operateur::ParentheseEntier);
					continue
				}
				"ferme-la-parenthese" => {
					if precede_par_operation {
						return Err(ErreurPendragon::OrdreCalculEntier("nombre".into(), element_precedent.into(), "la fermeture de parenthèse".into()))
					}
					while let Some(operateur) = pile_operateurs.pop() {
						if operateur == Operateur::ParentheseEntier {
							break;
						}
						expression.push(Element::Operateur(operateur));
					}
					continue
				}
				autre => {
					if !precede_par_operation {
						return Err(ErreurPendragon::OrdreCalculEntier("opérateur".into(), element_precedent.into(), autre.into()))
					}
					precede_par_operation = false;
					if format_de_variable(autre) {
						self.programme.variable_est_de_type(autre, TypeElement::Entier)?;
						expression.push(Element::Variable(autre.into(), TypeElement::Entier));
					} else {
						expression.push(texte_comme_nombre(autre)?);
					}
					continue;
				}
			}
			if precede_par_operation {
				return Err(ErreurPendragon::OrdreCalculEntier("nombre".into(), element_precedent.into(), element.into()))
			}
			precede_par_operation = true;
		}
		
		while let Some(operateur) = pile_operateurs.pop() {
			expression.push(Element::Operateur(operateur));
		}
	
		Ok(expression)
	}
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
	let texte = texte.trim();
	if texte.starts_with("et") || texte.ends_with("et") {
		return Err(ErreurPendragon::NombreInvalide(texte.to_string()))
	}
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
