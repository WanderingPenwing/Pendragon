use std::fmt;

pub enum Commande {
	Definis(String, TypeElement),
	Demande(String),
	Modifie(String, Expression),
	Affiche(Expression),
}

#[derive(PartialEq, Debug, Clone)]
pub struct Expression {
	type_expression: TypeElement,
	contenu: Vec<Element>
}

impl Expression {
	fn nouvelle(type_expression: TypeElement) -> Self {
		Self {
			type_expression,
			contenu: vec![]
		}
	}
	fn ajoute(&mut self, element: Element) -> Result<(), ErreurSophie> {
		let type_element = element.type_element();
		if self.type_expression != type_element {
			return Err(ErreurSophie::MauvaisType("inconnue".into(), type_element.nom(), self.type_expression.nom()))
		}
		self.contenu.push(element);
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
}

#[derive(PartialEq, Debug, Clone)]
pub enum Element {
	Entier(usize),
	Texte(String),
	Booleen(bool),
	Variable(String, TypeElement),
	Operateur(String, TypeElement),
	Expression(Expression, TypeElement),
}

impl Element {	
	pub fn type_element(&self) -> TypeElement {
		match self {
			Self::Entier(_) => TypeElement::Entier,
			Self::Texte(_) => TypeElement::Texte,
			Self::Booleen(_) => TypeElement::Booleen,
			Self::Variable(_, type_element) => type_element.clone(),
			Self::Operateur(_, type_element) => type_element.clone(),
			Self::Expression(_, type_element) => type_element.clone(),
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