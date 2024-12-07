use std::env;
use std::fs;
use std::fmt;
use std::collections::HashMap;

mod nombres;

enum ErreurSophie {
	CommandeInconnue(String),
	PhraseVide,
	ManqueArgument(String),
	OrthographeNombre(String),
	MauvaisArgument(String),
}

impl fmt::Display for ErreurSophie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
        	Self::CommandeInconnue(commande) => write!(f, "La commande \"{}\" est inconnue.", commande),
            Self::PhraseVide => write!(f, "La phrase est vide."),
            Self::ManqueArgument(commande) => write!(f, "Il manque un argument pour \"{}\".", commande),
            Self::OrthographeNombre(nombre) => write!(f, "Le nombre \"{}\" est mal orthographié.", nombre),
			Self::MauvaisArgument(message) => write!(f, "La commande a reçu un mauvais argument, {}.", message)
        }
    }
}

struct Sophie {
	variables: HashMap<String, usize>,
}

impl Sophie {
	fn new() -> Self {
		Self {
			variables: HashMap::new(),
		}
	}
	fn execute(&mut self, contenu: String) {
		let contenu_propre = contenu.replace("\n", "");
		let mut texte: Vec<&str> = contenu_propre.split('.').collect();
		texte.pop(); // remove empty phrase after last dot
		for (index_phrase, phrase) in texte.iter().enumerate() {
			match self.execute_phrase(phrase) {
				Ok(_) => {},
				Err(raison) => {
					eprintln!("Erreur phrase {} : {}", index_phrase + 1, raison);
					return
				}
			}
		}
	}

	fn execute_phrase(&mut self, phrase: &str) -> Result<(), ErreurSophie> {
		let phrase = phrase.trim();
		let parties: Vec<&str> = phrase.splitn(2, ' ').collect();

		if parties.is_empty() {
			return Err(ErreurSophie::PhraseVide)
		}

		if parties.len() == 1 {
			return Err(ErreurSophie::ManqueArgument(parties[0].to_string()))
		}

		match parties[0] {
			"Modifie" => {
				self.modifie(parties[1])?;
			},
			"Affiche" => {
				self.affiche(parties[1])?;
			},
			"Demande" => {
				self.demande(parties[1])?;
			}
			autre_commande => {
				return Err(ErreurSophie::CommandeInconnue(autre_commande.to_string()))
			}
		};
		Ok(())
	}

	fn modifie(&mut self, arguments: &str) -> Result<(), ErreurSophie> {
		let parties: Vec<&str> = arguments.splitn(2, "avec").collect();
		if parties.len() == 1 {
			return Err(ErreurSophie::ManqueArgument(format!("Modifie {}", parties[0])))
		}
		let variable: String = parties[0].trim().to_string();

		let Some(first_char) = variable.chars().next() else {
			return Err(ErreurSophie::MauvaisArgument("il n'y a pas de variable pour la commande Modifie".to_string()))
		};
		if !first_char.is_uppercase() {
			return Err(ErreurSophie::MauvaisArgument("il manque une majuscule à la variable pour la commande Modifie".to_string()))
		}

		let valeur = self.operation(parties[1].trim())?;
		self.variables.insert(variable, valeur);
		
		Ok(())
	}

	fn affiche(&self, arguments: &str) -> Result<(), ErreurSophie> {
		let liste_arguments: Vec<&str> = arguments.split(',').collect();

		let mut texte = "".to_string();

		for argument in liste_arguments {
			let argument: &str = argument.trim();
			if argument.starts_with('"') {
				if argument.ends_with('"') {
					texte += &argument[1..argument.len()-1];
				}
			} else {
				let resultat = self.operation(argument)?;
				texte += &nombres::nombre_comme_texte(resultat);
			}
		}
		println!("{}", texte);
		Ok(())
	}

	fn demande(&self, arguments: &str) -> Result<(), ErreurSophie> {
		println!("- demande : {}", arguments);
		Ok(())
	}

