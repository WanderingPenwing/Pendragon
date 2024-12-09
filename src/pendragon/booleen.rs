use super::Pendragon;
use super::ErreurPendragon;
use super::Element;

impl Pendragon {
	pub fn condition(&self, arguments: &str) -> Result<bool, ErreurPendragon> {
		self.condition_elementaire(arguments)
	}
	
	pub fn condition_elementaire(&self, texte: &str) -> Result<bool, ErreurPendragon> {
		let mut expression: Vec<String> = texte.split(" ").map(String::from).collect();
		
		let mut index = 0;
		while index < expression.len() {
			if expression[index] != "non" {
				index += 1;
				continue;
			}
			if index == expression.len() - 1 {
				return Err(ErreurPendragon::ManqueArgument);
			}
			let a = self.texte_comme_booleen(&expression[index + 1])?;
			expression[index] = booleen_comme_texte(!a);
			expression.remove(index + 1);
		}
		let mut index = 0;
		while index < expression.len() {
			if expression[index] != "et" {
				index += 1;
				continue;
			}
			if index == 0 || index == expression.len() - 1 {
				return Err(ErreurPendragon::ManqueArgument);
			}
			let a = self.texte_comme_booleen(&expression[index - 1])?;
			let b = self.texte_comme_booleen(&expression[index + 1])?;
			index -= 1;
			expression[index] = booleen_comme_texte(a && b);
			expression.remove(index + 1);
			expression.remove(index + 1);
		}
		let mut index = 0;
		while index < expression.len() {
			if expression[index] != "ou" {
				index += 1;
				continue;
			}
			if index == 0 || index == expression.len() - 1 {
				return Err(ErreurPendragon::ManqueArgument);
			}
			let a = self.texte_comme_booleen(&expression[index - 1])?;
			let b = self.texte_comme_booleen(&expression[index + 1])?;
			index -= 1;
			expression[index] = booleen_comme_texte(a || b);
			expression.remove(index + 1);
			expression.remove(index + 1);
		}
		if expression.len() > 1 {
			return Err(ErreurPendragon::MauvaisArgument("expression booléenne".to_string()))
		}
		self.texte_comme_booleen(&expression[0])
	}
	
	pub fn texte_comme_booleen(&self, texte: &str) -> Result<bool, ErreurPendragon> {
		if texte.chars().next().map_or(false, |c| c.is_uppercase()) {
			if self.variables.contains_key(texte) {
				let Element::Booleen(booleen) = self.variables[texte] else {
					return Err(ErreurPendragon::MauvaisType(texte.into(), self.variables[texte].type_element().nom(), "booleen".into()))
				};
				return Ok(booleen);
			} else {
				return Err(ErreurPendragon::VariableInconnue(texte.to_string()))
			}
		}
		texte_comme_booleen(texte)
	}
}

pub fn booleen_comme_texte(booleen: bool) -> String {
	if booleen {
		"vrai".into()
	} else {
		"faux".into()
	}
}

pub fn texte_comme_booleen(texte: &str) -> Result<bool, ErreurPendragon> {
	match texte {
		"vrai" => Ok(true),
		"faux" => Ok(false),
		_ => Err(ErreurPendragon::BooleenInvalide(texte.into())),
	}
}