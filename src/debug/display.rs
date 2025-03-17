use crate::pendragon::structure::*;
use crate::sophie;
use std::fmt;
use std::time::Duration;

pub const TEXTE_ROUGE: &str = "\x1b[31m";
pub const TEXTE_VERT: &str = "\x1b[32m";
//pub const TEXTE_JAUNE: &str = "\x1b[33m";
pub const TEXTE_BLEU: &str = "\x1b[34m";
pub const TEXTE_GRIS: &str = "\x1b[37m";
pub const TEXTE_NORMAL: &str = "\x1b[0m";

pub fn message_debut(action: &str, chemin_de_fichier: &str) {
    println!(
        "\n- {} de {}'{}'{}...", action,
        TEXTE_VERT, chemin_de_fichier, TEXTE_NORMAL
    );
}

pub fn message_echec(action: &str) {
    eprintln!(
        "\n{}x Échec de {}.{}",
        TEXTE_ROUGE, action, TEXTE_NORMAL
    );
}

pub fn message_ok(action: &str, temps: Duration) {
    println!(
        "{}✓ {} Ok.{} ({:.2?}){}",
        TEXTE_VERT, action, TEXTE_GRIS, temps, TEXTE_NORMAL
    );
}

pub enum ErreurMorgan {
	ErreurSysteme(String),
	MauvaisArgument(String),
}
impl fmt::Display for ErreurMorgan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //'
        match self {
            Self::ErreurSysteme(raison) => {
                write!(f, "{}Erreur système :{}{}",  TEXTE_ROUGE, TEXTE_NORMAL, raison)
            },
            Self::MauvaisArgument(argument) => {
            	write!(f, "{}Erreur :{} La commande a reçu un mauvais argument, {}",  TEXTE_ROUGE, TEXTE_NORMAL, argument)
            }
        }
    }
}


pub struct ErreurCompilation {
    index_ligne: usize,
    ligne: String,
    erreur: ErreurPendragon,
}

impl ErreurCompilation {
    pub fn nouvelle(index_ligne: usize, ligne: String, erreur: ErreurPendragon) -> Self {
        Self {
            index_ligne,
            ligne,
            erreur,
        }
    }

    #[cfg(test)]
    pub fn raison(&self) -> ErreurPendragon {
        self.erreur.clone()
    }
}

impl fmt::Display for ErreurCompilation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //'
        write!(
            f,
            "{}Erreur :{} {}\n{}ligne {} : {}{}",
            TEXTE_ROUGE,
            TEXTE_NORMAL,
            self.erreur,
            TEXTE_GRIS,
            self.index_ligne + 1,
            self.ligne,
            TEXTE_NORMAL
        )
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum ErreurPendragon {
    CommandeInconnue(String),
    BlocInconnu(String),
    ManqueArgument,
    MauvaisArgument(String),
    MauvaiseIndentation(String),
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //'
        match self {
            Self::CommandeInconnue(commande) => {
                write!(f, "La commande \"{}\" est inconnue.", commande)
            }
            Self::BlocInconnu(bloc) => write!(f, "Le bloc \"{}\" est inconnu.", bloc),
            Self::ManqueArgument => write!(f, "Il manque un argument."),
            Self::MauvaisArgument(message) => {
                write!(f, "La commande a reçu un mauvais argument, {}.", message)
            }
            Self::MauvaiseIndentation(message) => {
                write!(f, "L'indentation est mauvaise, {}.", message)
            }
            Self::ManquePonctuation => write!(f, "Il manque la ponctuation de la phrase."),

            Self::NombreInvalide(nombre) => {
                write!(f, "Le nombre \"{}\" est mal orthographié.", nombre)
            }
            Self::CalculEntier(raison) => write!(f, "Calcul entier échoué, {}.", raison),
            Self::OrdreCalculEntier(manque, precedent, suivant) => write!(
                f,
                "Calcul entier échoué, il manque un {} entre '{}' et '{}'.",
                manque, precedent, suivant
            ),

            Self::TexteInvalide(raison) => write!(f, "Le texte est invalide, {}.", raison),

            Self::BooleenInvalide(booleen) => write!(f, "Le booleen \"{}\" est invalide.", booleen),
            Self::CalculBooleen(raison) => write!(f, "Calcul booleen échoué, {}.", raison),
            Self::ComparaisonInvalide(raison) => {
                write!(f, "La comparaison est invalide, {}.", raison)
            }
            Self::OrdreCalculBooleen(manque, precedent, suivant) => write!(
                f,
                "Calcul boolen échoué, il manque un {} entre '{}' et '{}'.",
                manque, precedent, suivant
            ),

            Self::VariableInconnue(nom) => write!(f, "La variable \"{}\" est inconnue.", nom),
            Self::MauvaisType(nom, type_variable, type_attendu) => write!(
                f,
                "La {} est du mauvais type ({}), attendais {}.",
                nom, type_variable, type_attendu
            ),

            Self::Lecture(raison) => {
                write!(f, "Lecture d'entrées utilisateur impossible : {}.", raison)
            }
        }
    }
}

