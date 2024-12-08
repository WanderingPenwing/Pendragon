use std::fmt;
use std::collections::HashMap;

pub mod nombres;

#[cfg(test)]
mod tests;

pub enum ErreurSophie {
	CommandeInconnue(String),
	PhraseVide,
	ManqueArgument,
	OrthographeNombre(String),
	MauvaisArgument(String),
	DesequilibreParenthese,
	VariableInconnue(String),
	MauvaisType(String),
}

#[derive(PartialEq, Debug)]
pub enum Variable {
	Entier(usize),
	Texte(String),
}

impl fmt::Display for ErreurSophie {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {//'
		match self {
			Self::CommandeInconnue(commande) => write!(f, "La commande \"{}\" est inconnue.", commande),
			Self::PhraseVide => write!(f, "La phrase est vide."),
			Self::ManqueArgument => write!(f, "Il manque un argument."),
			Self::OrthographeNombre(nombre) => write!(f, "Le nombre \"{}\" est mal orthographié.", nombre),
			Self::MauvaisArgument(message) => write!(f, "La commande a reçu un mauvais argument, {}.", message),
			Self::DesequilibreParenthese => write!(f, "Les parenthèses sont déséquilibrés."),
			Self::VariableInconnue(nom) => write!(f, "La variable \"{}\" est inconnue.", nom),
			Self::MauvaisType(attendu) => write!(f, "La variable est du mauvais type, {}.", attendu),
		}
	}
}

pub struct Sophie {
	variables: HashMap<String, Variable>,
}

impl Sophie {
	pub fn new() -> Self {
		Self {
			variables: HashMap::new(),
		}
	}
	pub fn execute(&mut self, contenu: String) {
		let contenu_propre = contenu.replace("\n", "");
		let mut texte: Vec<&str> = contenu_propre.split('.').collect();
		texte.pop(); // remove empty phrase after last dot
		for (index_phrase, phrase) in texte.iter().enumerate() {
			match self.execute_phrase(phrase) {
				Ok(_) => {},
				Err(raison) => {
					eprintln!("Erreur phrase {} : {}", index_phrase + 1, raison);
					return
				}
			}
		}
	}

	fn execute_phrase(&mut self, phrase: &str) -> Result<(), ErreurSophie> {
		let phrase = phrase.trim();
		let parties: Vec<&str> = phrase.splitn(2, ' ').collect();

		if parties.is_empty() {
			return Err(ErreurSophie::PhraseVide)
		}

		if parties.len() == 1 {
			return Err(ErreurSophie::ManqueArgument)
		}

		match parties[0] {
			"Définie" => {
				self.definie(parties[1])?;
			}
			"Modifie" => {
				self.modifie(parties[1])?;
			},
			"Affiche" => {
				self.affiche(parties[1])?;
			},
			"Demande" => {
				self.demande(parties[1])?;
			}
			autre_commande => {
				return Err(ErreurSophie::CommandeInconnue(autre_commande.to_string()))
			}
		};
		Ok(())
	}

	fn definie(&mut self, arguments: &str) -> Result<(), ErreurSophie> {
		let (variable_nom, variable_type) = nom_de_variable(arguments, "comme")?;
	
		let contenu = match variable_type.as_str() {
			"entier" => Variable::Entier(0),
			"texte" => Variable::Texte("".to_string()),
			_ => return Err(ErreurSophie::MauvaisArgument("type de variable inconnu".into())),
		};
	
		self.variables.insert(variable_nom, contenu);
		Ok(())
	}
	
	fn modifie(&mut self, arguments: &str) -> Result<(), ErreurSophie> {
		let (variable_nom, contenu) = nom_de_variable(arguments, "avec")?;
		if !self.variables.contains_key(&variable_nom) {
			return Err(ErreurSophie::VariableInconnue(variable_nom))
		}

		let valeur = self.operation(&contenu)?;
		self.variables.insert(variable_nom, Variable::Entier(valeur));
		
		Ok(())
	}

	fn affiche(&self, arguments: &str) -> Result<(), ErreurSophie> {
		let liste_arguments: Vec<&str> = arguments.split(',').collect();

		let mut texte = "".to_string();

		for argument in liste_arguments {
			let argument: &str = argument.trim();
			if argument.starts_with('"') {
				if argument.ends_with('"') {
					texte += &argument[1..argument.len()-1];
				}
			} else {
				let resultat = self.operation(argument)?;
				texte += &nombres::nombre_comme_texte(resultat);
			}
		}
		println!("{}", texte);
		Ok(())
	}

	fn demande(&self, arguments: &str) -> Result<(), ErreurSophie> {
		println!("- demande : {}", arguments);
		Ok(())
	}
}

fn nom_de_variable(arguments: &str, separateur: &str) -> Result<(String, String), ErreurSophie> {
	let parties: Vec<&str> = arguments.splitn(2, separateur).collect();
	let nom_variable: String = parties[0].trim().to_string();
	if parties.len() == 1 {
		return Err(ErreurSophie::ManqueArgument)
	}
	let Some(first_char) = nom_variable.chars().next() else {
		return Err(ErreurSophie::MauvaisArgument("il n'y a pas de variable".to_string()))
	};
	if !first_char.is_uppercase() {
		return Err(ErreurSophie::MauvaisArgument("il manque une majuscule à la variable".to_string()))
	}
	Ok((nom_variable, parties[1].trim().to_string()))
}
