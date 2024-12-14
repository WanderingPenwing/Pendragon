use std::io;
use std::collections::HashMap;
use crate::pendragon::structure::*;
use crate::display::ErreurPendragon;

pub mod booleen;
pub mod texte;
pub mod nombre;

impl Programme {
	pub fn execute(&self) -> Result<(), ErreurPendragon> {
		let mut variables_globales: HashMap<String, Element> = HashMap::new();
		for phrase in &self.contenu {
			match phrase {
				Phrase::Commande(commande) => commande.execute(&mut variables_globales)?,
				Phrase::Bloc(bloc) => bloc.execute(&mut variables_globales)?,
			}			
		}
		Ok(())
	}
}

impl Bloc {
	pub fn execute(&self, variables: &mut HashMap<String, Element>) -> Result<(), ErreurPendragon> {
		if booleen::calcule_booleen(self.condition.clone(), variables)? {
			for phrase in &self.contenu {
				match phrase {
					Phrase::Commande(commande) => commande.execute(variables)?,
					Phrase::Bloc(bloc) => bloc.execute(variables)?,
				}			
			}
			while booleen::calcule_booleen(self.condition.clone(), variables)? && self.repete {
				for phrase in &self.contenu {
					match phrase {
						Phrase::Commande(commande) => commande.execute(variables)?,
						Phrase::Bloc(bloc) => bloc.execute(variables)?,
					}			
				}
			}
		} //else if self.contenu_sinon.len() > 0 {
//			for phrase in &self.contenu_sinon {
//				match phrase {
//					Phrase::Commande(commande) => commande.execute(variables)?,
//					Phrase::Bloc(bloc) => bloc.execute(variables)?,
//				}			
//			}
//		}
		Ok(())
	}
}

impl Commande {
	fn execute(&self, variables: &mut HashMap<String, Element>) -> Result<(), ErreurPendragon> {
		match self {
			Commande::Definis(nom, type_element) => {
				variables.insert(nom.to_string(), type_element.comme_element());
			}
			Commande::Demande(nom) => {
				let valeur = variables[nom].type_element().demande_valeur(&nom)?;
				variables.insert(nom.to_string(), valeur);
			}
			Commande::Modifie(nom, expression) => {
				let valeur = match variables[nom].type_element() {
					TypeElement::Entier => Element::Entier(nombre::calcule_nombre(expression.clone(), variables)?),
					TypeElement::Texte => Element::Texte(texte::calcule_texte(expression.clone(), variables)?),
					TypeElement::Booleen => Element::Booleen(booleen::calcule_booleen(expression.clone(), variables)?),
				};
				variables.insert(nom.to_string(), valeur);
			}
			Commande::Affiche(expression) => {
				println!("{}", texte::calcule_texte(expression.to_vec(), variables)?);
			}
		}
		Ok(())
	}
}

impl TypeElement {
	pub fn demande_valeur(&self, nom: &str) -> Result<Element, ErreurPendragon> {
		loop {
			println!("Quelle valeur pour {} ({}) ?", nom, self.nom());
			
			let mut reponse = String::new();
			if let Err(raison) = io::stdin().read_line(&mut reponse) {
				return Err(ErreurPendragon::Lecture(format!("{}", raison)));
			}
	
			match self.texte_comme_element(reponse.trim()) {
				Ok(element) => return Ok(element),
				Err(raison) => eprintln!("Erreur : {}", raison),
			}
		}
	}
}


impl Element {
	pub fn compare(&self, element: Element, comparaison: TypeComparaison) -> Result<bool, ErreurPendragon> {
		if let TypeComparaison::Egal = comparaison {
			return Ok(*self == element)
		}
		if let TypeComparaison::Different = comparaison {
			return Ok(*self != element)
		}
		let Self::Entier(nombre_a) = self else {
			return Err(ErreurPendragon::ComparaisonInvalide(format!("comparaison numérique avec {}", self)))
		};
		let Self::Entier(nombre_b) = element else {
			return Err(ErreurPendragon::ComparaisonInvalide(format!("comparaison numérique avec {}", element)))
		};
		match comparaison {
			TypeComparaison::SuperieurEgal => Ok(*nombre_a >= nombre_b),
			TypeComparaison::InferieurEgal => Ok(*nombre_a <= nombre_b),
			TypeComparaison::Superieur => Ok(*nombre_a > nombre_b),
			TypeComparaison::Inferieur => Ok(*nombre_a < nombre_b),
			_ => Err(ErreurPendragon::ComparaisonInvalide("problème de logique".into())),
		}
	}
}

impl Comparaison {
	pub fn calcule(&self, variables: &HashMap<String, Element>) -> Result<bool, ErreurPendragon> {
		let Some(ref comparaison) = self.type_comparaison else {
			return Err(ErreurPendragon::ComparaisonInvalide("la comparaison n'a pas de type".into()))
		};
		let Some(element) = self.membre_a.first() else {
			return Err(ErreurPendragon::ComparaisonInvalide("il n'y a pas de premier membre".into()))
		};
		let (membre_a, membre_b) = match element.type_element() {
			TypeElement::Entier => {
				let membre_a = Element::Entier(nombre::calcule_nombre(self.membre_a.clone(), variables)?);
				let membre_b = Element::Entier(nombre::calcule_nombre(self.membre_b.clone(), variables)?);
				(membre_a, membre_b)
			}
			TypeElement::Texte => {
				let membre_a = Element::Texte(texte::calcule_texte(self.membre_a.clone(), variables)?);
				let membre_b = Element::Texte(texte::calcule_texte(self.membre_b.clone(), variables)?);
				(membre_a, membre_b)
			}
			TypeElement::Booleen => {
				let membre_a = Element::Booleen(booleen::calcule_booleen(self.membre_a.clone(), variables)?);
				let membre_b = Element::Booleen(booleen::calcule_booleen(self.membre_b.clone(), variables)?);
				(membre_a, membre_b)
			}
		};
		Ok(membre_a.compare(membre_b, comparaison.clone())?)
	}
}