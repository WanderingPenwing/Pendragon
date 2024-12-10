use super::*;

impl Pendragon {
	pub fn elements_booleen(&self, arguments: &str) -> Result<Vec<Element>, ErreurPendragon> {
		let texte = arguments
					.replace("ouvre la parenthèse", "ouvre-la-parenthese")
					.replace("ferme la parenthèse", "ferme-la-parenthese");
		let elements_texte: Vec<&str> = texte.split(" ").collect();
		let mut expression: Vec<Element> = Vec::new();
		let mut pile_operateurs: Vec<Operateur> = Vec::new();
	
		for element in elements_texte {
			match element {
				"vrai" => expression.push(Element::Booleen(true)),
				"faux" => expression.push(Element::Booleen(false)),
				"non" => pile_operateurs.push(Operateur::Non),
				"et" => {
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
					while let Some(operateur) = pile_operateurs.last() {
						if *operateur == Operateur::Non || *operateur == Operateur::Et || *operateur == Operateur::Ou {
							expression.push(Element::Operateur(pile_operateurs.pop().unwrap()));
						} else {
							break;
						}
					}
					pile_operateurs.push(Operateur::Ou);
				}
				"ouvre-la-parenthese" => pile_operateurs.push(Operateur::ParentheseBooleen),
				"ferme-la-parenthese" => {
					while let Some(operateur) = pile_operateurs.pop() {
						if operateur == Operateur::ParentheseBooleen {
							break;
						}
						expression.push(Element::Operateur(operateur));
					}
				}
				autre => {
					if !format_de_variable(autre) {
						return Err(ErreurPendragon::MauvaisArgument(format!("{}", autre)))
					}
					self.programme.variable_est_de_type(autre, TypeElement::Booleen)?;
					expression.push(Element::Variable(autre.into(), TypeElement::Booleen));
				}
			}		
		}
		
		while let Some(operateur) = pile_operateurs.pop() {
			expression.push(Element::Operateur(operateur));
		}
	
		Ok(expression)
	}
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
		return Err(ErreurPendragon::CalculBooleen("la pile n'est pas vide".into()))
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