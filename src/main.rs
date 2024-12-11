use std::env;
use std::fs;
use std::time::Instant;

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
	
	let lecture = fs::read_to_string(chemin_de_fichier);
	
	if let Err(raison) = lecture {
		eprintln!("Fichier illisible : {}", raison);
		return
	}
	
	println!("# Compilation de '{}'.", chemin_de_fichier);
	let debut = Instant::now();
	if let Err(raison) = pendragon.compile(lecture.unwrap()) {
		eprintln!("\n{}", raison);
		eprintln!("\n# Échec de la compilation.");
		return
	}
	println!("# Compilation Ok. ({:.2?})\n", debut.elapsed());
	
	if debug_mode {
		println!("{}\n", pendragon.programme);
	}
	
	
	println!("# Exécution de '{}'.\n", chemin_de_fichier);
	let debut = Instant::now();
	if let Err(raison) = pendragon.programme.execute() {
		eprintln!("\nErreur : {}", raison);
		eprintln!("\n# Échec de l'exécution.");
		return
	}
	
	println!("\n# Exécution Ok. ({:.2?})", debut.elapsed());
}
