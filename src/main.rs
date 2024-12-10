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
	
	let debug_mode = arguments.contains(&"--debug".to_string());

	let chemin_de_fichier = &arguments[1];
	let mut pendragon = Pendragon::nouveau();
	
	match fs::read_to_string(chemin_de_fichier) {
		Ok(contenu) => {
			let Ok(_) = pendragon.compile(contenu) else {
				eprintln!("Compilation interrompue");
				return
			};
			if debug_mode {
				println!("{}\n-----------", pendragon.programme);
			}
			if let Err(raison) = pendragon.programme.execute() {
				eprintln!("Erreur Execution : {}", raison);
				return
			}
			println!("\n# Success");
		}
		Err(raison) => {
			eprintln!("Fichier illisible : {}", raison);
		}
	}
}
