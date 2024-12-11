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
}


impl fmt::Display for ErreurCompilation {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {//'
		write!(f, "Erreur ligne {} : {}", self.index_ligne + 1, self.erreur)
	}
}

pub enum ErreurPendragon {
	CommandeInconnue(String),
	ManqueArgument,
	NombreInvalide(String),
	BooleenInvalide(String),
	TexteInvalide(String),
	ComparaisonInvalide(String),
	MauvaisArgument(String),
	VariableInconnue(String),
	MauvaisType(String, String, String),
	ManquePonctuation,
	Lecture(String),
	CalculBooleen(String),
	CalculEntier(String),
}

impl fmt::Display for ErreurPendragon {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {//'
		match self {
			Self::CommandeInconnue(commande) => write!(f, "La commande \"{}\" est inconnue.", commande),
			Self::ManqueArgument => write!(f, "Il manque un argument."),
			Self::NombreInvalide(nombre) => write!(f, "Le nombre \"{}\" est mal orthographié.", nombre),
			Self::TexteInvalide(raison) => write!(f, "Le texte est invalide, {}.", raison),
			Self::BooleenInvalide(booleen) => write!(f, "Le booleen \"{}\" est invalide.", booleen),
			Self::ComparaisonInvalide(raison) => write!(f, "La comparaison est invalide, {}.", raison),
			Self::MauvaisArgument(message) => write!(f, "La commande a reçu un mauvais argument, {}.", message),
			Self::VariableInconnue(nom) => write!(f, "La variable \"{}\" est inconnue.", nom),
			Self::MauvaisType(nom, type_variable, type_attendu) => write!(f, "La {} est du mauvais type ({}), attendais {}.", nom, type_variable, type_attendu),
			Self::ManquePonctuation => write!(f, "Il manque la ponctuation de la phrase."),
			Self::Lecture(raison) => write!(f, "Lecture d'entrées utilisateur impossible : {}.", raison),
			Self::CalculBooleen(raison) => write!(f, "Calcul booleen échoué, {}.", raison),
			Self::CalculEntier(raison) => write!(f, "Calcul entier échoué, {}.", raison),
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

impl fmt::Display for Programme {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {//'
		let mut texte: String = format!("variables : {:?}", self.variables);
		for (index, commande) in self.commandes.iter().enumerate() {
			texte += &format!("\n#{:2}-{}", index+1, commande);
		}
		write!(f, "{}", texte)
	}
}