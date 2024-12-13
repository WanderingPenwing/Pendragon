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
	
	pub fn compile(&mut self, contenu: String) -> Result<(), Vec<ErreurCompilation>> {
		let texte: Vec<&str> = contenu.split('\n').collect();
		let mut erreurs: Vec<ErreurCompilation> = vec![];
		for (index_ligne, ligne) in texte.iter().enumerate() {
			let ligne = ligne.trim();
			let phrases: Vec<&str> = ligne.split_inclusive(|c| c == ',' || c == '.').collect();
			let Some(derniere_phrase) = phrases.last() else {
				continue
			};
			if !derniere_phrase.ends_with('.') && !derniere_phrase.ends_with(',') {
				erreurs.push(ErreurCompilation::nouvelle(index_ligne, ligne.into(), ErreurPendragon::ManquePonctuation))
			}
			for phrase in phrases {
				if phrase.ends_with(".") {
					if phrase.replace(" ", "").starts_with("NotaBene:") {
						continue
					}
					match self.compile_commande(&phrase[..phrase.len() - 1]) {
						Ok(commande) => self.programme.ajoute_commande(commande),
						Err(raison) => erreurs.push(ErreurCompilation::nouvelle(index_ligne, ligne.into(), raison)),
					}
					continue;
				}
				println!("todo : {}", phrase);
			}
		}
		if erreurs.len() > 0 {
			return Err(erreurs)
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
				Ok(_) => assert_eq!(pendragon.programme.contenu.len(), 0, "Le commentaire '{}' ne devrait pas générer de commande", commentaire),
				Err(erreurs) => {
					let affichage_erreurs = erreurs.iter().map(|item| format!("{}", item.raison())).collect::<Vec<_>>().join("\n\n");
					panic!("Erreur de compilation du commentaire '{}' : \n{}", commentaire, affichage_erreurs)
				}
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
			let Err(erreurs) = pendragon.compile(commentaire.into()) else {
				panic!("Ne devrait pas pouvoir compiler un commentaire invalide '{}'", commentaire);
			};
			if erreurs.len() > 1 {
				let affichage_erreurs = erreurs.iter().map(|item| format!("{}", item.raison())).collect::<Vec<_>>().join("\n\n");
				panic!("Plus d'erreurs que prévu pour le commentaire '{}' : {:?}", commentaire, affichage_erreurs)
			}
			let ErreurPendragon::CommandeInconnue(_) = erreurs[0].raison() else {
				panic!("Erreur inattendue de compilation du commentaire '{}' : {}", commentaire, erreurs[0].raison());
			};
		}
	}
	
	#[test]
	fn ponctuation_valide() {
		let mut pendragon = Pendragon::nouveau();
		let texte = "aah.\noooh.uuuh,\nna,\nbududu.bababa.\naaaaaaaaaaa,sssssss,";
		let Err(erreurs) = pendragon.compile(texte.into()) else {
			panic!("Il devrait y avoir des erreurs");
		};
		for erreur in erreurs {
			if let ErreurPendragon::ManquePonctuation = erreur.raison() {
				panic!("Erreur : manque ponctuation");
			}
		}
	}
	
	#[test]
	fn ponctuation_invalide() {
		let mut pendragon = Pendragon::nouveau();
		let textes = [
			"Aaaaa",
			"aaaa.\nooooo\n",
			"aaaa Définis."
		];
		for texte in textes {
			let Err(erreurs) = pendragon.compile(texte.into()) else {
				panic!("Ne devrait pas pouvoir compiler un texte invalide '{}'", texte);
			};
			let mut manque_ponctuation = false;
			for erreur in erreurs {
				if let ErreurPendragon::ManquePonctuation = erreur.raison() {
					manque_ponctuation = true;
				}
			}
			if !manque_ponctuation {
				panic!("Il devrait y avoir une erreur de ponctuation dans le texte '{}'", texte);
			}
		}
	}
	
	#[test]
	fn commande_valide() {
		let mut pendragon = Pendragon::nouveau();
		let texte = "Affiche.\nDemande.\nModifie.Définis.";
		let Err(erreurs) = pendragon.compile(texte.into()) else {
			panic!("Il devrait y avoir des erreurs");
		};
		for erreur in erreurs {
			if let ErreurPendragon::CommandeInconnue(_) = erreur.raison() {
				panic!("Erreur : commande invalide");
			}
		}
	}
	
	#[test]
	fn commande_invalide() {
		let mut pendragon = Pendragon::nouveau();
		let textes = [
			"Definis a.",
			"définis b.",
			"modifie c.",
			"affiche d.",
			"aaaa e."
		];
		for texte in textes {
			let Err(erreurs) = pendragon.compile(texte.into()) else {
				panic!("Ne devrait pas pouvoir compiler un texte invalide '{}'", texte);
			};
			let mut commande_inconnue = false;
			for erreur in &erreurs {
				if let ErreurPendragon::CommandeInconnue(_) = erreur.raison() {
					commande_inconnue = true;
				}
			}
			if !commande_inconnue {
				let affichage_erreurs = erreurs.iter().map(|item| format!("{}", item.raison())).collect::<Vec<_>>().join("\n\n");
				panic!("La commande devrait être inconnue '{}', erreurs : \n{}", texte, affichage_erreurs);
			}
		}
	}
}