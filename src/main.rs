use std::env;
use std::fs;
use std::time::Instant;

mod pendragon;
use pendragon::*;
mod sophie;
mod debug;
use debug::display;

fn main() {	
	let arguments: Vec<String> = env::args().collect();

	if arguments.len() < 2 {
		eprintln!("Utilisation : pendragon <FILE>");
		return
	}
	
	let mode_debug = arguments.contains(&"-d".to_string());
	let mode_interprete = arguments.contains(&"-i".to_string());

	let chemin_de_fichier = &arguments[1];
	let mut pendragon = Pendragon::nouveau();
	
	let lecture = fs::read_to_string(chemin_de_fichier);
	
	if let Err(raison) = lecture {
		eprintln!("{}Fichier illisible :{} {}", display::TEXTE_ROUGE, raison, display::TEXTE_NORMAL);
		return
	}
	
	display::message_compilation(chemin_de_fichier);
	let debut = Instant::now();
	if let Err(raison) = pendragon.compile(lecture.unwrap()) {
		for erreur in raison {
			eprintln!("\n{}", erreur);
		}
		display::message_compilation_echec();
		return
	}
	display::message_compilation_ok(debut.elapsed());
	
	if mode_debug {
		println!("\n{}\n", pendragon.programme);
	}
	if !mode_interprete {
		return
	}
	display::message_execution(chemin_de_fichier);
	let debut = Instant::now();
	if let Err(raison) = pendragon.programme.execute() {
		eprintln!("\nErreur : {}", raison);
		display::message_execution_echec();
		return
	}
	display::message_execution_ok(debut.elapsed());
}
