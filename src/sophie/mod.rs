use std::fmt;
use std::io;
use std::collections::HashMap;

pub mod nombre;
pub mod texte;
pub mod booleen;

#[cfg(test)]
mod tests;

#[derive(PartialEq, Debug, Clone)]
pub enum Variable {
	Entier(usize),
	Texte(String),
	Booleen(bool),
}

impl Variable {
	pub fn nom_type(&self) -> String {
		match self {
			Self::Entier(_) => "entier".into(),
			Self::Texte(_) => "texte".into(),
			Self::Booleen(_) => "booléen".into(),
		}
	}
}

pub enum ErreurSophie {
	CommandeInconnue(String),
	PhraseVide,
	ManqueArgument,
	NombreInvalide(String),
	BooleenInvalide(String),
	TexteInvalide(String),
	MauvaisArgument(String),
	DesequilibreParenthese,
	VariableInconnue(String),
	MauvaisType(String, String, String),
	ProblemeTerminal(String),
	ManquePoint,
}

impl fmt::Display for ErreurSophie {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {//'
		match self {
			Self::CommandeInconnue(commande) => write!(f, "La commande \"{}\" est inconnue.", commande),
			Self::PhraseVide => write!(f, "La phrase est vide."),
			Self::ManqueArgument => write!(f, "Il manque un argument."),
			Self::NombreInvalide(nombre) => write!(f, "Le nombre \"{}\" est mal orthographié.", nombre),
			Self::TexteInvalide(raison) => write!(f, "Le texte est invalide, {}.", raison),
			Self::BooleenInvalide(booleen) => write!(f, "Le booleen \"{}\" est mal orthographié.", booleen),
			Self::MauvaisArgument(message) => write!(f, "La commande a reçu un mauvais argument, {}.", message),
			Self::DesequilibreParenthese => write!(f, "Les parenthèses sont déséquilibrés."),
			Self::VariableInconnue(nom) => write!(f, "La variable \"{}\" est inconnue.", nom),
			Self::MauvaisType(nom, type_variable, type_attendu) => write!(f, "La variable {} est du mauvais type ({}), attendais {}.", nom, type_variable, type_attendu),
			Self::ProblemeTerminal(probleme) => write!(f, "Problème d'accès terminal : {}.", probleme),
			Self::ManquePoint => write!(f, "Il manque un point."),
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
	pub fn execute(&mut self, contenu: String) -> Result<(), ErreurSophie> {
		let contenu_propre = contenu.replace("\n", "");
		let mut texte: Vec<&str> = contenu_propre.split('.').collect();
		let reste = texte.pop(); // remove empty phrase after last dot
		if reste != Some("") {
			eprintln!("Erreur phrase {} : Il manque un point.", texte.len() + 1);
			return Err(ErreurSophie::ManquePoint)
		}
		for (index_phrase, phrase) in texte.iter().enumerate() {
			match self.execute_phrase(phrase) {
				Ok(_) => {},
				Err(raison) => {
					eprintln!("Erreur phrase {} : {}", index_phrase + 1, raison);
					return Err(raison)
				}
			}
		}
		Ok(())
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
			"Définis" => {
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

	fn affiche(&self, arguments: &str) -> Result<(), ErreurSophie> {
		println!("{}", self.texte(arguments)?);
		Ok(())
	}
	
	fn definie(&mut self, arguments: &str) -> Result<(), ErreurSophie> {
		let (variable_nom, variable_type) = self.nom_de_variable(arguments, "comme")?;
		
		let possible_variable = self.recupere_variable(&variable_nom);
		
		let Err(raison) = possible_variable else {
			return Err(ErreurSophie::MauvaisArgument(format!("la variable \"{}\" existe déjà", variable_nom)))
		};
		let ErreurSophie::VariableInconnue(_) = raison else {
			return Err(raison)
		};
	
		let contenu = match variable_type.as_str() {
			"entier" => Variable::Entier(0),
			"texte" => Variable::Texte("".to_string()),
			"booléen" => Variable::Booleen(false),
			_ => return Err(ErreurSophie::MauvaisArgument(format!("type de variable \"{}\" inconnu", variable_type))),
		};
	
		self.variables.insert(variable_nom, contenu);
		Ok(())
	}
	
	fn modifie(&mut self, arguments: &str) -> Result<(), ErreurSophie> {
		let (variable_nom, contenu) = self.nom_de_variable(arguments, "avec")?;
		let variable = self.recupere_variable(&variable_nom)?;

		let valeur = match variable {
			Variable::Entier(_) => Variable::Entier(self.operation(&contenu)?),
			Variable::Texte(_) => Variable::Texte(self.texte(&contenu)?),
			Variable::Booleen(_) => Variable::Booleen(self.condition(&contenu)?),
		};
		self.variables.insert(variable_nom, valeur);
		
		Ok(())
	}

	fn demande(&mut self, arguments: &str) -> Result<(), ErreurSophie> {
		let (variable_nom, _) = self.nom_de_variable(arguments, "")?;
		
		let _ = self.recupere_variable(&variable_nom)?;
		
		println!("Quelle valeur pour {} ?", variable_nom);
		let mut reponse: String = String::new();
		if let Err(_) = io::stdin().read_line(&mut reponse) {
			return Err(ErreurSophie::ProblemeTerminal("lecture d'entrées utilisateur impossible".into()))
		}
		
		let contenu = reponse.trim();
		
		let valeur = match self.variables[&variable_nom] {
			Variable::Entier(_) => Variable::Entier(nombre::texte_comme_nombre(contenu)?),
			Variable::Texte(_) => Variable::Texte(contenu.into()),
			Variable::Booleen(_) => Variable::Booleen(booleen::texte_comme_booleen(contenu)?),
		};
		self.variables.insert(variable_nom, valeur);
		
		Ok(())
	}
	
	fn recupere_variable(&self, nom: &str) -> Result<Variable, ErreurSophie> {
		let Some(first_char) = nom.chars().next() else {
			return Err(ErreurSophie::MauvaisArgument("il n'y a pas de variable".to_string()))
		};
		if !first_char.is_uppercase() {
			return Err(ErreurSophie::MauvaisArgument("il manque une majuscule à la variable".to_string()))
		}
		if !self.variables.contains_key(nom) {
			return Err(ErreurSophie::VariableInconnue(nom.into()))
		}
		Ok(self.variables[nom].clone())
	}
	
	fn nom_de_variable(&self, arguments: &str, separateur: &str) -> Result<(String, String), ErreurSophie> {
		let parties = if separateur == "" {
			vec![arguments, ""]
		} else {
			arguments.splitn(2, separateur).collect()
		};
		
		let nom_variable = parties[0].trim().to_string();
		
		if parties.len() == 1 {
			return Err(ErreurSophie::ManqueArgument)
		}
		Ok((nom_variable, parties[1].trim().to_string()))
	}
}