impl fmt::Display for Commande {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //'
        match self {
            Self::Definis(nom, type_element) => write!(
                f,
                "{}{}{}:{}",
                TEXTE_VERT,
                nom,
                TEXTE_NORMAL,
                type_element.nom()
            ),
            Self::Demande(nom) => write!(f, "{}{}{}?", TEXTE_VERT, nom, TEXTE_NORMAL),
            Self::Modifie(nom, expression) => write!(
                f,
                "{}{}{}={}[{}{}{}]{}",
                TEXTE_VERT,
                nom,
                TEXTE_NORMAL,
                TEXTE_ROUGE,
                TEXTE_NORMAL,
                liste_element(expression),
                TEXTE_ROUGE,
                TEXTE_NORMAL
            ),
            Self::Affiche(expression) => write!(
                f,
                "#{}[{}{}{}]{}",
                TEXTE_ROUGE,
                TEXTE_NORMAL,
                liste_element(expression),
                TEXTE_ROUGE,
                TEXTE_NORMAL
            ),
        }
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //'
        match self {
            Self::Entier(nombre) => write!(f, "{}", sophie::nombre::nombre_comme_texte(*nombre)),
            Self::Texte(texte) => write!(f, "\"{}\"", texte),
            Self::Booleen(booleen) => {
                write!(f, "{}", sophie::booleen::booleen_comme_texte(*booleen))
            }
            Self::Variable(nom, type_variable) => write!(
                f,
                "{}{}{}:{}",
                TEXTE_VERT,
                nom,
                TEXTE_NORMAL,
                type_variable.nom()
            ),
            Self::Operateur(operateur) => write!(f, "{}", operateur),
            Self::Comparaison(comparaison) => write!(f, "{}", comparaison),
        }
    }
}

impl fmt::Display for Comparaison {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //'
        let mut texte_membre_a: String = String::new();
        for element in &self.membre_a {
            texte_membre_a += &format!("{} ", element);
        }
        let mut texte_membre_b: String = String::new();
        for element in &self.membre_b {
            texte_membre_b += &format!(" {}", element);
        }
        let comparaison = if let Some(type_comparaison) = &self.type_comparaison {
            format!("{}", type_comparaison)
        } else {
            "?".to_string()
        };
        write!(f, "({}{}{})", texte_membre_a, comparaison, texte_membre_b,)
    }
}

impl fmt::Display for TypeComparaison {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //'
        match self {
            Self::Egal => write!(f, "=="),
            Self::Different => write!(f, "!="),
            Self::SuperieurEgal => write!(f, ">="),
            Self::InferieurEgal => write!(f, "<="),
            Self::Superieur => write!(f, ">"),
            Self::Inferieur => write!(f, "<"),
        }
    }
}

impl fmt::Display for Operateur {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //'
        match self {
            Self::Ou => write!(f, "ou"),
            Self::Et => write!(f, "et"),
            Self::Non => write!(f, "non"),
            Self::ParentheseBooleen => write!(f, "["),
            Self::Puis => write!(f, "~"),
            Self::Plus => write!(f, "+"),
            Self::Moins => write!(f, "-"),
            Self::Fois => write!(f, "x"),
            Self::Divise => write!(f, "/"),
            Self::ParentheseEntier => write!(f, "("),
        }
    }
}

impl fmt::Display for Programme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //'
        let mut texte: String = format!(
            "{}variables : {:?}{}\n\n",
            TEXTE_GRIS, self.variables, TEXTE_NORMAL
        );
        for phrase in &self.contenu {
            texte += &format!("{}\n", phrase);
        }
        write!(f, "{}", texte)
    }
}

impl fmt::Display for Phrase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //'
        match self {
            Self::Commande(commande) => write!(f, "{}", commande),
            Self::Bloc(bloc) => write!(f, "{}", bloc),
        }
    }
}

impl fmt::Display for Bloc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //'
        let mut texte: String = format!(
            "|{}| : {}[{}",
            liste_element(&self.condition),
            TEXTE_BLEU,
            TEXTE_NORMAL
        );
        for (index, phrase) in self.contenu.iter().enumerate() {
            texte += &format!("{}", phrase);
            if index < self.contenu.len() - 1 {
                texte += &format!("{},{} ", TEXTE_GRIS, TEXTE_NORMAL);
            }
        }
        write!(f, "{}{}]{}", texte, TEXTE_BLEU, TEXTE_NORMAL)
    }
}

fn liste_element(expression: &[Element]) -> String {
    let mut texte = String::new();
    for (index, element) in expression.iter().enumerate() {
        texte += &format!("{}", element);
        if index < expression.len() - 1 {
            texte += &format!("{},{} ", TEXTE_GRIS, TEXTE_NORMAL);
        }
    }
    texte
}
