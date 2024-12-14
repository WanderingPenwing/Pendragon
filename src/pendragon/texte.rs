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