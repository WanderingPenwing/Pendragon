use super::*;

impl Pendragon {
	pub fn elements_texte(&self, arguments: &str) -> Result<Vec<Element>, ErreurPendragon> {
		let mut expression: Vec<Element> = Vec::new();
		
		for argument in arguments.split(',').map(|arg| arg.trim()) {
			if expression.len() > 0 {
				expression.push(Element::Operateur(Operateur::Virgule));
			}
			if argument.starts_with('"') {
				if argument.ends_with('"') {
					expression.push(Element::Texte(argument[1..argument.len() - 1].into()));
				} else {
					return Err(ErreurPendragon::TexteInvalide("guillemet mal refermé".into()))
				}
				continue;
			}
			if format_de_variable(argument) && !argument.contains(" ") {
				expression.push(Element::Variable(argument.into(), self.programme.variable(argument)?));
				continue;
			}
			if let Ok(elements_nombre) = self.elements_nombre(argument) {
				expression.extend(elements_nombre);
			} else if let Ok(elements_booleen) = self.elements_booleen(argument) {
				expression.extend(elements_booleen);
			} else {
				return Err(ErreurPendragon::MauvaisArgument(argument.to_string()));
			}
		}
		expression.push(Element::Operateur(Operateur::Virgule));
		Ok(expression)
	}
}

pub fn calcule_texte(expression: Vec<Element>, variables: &HashMap<String, Element>) -> Result<String, ErreurPendragon> {
	let mut pile: Vec<Element> = Vec::new();
	let mut texte: String = String::new();
	
	for element in expression {
		let Element::Operateur(ref operateur) = element else {
			pile.push(element);
			continue;
		};
		let Operateur::Virgule = operateur else {
			pile.push(element);
			continue;
		};
		let Some(element_pile) = pile.last() else {
			continue;
		};
		if let TypeElement::Booleen = element_pile.type_element() {
			texte += &booleen::affiche_booleen(pile.clone(), variables)?;
			continue;
		}
		if let TypeElement::Entier = element_pile.type_element() {
			texte += &nombre::affiche_nombre(pile.clone(), variables)?;
			continue;
		}
		match element_pile {
			Element::Texte(contenu) => texte += contenu,
			Element::Variable(nom, type_element) => {
				let Some(variable) = variables.get(nom) else {
					return Err(ErreurPendragon::VariableInconnue(nom.into()))
				};
				let Element::Texte(contenu) = variable else {
					return Err(ErreurPendragon::MauvaisType(nom.into(), type_element.nom(), "texte".into()))
				};
				texte += &contenu;
			}
			autre => {
				return Err(ErreurPendragon::MauvaisArgument(format!("{:?}", autre)))
			}
		}
		pile = Vec::new();
	}
	Ok(texte)
}



// -----------------------------------------------------------------------


#[cfg(test)]
mod test {
	use std::collections::HashMap;
	use super::*;
	
	#[test]
	fn teste_calcul_texte() {
		let pendragon = Pendragon::nouveau();
		let a = 2345678;
		let b = 987654;
		
		let possible_expression = pendragon.elements_texte(&format!("\"hello\", {} fois {}, \"there\", vrai ou faux",
			nombre::nombre_comme_texte(a),
			nombre::nombre_comme_texte(b)));
		match possible_expression {
			Ok(expression) => {
				match calcule_texte(expression, &HashMap::new()) {
					Ok(texte) => {
						let vrai_texte = format!("hello{}therevrai", nombre::nombre_comme_texte(a*b));
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
}