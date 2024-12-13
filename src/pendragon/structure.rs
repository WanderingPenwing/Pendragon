use std::io;
use std::collections::HashMap;

use super::*;

pub struct Programme {
	pub variables: HashMap<String, TypeElement>,
	pub contenu: Vec<Phrase>,
}

impl Programme {
	pub fn nouveau() -> Self {
		Self {
			variables: HashMap::new(),
			contenu: vec![],
		}
	}
	
	pub fn ajoute_commande(&mut self, commande: Commande) {
		self.contenu.push(Phrase::Commande(commande));
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
		for phrase in &self.contenu {
			let Phrase::Commande(commande) = phrase else {
				println!("doit executer bloc");
				continue;
			};
			commande.execute(&mut variables_globales)?;			
		}
		Ok(())
	}
}

pub struct Bloc {
	condition: Vec<Element>,
	contenu: Vec<Phrase>,
}

pub enum Phrase {
	Bloc(Bloc),
	Commande(Commande),
}

pub enum Commande {
	Definis(String, TypeElement),
	Demande(String),
	Modifie(String, Vec<Element>),
	Affiche(Vec<Element>),
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
			Self::Booleen(_) | Self::Comparaison(_) => TypeElement::Booleen,
			Self::Variable(_, type_element) => type_element.clone(),
			Self::Operateur(operateur) => operateur.type_element(),
		}
	}
	
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

#[derive(Clone, Debug, PartialEq)]
pub enum Operateur {
	Ou,
	Et,
	Non,
	ParentheseBooleen,
	Puis,
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
			Self::Puis => TypeElement::Texte,
			Self::Plus | Self::Moins | Self::Fois | Self::Divise | Self::ParentheseEntier => TypeElement::Entier,
		}
	}		
}

#[derive(Clone, Debug, PartialEq)]
pub struct Comparaison {
	pub type_comparaison: Option<TypeComparaison>,
	pub membre_a: Vec<Element>,
	pub membre_b: Vec<Element>,
}

impl Comparaison {
	pub fn nouvelle() -> Self {
		Self {
			type_comparaison: None,
			membre_a: vec![],
			membre_b: vec![],
		}
	}
	
	pub fn ajoute_type(&mut self, type_comparaison: TypeComparaison) -> Result<(), ErreurPendragon> {
		let Some(element) = self.membre_a.first() else {
			return Err(ErreurPendragon::ComparaisonInvalide("il n'y a pas de premier membre".into()))
		};
		if let TypeComparaison::Egal = type_comparaison {
			self.type_comparaison = Some(type_comparaison);
			return Ok(());
		}
		if let TypeComparaison::Different = type_comparaison {
			self.type_comparaison = Some(type_comparaison);
			return Ok(());
		}
		if let TypeElement::Entier = element.type_element() {
			self.type_comparaison = Some(type_comparaison);
			return Ok(());
		}
		return Err(ErreurPendragon::ComparaisonInvalide(format!("voulait comparer {} avec {}", element.type_element().nom(), type_comparaison)))
	}
	
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

#[derive(Clone, Debug, PartialEq)]
pub enum TypeComparaison {
	Egal,
	Different,
	SuperieurEgal,
	InferieurEgal,
	Superieur,
	Inferieur,
}



