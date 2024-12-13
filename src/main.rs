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
		eprintln!("{}Fichier illisible :{} {}", debug::TEXTE_ROUGE, raison, debug::TEXTE_NORMAL);
		return
	}
	
	debug::message_compilation(chemin_de_fichier);
	let debut = Instant::now();
	if let Err(raison) = pendragon.compile(lecture.unwrap()) {
		eprintln!("\n{}", raison);
		debug::message_compilation_echec();
		return
	}
	debug::message_compilation_ok(debut.elapsed());
	
	if debug_mode {
		println!("{}\n", pendragon.programme);
	}
	
	
	debug::message_execution(chemin_de_fichier);
	let debut = Instant::now();
	if let Err(raison) = pendragon.programme.execute() {
		eprintln!("\nErreur : {}", raison);
		debug::message_execution_echec();
		return
	}
	debug::message_execution_ok(debut.elapsed());
}
