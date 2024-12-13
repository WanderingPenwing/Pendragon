use std::collections::HashMap;

pub mod nombre;
pub mod texte;
pub mod booleen;
pub mod structure;
use structure::*;
pub mod debug;
use debug::*;

pub struct Pendragon {
	pub programme: Programme,
}

impl Pendragon {
	pub fn nouveau() -> Self {
		Self {
			programme: Programme::nouveau(),
		}
	}
	
	pub fn compile(&mut self, contenu: String) -> Result<(), ErreurCompilation> {
		let texte: Vec<&str> = contenu.split('\n').collect();
		for (index_ligne, ligne) in texte.iter().enumerate() {
			let ligne = ligne.trim();
			let phrases: Vec<&str> = ligne.split_inclusive(|c| c == ',' || c == '.').collect();
			let Some(derniere_phrase) = phrases.last() else {
				continue
			};
			if !derniere_phrase.ends_with('.') && !derniere_phrase.ends_with(',') {
				return Err(ErreurCompilation::nouvelle(index_ligne, ligne.into(), ErreurPendragon::ManquePonctuation))
			}
			for phrase in phrases {
				if phrase.ends_with(".") {
					if phrase.replace(" ", "").starts_with("NotaBene:") {
						continue
					}
					match self.compile_commande(&phrase[..phrase.len() - 1]) {
						Ok(commande) => self.programme.ajoute_commande(commande),
						Err(raison) => return Err(ErreurCompilation::nouvelle(index_ligne, ligne.into(), raison)),
					}
					continue;
				}
				println!("todo : {}", phrase);
			}
		}
		Ok(())
	}
	
	fn compile_commande(&mut self, phrase: &str) -> Result<Commande, ErreurPendragon> {
		let phrase = phrase.trim();
		let parties: Vec<&str> = phrase.splitn(2, ' ').collect();
		if parties.len() == 1 {
			return Err(ErreurPendragon::ManqueArgument)
		}
		if parties[1].contains("Définis") || parties[1].contains("Modifie") || parties[1].contains("Affiche") || parties[1].contains("Demande") {
			return Err(ErreurPendragon::ManquePonctuation)
		}
		match parties[0] {
			"Définis" => self.definis(parties[1]),
			"Modifie" => self.modifie(parties[1]),
			"Affiche" => self.affiche(parties[1]),
			"Demande" => self.demande(parties[1]),
			autre => Err(ErreurPendragon::CommandeInconnue(autre.into())),
		}
	}

	fn affiche(&self, arguments: &str) -> Result<Commande, ErreurPendragon> {
		Ok(Commande::Affiche(self.elements_texte(arguments)?))
	}
	
	fn definis(&mut self, arguments: &str) -> Result<Commande, ErreurPendragon> {
		let (variable_nom, variable_type_texte) = nom_de_variable(arguments, "comme")?;
		let variable_type = TypeElement::texte_comme_type(&variable_type_texte)?;
		
		self.programme.ajoute_variable(variable_nom.clone(), variable_type.clone())?;
		
		Ok(Commande::Definis(variable_nom.into(), variable_type))
	}
	
	fn modifie(&mut self, arguments: &str) -> Result<Commande, ErreurPendragon> {
		let (variable_nom, contenu) = nom_de_variable(arguments, "avec")?;
		let variable = self.programme.variable(&variable_nom)?;
		
		let elements = match variable {
			TypeElement::Entier => self.elements_nombre(&contenu)?,
			TypeElement::Texte => self.elements_texte(&contenu)?,
			TypeElement::Booleen => self.elements_booleen(&contenu)?,
		};
		
		Ok(Commande::Modifie(variable_nom, elements))
	}

	fn demande(&mut self, arguments: &str) -> Result<Commande, ErreurPendragon> {
		let (variable_nom, _) = nom_de_variable(arguments, "")?;
		let _ = self.programme.variable(&variable_nom)?;
		
		Ok(Commande::Demande(variable_nom.into()))
	}
}

fn nom_de_variable(arguments: &str, separateur: &str) -> Result<(String, String), ErreurPendragon> {
	let parties = if separateur == "" {
		vec![arguments, ""]
	} else {
		arguments.splitn(2, separateur).collect()
	};
	
	let nom_variable = parties[0].trim().to_string();
	
	if parties.len() == 1 {
		return Err(ErreurPendragon::ManqueArgument)
	}
	Ok((nom_variable, parties[1].trim().to_string()))
}

pub fn format_de_variable(nom: &str) -> bool {
	let Some(first_char) = nom.chars().next() else {
		return false
	};
	if !first_char.is_uppercase() {
		return false
	}
	true
}

#[cfg(test)]
mod test {
	use std::collections::HashMap;
	use super::*;
	
	#[test]
	fn commentaire_valide() {
		let mut pendragon = Pendragon::nouveau();
		let commentaires = [
			"Nota Bene : ceci est un commentaire.",
			"NotaBene : ceci est un commentaire.",
			"Nota Bene: ceci est un commentaire.",
			"NotaBene: ceci est un commentaire.",
			"Nota Bene :ceci est un commentaire.",
			"NotaBene :ceci est un commentaire.",
			"Nota Bene:ceci est un commentaire.",
			"NotaBene:ceci est un commentaire."
		];
		for commentaire in commentaires {
			match pendragon.compile(commentaire.into()) {
				Ok(_) => assert_eq!(pendragon.programme.commandes.len(), 0, "Le commentaire '{}' ne devrait pas générer de commande", commentaire),
				Err(raison) => panic!("Erreur de compilation du commentaire '{}' : {}", commentaire, raison)
			}
		}
	}
	
	#[test]
	fn commentaire_invalide() {
		let mut pendragon = Pendragon::nouveau();
		let commentaires = [
			"Nota Bene ceci n'est pas un commentaire.",
			"Nota bene : ceci n'est pas un commentaire.",
			"Nota ene: ceci n'est pas un commentaire.",
			"notaBene: ceci n'est pas  un commentaire.",
			"NotBene :ceci n'est pas un commentaire.",
			"NotaBenececi n'est pas un commentaire.",
			"notabene:ceci n'est pas un commentaire.",
			"NNotaBene:ceci n'est pas un commentaire."
		];
		for commentaire in commentaires {
			let Err(erreur) = pendragon.compile(commentaire.into()) else {
				panic!("Ne devrait pas pouvoir compiler un commentaire invalide '{}'", commentaire);
			};
			let ErreurPendragon::CommandeInconnue(_) = erreur.raison() else {
				panic!("Erreur inattendue de compilation du commentaire '{}' : {}", commentaire, erreur.raison());
			};
		}
	}
	
	#[test]
	fn ponctuation_valide() {
		panic!("todo");
	}
	
	#[test]
	fn ponctuation_invalide() {
		panic!("todo");
	}
	
	#[test]
	fn commande_valide() {
		panic!("todo");
	}
	
	#[test]
	fn commande_invalide() {
		panic!("todo");
	}
}