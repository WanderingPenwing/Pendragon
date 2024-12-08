use std::env;
use std::fs;

mod sophie;
use sophie::*;

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
			let _ = sophie.execute(contenu);
		}
		Err(raison) => {
			eprintln!("Fichier illisible : {}", raison);
		}
	}
}
