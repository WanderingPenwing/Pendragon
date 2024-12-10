use std::io;
use std::collections::HashMap;

use super::*;

pub struct Programme {
	pub variables: HashMap<String, TypeElement>,
	pub commandes: Vec<Commande>,
}

impl Programme {
	pub fn nouveau() -> Self {
		Self {
			variables: HashMap::new(),
			commandes: vec![],
		}
	}
	
	pub fn ajoute_commande(&mut self, commande: Commande) {
		self.commandes.push(commande);
	}
	
	pub fn ajoute_variable(&mut self, nom: String, type_variable: TypeElement) -> Result<(), ErreurPendragon> {
		let Err(raison) = self.variable(&nom) else {
			return Err(ErreurPendragon::MauvaisArgument(format!("la variable \"{}\" existe déjà", nom)))
		};
		let ErreurPendragon::VariableInconnue(_) = raison else {
			return Err(raison)
		};
		self.variables.insert(nom, type_variable);
		Ok(())
	}
	
	pub fn variable(&self, nom: &str) -> Result<TypeElement, ErreurPendragon> {
		let Some(first_char) = nom.chars().next() else {
			return Err(ErreurPendragon::MauvaisArgument("il n'y a pas de variable".to_string()))
		};
		if !first_char.is_uppercase() {
			return Err(ErreurPendragon::MauvaisArgument("il manque une majuscule à la variable".to_string()))
		}
		if !self.variables.contains_key(nom) {
			return Err(ErreurPendragon::VariableInconnue(nom.into()))
		}
		Ok(self.variables[nom].clone())
	}
	
	pub fn variable_est_de_type(&self, nom: &str, type_element: TypeElement) -> Result<(), ErreurPendragon> {
		let type_variable = self.variable(nom)?;
		if type_variable != type_element {
			return Err(ErreurPendragon::MauvaisType(nom.into(), type_variable.nom(), type_element.nom()))
		}
		Ok(())
	}
	
	pub fn execute(&self) -> Result<(), ErreurPendragon> {
		let mut variables_globales: HashMap<String, Element> = HashMap::new();
		for commande in &self.commandes {
			match commande {
				Commande::Definis(nom, type_element) => {
					variables_globales.insert(nom.to_string(), type_element.comme_element());
				}
				Commande::Demande(nom) => {
					let valeur = variables_globales[nom].type_element().demande_valeur(&nom)?;
					variables_globales.insert(nom.to_string(), valeur);
				}
				Commande::Modifie(nom, expression) => {
					let valeur = match variables_globales[nom].type_element() {
						TypeElement::Entier => Element::Entier(nombre::calcule_nombre(expression.clone(), &variables_globales)?),
						TypeElement::Texte => Element::Texte(texte::calcule_texte(expression.clone(), &variables_globales)?),
						TypeElement::Booleen => Element::Booleen(booleen::calcule_booleen(expression.clone(), &variables_globales)?),
					};
					variables_globales.insert(nom.to_string(), valeur);
				}
				Commande::Affiche(expression) => {
					println!("{}", texte::calcule_texte(expression.to_vec(), &variables_globales)?);
				}
			}			
		}
		Ok(())
	}
}

pub enum Commande {
	Definis(String, TypeElement),
	Demande(String),
	Modifie(String, Vec<Element>),
	Affiche(Vec<Element>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum TypeElement {
	Entier,
	Texte,
	Booleen,
}

impl TypeElement {
	pub fn nom(&self) -> String {
		match self {
			Self::Entier => "entier".into(),
			Self::Texte => "texte".into(),
			Self::Booleen => "booléen".into(),
		}
	}
	
	pub fn texte_comme_type(texte: &str) -> Result<Self, ErreurPendragon> {
		match texte.trim() {
			"entier" => Ok(TypeElement::Entier),
			"texte" => Ok(TypeElement::Texte),
			"booléen" => Ok(TypeElement::Booleen),
			autre => return Err(ErreurPendragon::MauvaisArgument(format!("type de variable \"{}\" inconnu", autre))),
		}
	}
	
	pub fn comme_element(&self) -> Element {
		match self {
			Self::Entier => Element::Entier(0),
			Self::Texte => Element::Texte("".into()),
			Self::Booleen => Element::Booleen(false),
		}
	}
	
	pub fn texte_comme_element(&self, texte: &str) -> Result<Element, ErreurPendragon> {
		match self {
			Self::Entier => nombre::texte_comme_nombre(texte),
			Self::Texte => Ok(Element::Texte(texte.into())),
			Self::Booleen => booleen::texte_comme_booleen(texte),
		}
	}
	
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

#[derive(PartialEq, Debug, Clone)]
pub enum Element {
	Entier(usize),
	Texte(String),
	Booleen(bool),
	Variable(String, TypeElement),
	Operateur(Operateur),
	Comparaison(Comparaison),
}

impl Element {	
	pub fn type_element(&self) -> TypeElement {
		match self {
			Self::Entier(_) => TypeElement::Entier,
			Self::Texte(_) => TypeElement::Texte,
			Self::Booleen(_) || Self::Comparaison(_) => TypeElement::Booleen,
			Self::Variable(_, type_element) => type_element.clone(),
			Self::Operateur(operateur) => operateur.type_element(),
		}
	}
}

#[derive(Clone, Debug, PartialEq)]
pub enum Operateur {
	Ou,
	Et,
	Non,
	ParentheseBooleen,
	Virgule,
	Plus,
	Moins,
	Fois,
	Divise,
	ParentheseEntier,
}

impl Operateur {
	pub fn type_element(&self) -> TypeElement {
		match self {
			Self::Ou | Self::Et | Self::Non | Self::ParentheseBooleen => TypeElement::Booleen,
			Self::Virgule => TypeElement::Texte,
			Self::Plus | Self::Moins | Self::Fois | Self::Divise | Self::ParentheseEntier => TypeElement::Entier,
		}
	}		
}

pub enum Comparaison {
	Egal(Element, Element),
	Different(Element, Element),
	Superieur(Element, Element),
	Inferieur(Element, Element),
	SuperieurEgal(Element, Element),
	InferieurEgal(Element, Element)
}





