use super::*;

impl Pendragon {
	pub fn elements_texte(&self, arguments: &str) -> Result<Vec<Element>, ErreurPendragon> {
		let texte = arguments.replace("\"", " \" ");
		let elements_texte = texte.split(" ");
		
		let mut expression: Vec<Element> = Vec::new();
		
		let mut pile_texte: Vec<String> = Vec::new();
		let mut pile_inconnu: Vec<String> = Vec::new();
		
		for element in elements_texte {
			if element == "\"" {
				if !pile_texte.is_empty() {
					let element_texte = pile_texte[1..pile_texte.len()].join(" ");
					if let Some(dernier_element) = expression.last() {
						if *dernier_element != Element::Operateur(Operateur::Puis) {
							return Err(ErreurPendragon::TexteInvalide(format!("attends un 'puis' entre '{}' et '{}'", dernier_element, element)))
						}
					}
					expression.push(Element::Texte(element_texte));
					pile_texte = Vec::new();
				} else {
					pile_texte.push(element.into());
				}
				continue;
			}
			if !pile_texte.is_empty() {
				pile_texte.push(element.into());
				continue;
			}
			if element == "" {
				continue;
			}
			if element != "puis" {
				pile_inconnu.push(element.into());
				continue;
			}
			expression.extend(self.puis(&expression, &pile_inconnu)?);
			pile_inconnu = Vec::new();
			expression.push(Element::Operateur(Operateur::Puis));
		}
		expression.extend(self.puis(&expression, &pile_inconnu)?);
		pile_inconnu = Vec::new();
		expression.push(Element::Operateur(Operateur::Puis));
		Ok(expression)
	}
	
	pub fn puis(&self, expression: &Vec<Element>, pile_inconnu: &Vec<String>) -> Result<Vec<Element>, ErreurPendragon> {
		let Some(premier_element) = pile_inconnu.first() else {
			if let Some(dernier_element) = expression.last() {
				if let Element::Texte(_) = dernier_element.clone() {
					return Ok(vec![]);
				}
			}
			return Err(ErreurPendragon::TexteInvalide("il manque un élément avant le puis".into()))
		};
		let total_inconnu = pile_inconnu.join(" ");
		if total_inconnu == "alinéa" {
			return Ok(vec![Element::Texte("\t".into())])
		}
		if total_inconnu == "retour à la ligne" {
			return Ok(vec![Element::Texte("\n".into())])
		}
		if pile_inconnu.len() == 1 && format_de_variable(premier_element) {
			return Ok(vec![Element::Variable(premier_element.into(), self.programme.variable(premier_element)?)]);
		}
		let Err(raison) = self.elements_nombre(premier_element) else {
			return self.elements_nombre(&total_inconnu)
		};
		if let ErreurPendragon::CalculEntier(_) = raison {
			return Err(raison)
		}
		let Err(raison) = self.elements_booleen(premier_element) else {
			return self.elements_booleen(&total_inconnu)
		};
		let ErreurPendragon::CalculBooleen(_) = raison else {
			return Err(ErreurPendragon::TexteInvalide(format!("'{}' ne peut pas être converti en texte", pile_inconnu.join(" "))));
		};
		Err(raison)
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
		let Operateur::Puis = operateur else {
			pile.push(element);
			continue;
		};
		let Some(element_pile) = pile.last() else {
			continue;
		};
		if let TypeElement::Booleen = element_pile.type_element() {
			texte += &booleen::affiche_booleen(pile.clone(), variables)?;
			pile = Vec::new();
			continue;
		}
		if let TypeElement::Entier = element_pile.type_element() {
			texte += &nombre::affiche_nombre(pile.clone(), variables)?;
			pile = Vec::new();
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
				return Err(ErreurPendragon::MauvaisArgument(format!("{}", autre)))
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
	fn calcul_texte() {
		let pendragon = Pendragon::nouveau();
		let a = 2345678;
		let b = 987654;
		
		let possible_expression = pendragon.elements_texte(&format!("\"hello\" puis {} fois {} puis \"there\" puis vrai ou faux puis trois puis deux puis alinéa puis retour à la ligne",
			nombre::nombre_comme_texte(a),
			nombre::nombre_comme_texte(b)));
		match possible_expression {
			Ok(expression) => {
				match calcule_texte(expression, &HashMap::new()) {
					Ok(texte) => {
						let vrai_texte = format!("hello{}therevraitroisdeux\t\n", nombre::nombre_comme_texte(a*b));
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
		let pendragon = Pendragon::nouveau();
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
		let pendragon = Pendragon::nouveau();
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
}