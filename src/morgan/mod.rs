use crate::pendragon::structure::*;
use crate::display::ErreurMorgan;
use std::process::Command;
use std::fs::File;
use std::io::Write;

pub mod booleen;
pub mod nombre;
pub mod texte;

pub const MAIN_IR: &str = include_str!("../../ir-src/main.ll");

#[derive(Default)]
pub struct Instruction {
	body: String,
}

impl Instruction {
	pub fn add(&mut self, instruction: Self) {
		self.body += &instruction.body;
	}
}

impl Programme {
    pub fn compile(&self, name: &str) -> Result<(), ErreurMorgan> {
    	//let mut main_body: String = "call void @affiche_nombre(i64 987)".to_string();
    	let mut main_body: String = String::new();
		let variable_declaration: String = String::new();

		for phrase in &self.contenu {
            match phrase {
                Phrase::Commande(commande) => {
                	let result = commande.traduit()?;
          			main_body += &result.body;
                }
                Phrase::Bloc(_) => {},
            }
        }
		
    	let main_start: &str = "\ndefine i32 @main() {\n";
    	let main_end: &str = "\nret i32 0\n}";
    	let programme: String = format!("{}{}{}{}{}", MAIN_IR, variable_declaration, main_start, main_body, main_end);
        
        match File::create(format!("{}.ll", name)) {
        	Ok(mut fichier) => {
        		if let Err(raison) = fichier.write_all(programme.as_bytes()) {
        			return Err(ErreurMorgan::ErreurSysteme(format!("l'écriture du fichier .ll a échoué : {}", raison)));
        		}

				let status = Command::new("llc")
			        .arg("-filetype=asm")
			        .arg("-relocation-model=pic") // Generate position-independent code
			        .arg(format!("{}.ll", name)) // Input LLVM IR file
			        .arg("-o")
			        .arg(format!("{}.s", name)) // Output assembly file
			        .status()
			        .expect("Failed to execute llc");
			    if !status.success() {
		            return Err(ErreurMorgan::ErreurSysteme("llc n'a pas pu compiler le fichier .ll".to_string()));
		        }
		        let status = Command::new("clang")
	                .arg("-fPIE") // Ensure position-independent code
	                .arg("-pie")  // Generate PIE executable
	                .arg(format!("{}.s", name)) // Input assembly file
	                .arg("-o")
	                .arg(name) // Output executable
	                .status()
	                .expect("Failed to execute clang");
	            if !status.success() {
   		            return Err(ErreurMorgan::ErreurSysteme("clang n'a pas pu lier le fichier .s".to_string()));
   		        }
        		Ok(())
        	}
        	Err(raison) => {
        		Err(ErreurMorgan::ErreurSysteme(format!("la création du fichier .ll a échoué : {}", raison)))
        	}
        }
    }
}


impl Commande {
    fn traduit(&self) -> Result<Instruction, ErreurMorgan> {
        match self {
            Commande::Definis(_nom, _type_element) => {
                Ok(Instruction::default())
            }
            Commande::Demande(_nom) => {
                Ok(Instruction::default())
            }
            Commande::Modifie(_nom, _expression) => {
                Ok(Instruction::default())
            }
            Commande::Affiche(expression) => {
            	Ok(texte::calcule_texte(expression.to_vec())?)
            }
        }
    }
}
