use std::env;
use std::fs;
use std::time::Instant;
use std::path::Path;
use std::process::Command;

mod pendragon;
use pendragon::*;
mod debug;
mod sophie;
mod morgan;
use debug::display;

fn main() {
	let arguments: Vec<String> = env::args().collect();

	if arguments.len() < 2 {
		eprintln!("Utilisation : pendragon <FILE>");
		return;
	}

	let mode_debug = arguments.contains(&"-d".to_string());
	let mode_debug_verbeux = arguments.contains(&"-v".to_string());
	let mode_interprete = arguments.contains(&"-i".to_string());

	let chemin_de_fichier = &arguments[1];
	let chemin = Path::new(chemin_de_fichier);

	if chemin.extension() != Some(std::ffi::OsStr::new("dr")) {
		eprintln!("devrait être un .dr");
	}
	let Some(nom_fichier_os) = chemin.file_stem() else {
        eprintln!("le fichier n'a pas de nom");
        return;
    };
	let Some(nom_fichier) = nom_fichier_os.to_str() else {
        eprintln!("le nom du fichier n'a pas pu être converti en chaîne de caractères");
        return;
    };
    
	let mut pendragon = Pendragon::nouveau();

	let lecture = fs::read_to_string(chemin_de_fichier);

	if let Err(raison) = lecture {
		eprintln!(
			"{}Fichier illisible :{} {}",
			display::TEXTE_ROUGE,
			raison,
			display::TEXTE_NORMAL
		);
		return;
	}

	display::message_debut("Analyse", chemin_de_fichier);
	let debut = Instant::now();
	if let Err(raison) = pendragon.analyse(lecture.unwrap()) {
		for erreur in raison {
			eprintln!("\n{}", erreur);
		}
		display::message_echec("l'analyse");
		return;
	}
	display::message_ok("Analyse",debut.elapsed());

	if mode_debug {
		println!("\n{}\n", pendragon.programme);
	}
	
	if mode_debug_verbeux {
		println!("\n{:?}\n", pendragon.programme);
	}
	
	if mode_interprete {
		display::message_debut("Interprétation", chemin_de_fichier);
		println!(" ");
		let debut = Instant::now();
		if let Err(raison) = pendragon.programme.execute() {
			eprintln!("\nErreur : {}", raison);
			display::message_echec("l'interprétation");
			return;
		}
		println!(" ");
		display::message_ok("Interprétation", debut.elapsed());
		return;
	}
	display::message_debut("Compilation", chemin_de_fichier);
	let debut = Instant::now();
	if let Err(raison) = pendragon.programme.compile(nom_fichier) {
		eprintln!("\nErreur : {}", raison);
		display::message_echec("la compilation");
		return;
	}
	display::message_ok("Compilation", debut.elapsed());
	display::message_debut("Exécution", nom_fichier);
	println!(" ");
	let debut = Instant::now();
	let status = Command::new(format!("./{}", nom_fichier))
        .status()
        .expect("Failed to execute file");
    if !status.success() {
   		eprintln!("Erreur : Le fichier n'a pas pu être exécuté");
   		display::message_echec("l'exécution");
        return;
    }
    println!(" ");
	display::message_ok("Execution", debut.elapsed());
}
