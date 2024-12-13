use std::fmt;
use super::*;

pub struct ErreurCompilation {
	index_ligne: usize,
	erreur: ErreurPendragon,
}

impl ErreurCompilation {
	pub fn nouvelle(index_ligne: usize, erreur: ErreurPendragon) -> Self {
		Self {
			index_ligne,
			erreur,
		}
	}
	pub fn raison(&self) -> ErreurPendragon {
		self.erreur.clone()
	}
}


impl fmt::Display for ErreurCompilation {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {//'
		write!(f, "Erreur ligne {} : {}", self.index_ligne + 1, self.erreur)
	}
}

#[derive(PartialEq, Debug, Clone)]
pub enum ErreurPendragon {
	CommandeInconnue(String),
	ManqueArgument,
	MauvaisArgument(String),
	ManquePonctuation,
	
	NombreInvalide(String),
	CalculEntier(String),
	OrdreCalculEntier(String, String, String),
	
	TexteInvalide(String),
	
	BooleenInvalide(String),
	ComparaisonInvalide(String),
	CalculBooleen(String),
	OrdreCalculBooleen(String, String, String),
	
	VariableInconnue(String),
	MauvaisType(String, String, String),
	
	Lecture(String),
}

impl fmt::Display for ErreurPendragon {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {//'
		match self {
			Self::CommandeInconnue(commande) => write!(f, "La commande \"{}\" est inconnue.", commande),
			Self::ManqueArgument => write!(f, "Il manque un argument."),
			Self::MauvaisArgument(message) => write!(f, "La commande a reçu un mauvais argument, {}.", message),
			Self::ManquePonctuation => write!(f, "Il manque la ponctuation de la phrase."),
			
			Self::NombreInvalide(nombre) => write!(f, "Le nombre \"{}\" est mal orthographié.", nombre),
			Self::CalculEntier(raison) => write!(f, "Calcul entier échoué, {}.", raison),
			Self::OrdreCalculEntier(manque, precedent, suivant) => write!(f, "Calcul entier échoué, il manque un {} entre '{}' et '{}'.", manque, precedent, suivant),
			
			Self::TexteInvalide(raison) => write!(f, "Le texte est invalide, {}.", raison),
			
			Self::BooleenInvalide(booleen) => write!(f, "Le booleen \"{}\" est invalide.", booleen),
			Self::CalculBooleen(raison) => write!(f, "Calcul booleen échoué, {}.", raison),
			Self::ComparaisonInvalide(raison) => write!(f, "La comparaison est invalide, {}.", raison),
			Self::OrdreCalculBooleen(manque, precedent, suivant) => write!(f, "Calcul boolen échoué, il manque un {} entre '{}' et '{}'.", manque, precedent, suivant),
			
			Self::VariableInconnue(nom) => write!(f, "La variable \"{}\" est inconnue.", nom),
			Self::MauvaisType(nom, type_variable, type_attendu) => write!(f, "La {} est du mauvais type ({}), attendais {}.", nom, type_variable, type_attendu),
			
			Self::Lecture(raison) => write!(f, "Lecture d'entrées utilisateur impossible : {}.", raison),
		}
	}
}

impl fmt::Display for Commande {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {//'
		match self {
			Self::Definis(nom, type_element) => write!(f, "Definis {} comme {}.", nom, type_element.nom()),
			Self::Demande(nom) => write!(f, "Demande {}.", nom),
			Self::Modifie(nom, expression) => write!(f, "Modifie {} avec {:?}.", nom, expression),
			Self::Affiche(expression) => write!(f, "Affiche {:?}.", expression),
		}
	}
}

impl fmt::Display for Element {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {//'
		match self {
			Self::Entier(nombre) => write!(f, "{}", nombre::nombre_comme_texte(*nombre)),
			Self::Texte(texte) => write!(f, "\"{}\"", texte),
			Self::Booleen(booleen) => write!(f, "{}", booleen::booleen_comme_texte(*booleen)),
			Self::Variable(nom, type_variable) => write!(f, "{}:{}", nom, type_variable.nom()),
			Self::Operateur(operateur) => write!(f, "{}", operateur),
			Self::Comparaison(comparaison) => write!(f, "{}.", comparaison),
		}
	}
}

impl fmt::Display for Comparaison {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {//'
		let mut texte_membre_a: String = String::new(); 
		for element in &self.membre_a {
			texte_membre_a += &format!("{} ", element);
		}
		let mut texte_membre_b: String = String::new(); 
		for element in &self.membre_b {
			texte_membre_b += &format!(" {}", element);
		}
		write!(f, "({}{:?}{})", 
			texte_membre_a,
			self.type_comparaison,
			texte_membre_b,
		)
	}
}

impl fmt::Display for TypeComparaison {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {//'
		match self {
			Self::Egal => write!(f, "égal à"),
			Self::Different => write!(f, "différent de"),
			Self::SuperieurEgal => write!(f, "supérieur ou égal à"),
			Self::InferieurEgal => write!(f, "inférieur ou égal à"),
			Self::Superieur => write!(f, "supérieur à"),
			Self::Inferieur => write!(f, "inférieur à"),
		}
	}
}

impl fmt::Display for Operateur {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {//'
		match self {
			Self::Ou => write!(f, "ou"),
			Self::Et => write!(f, "et"),
			Self::Non => write!(f, "non"),
			Self::ParentheseBooleen => write!(f, "["),
			Self::Puis => write!(f, ";"),
			Self::Plus => write!(f, "+"),
			Self::Moins => write!(f, "-"),
			Self::Fois => write!(f, "*"),
			Self::Divise => write!(f, "/"),
			Self::ParentheseEntier => write!(f, "("),
		}
	}
}

impl fmt::Display for Programme {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {//'
		let mut texte: String = format!("variables : {:?}", self.variables);
		for (index, commande) in self.commandes.iter().enumerate() {
			texte += &format!("\n#{:2}-{}", index+1, commande);
		}
		write!(f, "{}", texte)
	}
}