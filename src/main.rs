use std::env;
use std::fs;
use std::fmt;

mod nombres;

enum ErreurSophie {
	CommandeInconnue(String),
	PhraseVide,
	ManqueArgument,
	OrthographeNombre(String),
}

impl fmt::Display for ErreurSophie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
        	Self::CommandeInconnue(commande) => write!(f, "La commande \"{}\" est inconnue.", commande),
            Self::PhraseVide => write!(f, "La phrase est vide."),
            Self::ManqueArgument => write!(f, "Il manque un argument."),
            Self::OrthographeNombre(nombre) => write!(f, "Le nombre \"{}\" est mal orthographié.", nombre),
        }
    }
}

fn main() {
	let arguments: Vec<String> = env::args().collect();

	if arguments.len() < 2 {
		eprintln!("Utilisation : sophie <FILE>");
		return
	}

	let chemin_de_fichier = &arguments[1];
	
	match fs::read_to_string(chemin_de_fichier) {
		Ok(contenu) => {
			execute(contenu);
		}
		Err(raison) => {
			eprintln!("Fichier illisible : {}", raison);
		}
	}
}

fn execute(contenu: String) {
	let contenu_propre = contenu.replace("\n", "");
	let mut texte: Vec<&str> = contenu_propre.split('.').collect();
	texte.pop(); // remove empty phrase after last dot
	for (index_phrase, phrase) in texte.iter().enumerate() {
		match execute_phrase(phrase) {
			Ok(_) => {},
			Err(raison) => {
				eprintln!("Erreur phrase {} : {}", index_phrase + 1, raison);
				return
			}
		}
	}
}

fn execute_phrase(phrase: &str) -> Result<(), ErreurSophie> {
	let phrase = phrase.trim();
	let parties: Vec<&str> = phrase.splitn(2, ' ').collect();

	if parties.is_empty() {
		return Err(ErreurSophie::PhraseVide)
	}

	if parties.len() == 1 {
		return Err(ErreurSophie::ManqueArgument)
	}

	match parties[0] {
		"Modifie" => {
			modifie(parties[1])?;
		},
		"Affiche" => {
			affiche(parties[1])?;
		},
		"Demande" => {
			demande(parties[1])?;
		}
		autre_commande => {
			return Err(ErreurSophie::CommandeInconnue(autre_commande.to_string()))
		}
	};
	Ok(())
}

fn modifie(arguments: &str) -> Result<(), ErreurSophie> {
	println!("- modifie : {}", arguments);
	Ok(())
}

fn affiche(arguments: &str) -> Result<(), ErreurSophie> {
	let liste_arguments: Vec<&str> = arguments.split(',').collect();

	let mut texte = "".to_string();

	for argument in liste_arguments {
		let argument: &str = argument.trim();
		if argument.starts_with('"') {
			if argument.ends_with('"') {
				texte += &argument[1..argument.len()-1];
			}
		} else {
			let resultat = nombres::operation(argument)?;
			texte += &nombres::nombre_comme_texte(resultat);
		}
	}
	println!("{}", texte);
	Ok(())
}

fn demande(arguments: &str) -> Result<(), ErreurSophie> {
	println!("- demande : {}", arguments);
	Ok(())
}



#[cfg(test)] // Compile and run only during testing
mod tests {
    use super::*;

    #[test]
    fn teste_conversion_nombres_texte() {
        // Test on a limited set of numbers to ensure feasibility
        for i in [0, 1, 42, 123, 999, 1031, 1_001_091, 72_036_854_775_807usize].iter() {
            let texte = nombres::nombre_comme_texte(*i); // Convert number to text
            match nombres::texte_comme_nombre(&texte) { // Convert text back to number
                Ok(nombre) => {
                    assert_eq!(*i, nombre, "Mismatch for number: {}, text: {}", i, texte);
                }
                Err(raison) => {
                    panic!("Conversion failed for number: {} with error: {}", i, raison);
                }
            }
        }
    }

	#[test]
    fn teste_somme() {
    	for (a, b) in [(5, 7), (1467,45678), (1001, 0), (72_036_854_775_807usize, 14_036_567_775_807usize)] {
    		let texte_a = nombres::nombre_comme_texte(a);
    		let texte_b = nombres::nombre_comme_texte(b);
			let resultat = nombres::operation(&format!("{} plus {}", texte_a, texte_b));
			match resultat { // Convert text back to number
                Ok(nombre) => {
                    assert_eq!(a+b, nombre, "Mismatch for {}+{}, got: {}", a, b, nombre);
                }
                Err(raison) => {
                    panic!("Conversion failed for number: ({},{}) with error: {}", a, b, raison);
                }
            }
    	}
    }
}
