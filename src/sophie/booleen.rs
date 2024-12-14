use super::*;

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
			return Err(ErreurPendragon::MauvaisArgument(format!("{}, attendais un opérateur", element)))
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
			_ => return Err(ErreurPendragon::MauvaisArgument(format!("{}, attendais un opérateur booléen", element)))
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