	pub fn operation(&self, arguments: &str) -> Result<usize, ErreurSophie> {
		let somme_texte: Vec<&str> = arguments.split("plus").collect();
		let mut somme : usize = 0;
		for somme_element in somme_texte {
			let somme_element_propre: &str = somme_element.trim();
			let produit_texte: Vec<&str> = somme_element_propre.split("fois").collect();

			let mut produit : usize = 1;
			for produit_element in produit_texte {
				let produit_element_propre: &str = produit_element.trim();
				let Some(first_char) = produit_element_propre.chars().next() else {
					return Err(ErreurSophie::MauvaisArgument("il y a un argument vide pour l'operation".to_string()))
				};
				let nombre = if first_char.is_uppercase() {
					self.variables[produit_element_propre]
				} else {
					nombres::texte_comme_nombre(produit_element_propre)?
				};
				produit *= nombre;
			}
			somme += produit;
		}
		Ok(somme)
	}
}

fn main() {
	let arguments: Vec<String> = env::args().collect();

	if arguments.len() < 2 {
		eprintln!("Utilisation : sophie <FILE>");
		return
	}

	let chemin_de_fichier = &arguments[1];
	let mut sophie = Sophie::new();
	
	match fs::read_to_string(chemin_de_fichier) {
		Ok(contenu) => {
			sophie.execute(contenu);
		}
		Err(raison) => {
			eprintln!("Fichier illisible : {}", raison);
		}
	}
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
                    assert_eq!(*i, nombre, "Nombre inexact : {}, texte : {}", i, texte);
                }
                Err(raison) => {
                    panic!("Conversion échouée pour : {}, avec l'erreur : {}", i, raison);
                }
            }
        }
    }

	#[test]
    fn teste_somme() {
    	for (a, b) in [(0, 0), (5, 7), (1467,45678), (1001, 0), (72_036_854_775_807usize, 14_036_567_775_807usize)] {
    		let texte_a = nombres::nombre_comme_texte(a);
    		let texte_b = nombres::nombre_comme_texte(b);
    		let sophie = Sophie::new();
			let resultat = sophie.operation(&format!("{} plus {}", texte_a, texte_b));
			match resultat { // Convert text back to number
                Ok(nombre) => {
                    assert_eq!(a+b, nombre, "Résultat inexact pour {}+{} : {}", a, b, nombre);
                }
                Err(raison) => {
                    panic!("Conversion échouée pour : ({},{}), avec l'erreur : {}", a, b, raison);
                }
            }
    	}
    }
    
    #[test]
    fn teste_initialisation_variable() {
    	let mut sophie = Sophie::new();
    	let phrase = "Modifie Variable avec trois plus cent-deux";
    	let resultat = sophie.execute_phrase(phrase);
		match resultat {
			Ok(_) => {
				assert_eq!(sophie.variables["Variable"], 105, "Echec d'initialisation de variable");
			}
			Err(raison) => {
                panic!("Execution échouée pour \"{}\", avec l'erreur : {}", phrase, raison);
            }
		}
    }

    #[test]
    fn teste_operation_variable() {
    	let mut sophie = Sophie::new();
    	let a = 2345678;
    	let b = 987654;
    	let phrase = format!("Modifie Variable avec {}", nombres::nombre_comme_texte(a));
    	if let Err(raison) = sophie.execute_phrase(&phrase) {
    		panic!("Execution échouée pour \"{}\", avec l'erreur : {}", phrase, raison);
    	}
    	let resultat = sophie.operation(&format!("Variable plus {}", nombres::nombre_comme_texte(b)));

    	match resultat {
			Ok(nombre) => {
				assert_eq!(nombre, a+b, "Echec de la somme d'un entier et d'une variable, attendais {}, a reçu {}", a+b, nombre);
			}
			Err(raison) => {
                panic!("Execution échouée pour \"Variable plus {}\", avec l'erreur : {}", nombres::nombre_comme_texte(b), raison);
            }
		}
    }

	#[test]
    fn teste_multiplication() {
    	let sophie = Sophie::new();
    	let resultat = sophie.operation("trois fois deux plus quatre fois sept");
		match resultat {
			Ok(nombre) => {
				assert_eq!(nombre, 34, "Echec de la multiplication de 3*2+4*7, got {}", nombre);
			}
			Err(raison) => {
                panic!("Execution échouée pour multiplication, avec l'erreur : {}", raison);
            }
		}
    }
}
