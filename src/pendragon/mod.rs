use std::collections::HashMap;
use std::time::Instant;

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
	
	pub fn compile(&mut self, contenu: String) -> Result<(), ErreurPendragon> {
		println!();
		let debut = Instant::now();
		let contenu_propre = contenu.replace("\n", " ");
		let mut texte: Vec<&str> = contenu_propre.split('.').collect();
		let reste = texte.pop();
		if reste != Some("") {
			eprintln!("Erreur Compilation, phrase {} : Il manque un point.", texte.len() + 1);
			return Err(ErreurPendragon::ManquePoint)
		}
		for (index_phrase, phrase) in texte.iter().enumerate() {
			let phrase = phrase.trim();
			match self.compile_phrase(phrase) {
				Ok(commande) => {self.programme.ajoute_commande(commande)},
				Err(raison) => {
					eprintln!("Erreur phrase {} : {}", index_phrase + 1, raison);
					return Err(raison)
				}
			}
		}
		println!("# Compilation Ok. ({:.2?})\n", debut.elapsed());
		Ok(())
	}
	
	fn compile_phrase(&mut self, phrase: &str) -> Result<Commande, ErreurPendragon> {
		let phrase = phrase.trim();
		let parties: Vec<&str> = phrase.splitn(2, ' ').collect();
		if parties.len() == 1 {
			return Err(ErreurPendragon::ManqueArgument)
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
