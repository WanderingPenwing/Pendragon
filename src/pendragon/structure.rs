use std::collections::HashMap;

use super::*;

#[derive(Debug)]
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

	pub fn ajoute_bloc(&mut self, bloc: Bloc) {
		self.contenu.push(Phrase::Bloc(bloc));
	}

	pub fn ajoute_variable(
		&mut self,
		nom: String,
		type_variable: TypeElement,
	) -> Result<(), ErreurPendragon> {
		let Err(raison) = self.variable(&nom) else {
			return Err(ErreurPendragon::MauvaisArgument(format!(
				"la variable \"{}\" existe déjà",
				nom
			)));
		};
		let ErreurPendragon::VariableInconnue(_) = raison else {
			return Err(raison);
		};
		self.variables.insert(nom, type_variable);
		Ok(())
	}

	pub fn variable(&self, nom: &str) -> Result<TypeElement, ErreurPendragon> {
		let Some(first_char) = nom.chars().next() else {
			return Err(ErreurPendragon::MauvaisArgument(
				"il n'y a pas de variable".to_string(),
			));
		};
		if !first_char.is_uppercase() {
			return Err(ErreurPendragon::MauvaisArgument(
				"il manque une majuscule à la variable".to_string(),
			));
		}
		if !self.variables.contains_key(nom) {
			return Err(ErreurPendragon::VariableInconnue(nom.into()));
		}
		Ok(self.variables[nom].clone())
	}

	pub fn variable_est_de_type(
		&self,
		nom: &str,
		type_element: TypeElement,
	) -> Result<(), ErreurPendragon> {
		let type_variable = self.variable(nom)?;
		if type_variable != type_element {
			return Err(ErreurPendragon::MauvaisType(
				nom.into(),
				type_variable.nom(),
				type_element.nom(),
			));
		}
		Ok(())
	}
}

#[derive(Debug)]
pub struct Bloc {
	pub condition: Vec<Element>,
	pub repete: bool,
	pub contenu: Vec<Phrase>,
	pub variables_internes: HashMap<String, TypeElement>,
	pub variables_externes: Vec<String>,
}

impl Bloc {
	pub fn nouveau(condition: Vec<Element>, repete: bool) -> Self {
		let mut variables_externes: Vec<String> = vec![];
		for element in condition.iter() {
			let variables = element.recupere_variables();
			for nom_element in variables.iter() {
				if !variables_externes.contains(nom_element) {
					variables_externes.push(nom_element.clone());
				}
			}
		}
		Self {
			condition,
			repete,
			contenu: vec![],
			variables_internes: HashMap::new(),
			variables_externes,
		}
	}

	pub fn ajoute_commande(&mut self, commande: Commande) {
		match commande.clone() {
			Commande::Definis(nom, type_element) => {
				self.variables_internes.insert(nom.to_string(), type_element);
			}
			Commande::Demande(nom) => {
				if !self.variables_internes.contains_key(&nom) && !self.variables_externes.contains(&nom) {
					self.variables_externes.push(nom);
				}
			}
			Commande::Modifie(nom, expression) => {
				if !self.variables_internes.contains_key(&nom) && !self.variables_externes.contains(&nom) {
					self.variables_externes.push(nom);
				}
				for element in expression.iter() {
					let variables = element.recupere_variables();
					for nom_element in variables.iter() {
						if !self.variables_internes.contains_key(nom_element) && !self.variables_externes.contains(nom_element) {
							self.variables_externes.push(nom_element.clone());
						}
					}
				}
			}
			Commande::Affiche(expression) => {
				for element in expression.iter() {
					let variables = element.recupere_variables();
					for nom_element in variables.iter() {
						if !self.variables_internes.contains_key(nom_element) && !self.variables_externes.contains(nom_element) {
							self.variables_externes.push(nom_element.clone());
						}
					}
				}
			}
		}
		self.contenu.push(Phrase::Commande(commande)); //check variable
	}

	pub fn ajoute_bloc(&mut self, bloc: Bloc) {
		for variable in bloc.variables_externes.iter() {
			if !self.variables_internes.contains_key(variable) && !self.variables_externes.contains(variable) {
				self.variables_externes.push(variable.clone());
			}
		}
		self.contenu.push(Phrase::Bloc(bloc));
	}
}

#[derive(Debug)]
pub enum Phrase {
	Bloc(Bloc),
	Commande(Commande),
}

#[derive(Debug, Clone)]
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
			autre => Err(ErreurPendragon::MauvaisArgument(format!(
				"type de variable \"{}\" inconnu",
				autre
			))),
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
	
	pub fn type_ir(&self) -> String {
		match self{
			Self::Entier => "i64".into(),
			Self::Texte => "i8*".into(),
			Self::Booleen => "i1".into(),
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
			Self::Plus | Self::Moins | Self::Fois | Self::Divise | Self::ParentheseEntier => {
				TypeElement::Entier
			}
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

	pub fn ajoute_type(
		&mut self,
		type_comparaison: TypeComparaison,
	) -> Result<(), ErreurPendragon> {
		let Some(element) = self.membre_a.first() else {
			return Err(ErreurPendragon::ComparaisonInvalide(
				"il n'y a pas de premier membre".into(),
			));
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
		Err(ErreurPendragon::ComparaisonInvalide(format!(
			"voulait comparer {} avec {}",
			element.type_element().nom(),
			type_comparaison
		)))
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
