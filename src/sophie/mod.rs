use std::io;
use std::collections::HashMap;

pub mod nombre;
pub mod texte;
pub mod booleen;
pub mod structure;
use structure::*;

#[cfg(test)]
mod tests;

pub struct Sophie {
	variables: HashMap<String, Element>,
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
			"entier" => Element::Entier(0),
			"texte" => Element::Texte("".to_string()),
			"booléen" => Element::Booleen(false),
			_ => return Err(ErreurSophie::MauvaisArgument(format!("type de variable \"{}\" inconnu", variable_type))),
		};
	
		self.variables.insert(variable_nom, contenu);
		Ok(())
	}
	
	fn modifie(&mut self, arguments: &str) -> Result<(), ErreurSophie> {
		let (variable_nom, contenu) = self.nom_de_variable(arguments, "avec")?;
		let variable = self.recupere_variable(&variable_nom)?;

		let valeur = match variable.type_element() {
			TypeElement::Entier => Element::Entier(self.operation(&contenu)?),
			TypeElement::Texte => Element::Texte(self.texte(&contenu)?),
			TypeElement::Booleen => Element::Booleen(self.condition(&contenu)?),
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
		
		let valeur = match self.variables[&variable_nom].type_element() {
			TypeElement::Entier => Element::Entier(nombre::texte_comme_nombre(contenu)?),
			TypeElement::Texte => Element::Texte(contenu.into()),
			TypeElement::Booleen => Element::Booleen(booleen::texte_comme_booleen(contenu)?)
		};
		self.variables.insert(variable_nom, valeur);
		
		Ok(())
	}
	
	fn recupere_variable(&self, nom: &str) -> Result<Element, ErreurSophie> {
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
