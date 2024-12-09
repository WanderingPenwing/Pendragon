use std::env;
use std::fs;

mod pendragon;
use pendragon::*;

fn main() {
	let arguments: Vec<String> = env::args().collect();

	if arguments.len() < 2 {
		eprintln!("Utilisation : pendragon <FILE>");
		return
	}

	let chemin_de_fichier = &arguments[1];
	let mut pendragon = Pendragon::new();
	
	match fs::read_to_string(chemin_de_fichier) {
		Ok(contenu) => {
			let _ = pendragon.execute(contenu);
		}
		Err(raison) => {
			eprintln!("Fichier illisible : {}", raison);
		}
	}
}
