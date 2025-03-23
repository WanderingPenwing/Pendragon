use crate::pendragon::structure::*;
use crate::display::ErreurMorgan;
use std::process::Command;
use std::fs::File;
use std::io::Write;
use std::collections::HashMap;

pub mod booleen;
pub mod nombre;
pub mod texte;

pub const MAIN_IR: &str = include_str!("../../ir-src/main.ll");
pub const EXPRESSION_TEXTE: &str = "expression_texte";
pub const EXPRESSION_NOMBRE: &str = "expression_nombre";
pub const EXPRESSION_BOOLEEN: &str = "expression_booleen";
pub const TEXTE_GLOBAL: &str = "texte_global";
pub const COMPARAISON: &str = "comparaison";

#[derive(Default)]
pub struct Instruction {
	body: String,
	var: HashMap<String, usize>,
	var_types: HashMap<String, TypeElement>,
	declaration : String,
}

impl Instruction {
	pub fn add(&mut self, instruction: Self) {
		self.body += &instruction.body;
		self.declaration += &instruction.declaration;
		for (key, &value) in instruction.var.iter() {
			self.var.entry(key.clone())
				.and_modify(|e| *e = (*e).max(value))
				.or_insert(value);
		}
		for (key, value) in instruction.var_types.iter() {
			self.var_types.entry(key.clone())
				.or_insert(value.clone());
		}
	}
	pub fn new(var: HashMap<String, usize>) -> Self {
		Self {
			body: String::new(),
			var,
			var_types: HashMap::new(),
			declaration: String::new(),
		}
	}
}

impl Programme {
	pub fn compile(&self, name: &str) -> Result<(), ErreurMorgan> {
		//let mut main_body: String = "call void @affiche_nombre(i64 987)".to_string();
		let mut main_instruction = Instruction::default();

		for phrase in &self.contenu {
			match phrase {
				Phrase::Commande(commande) => {
					main_instruction.add(commande.traduit(main_instruction.var.clone(), main_instruction.var_types.clone())?);
				}
				Phrase::Bloc(_) => {},
			}
		}
		
		let main_start: &str = "\ndefine i32 @main() {\n%fmt_ptr = getelementptr [3 x i8], [3 x i8]* @format_str, i32 0, i32 0\n%nouvelle_ligne = getelementptr [2 x i8], [2 x i8]* @newline, i32 0, i32 0\n";
		let main_end: &str = "\nret i32 0\n}";
		let programme: String = format!("{}{}{}{}{}", MAIN_IR, main_instruction.declaration, main_start, main_instruction.body, main_end);
		
		match File::create(format!("{}.ll", name)) {
			Ok(mut fichier) => {
				if let Err(raison) = fichier.write_all(programme.as_bytes()) {
					return Err(ErreurMorgan::ErreurSysteme(format!("l'écriture du fichier .ll a échoué : {}", raison)));
				}

				let status = Command::new("llc") // llc -filetype=asm -relocation-model=pic example.ll -o example.s
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
				let status = Command::new("clang") // clang -fPIE -pie example.s -o example
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
	fn traduit(&self, var: HashMap<String, usize>, var_types: HashMap<String, TypeElement>) -> Result<Instruction, ErreurMorgan> {
		match self {
			Commande::Definis(nom, type_element) => {
				let mut instruction = Instruction::new([(nom.to_string(),0)].into_iter().collect());
				instruction.var_types.insert(nom.to_string(), type_element.clone());
				let default_value: &str = match type_element {
					TypeElement::Entier => "add i64 0, 0",
					TypeElement::Booleen => "add i1 0, 0",
					TypeElement::Texte => "getelementptr [1 x i8], [1 x i8]* @vide, i32 0, i32 0",
				};
				instruction.body += &format!("%{}-0 = {}\n", nom, default_value);
				Ok(instruction)
			}
			Commande::Demande(nom) => {
				let mut instruction = Instruction::new(var);
				let current_variable_index = instruction.var.entry(nom.to_string()).and_modify(|e| *e += 1).or_insert(0);
				instruction.declaration += &format!("@{}-{}-nom = private unnamed_addr constant [{} x i8] c\"{}\\00\"\n", nom, current_variable_index, nom.len()+1, nom);
				instruction.body += &format!("%{}-{}-nom = getelementptr [{} x i8], [{} x i8]* @{}-{}-nom, i32 0, i32 0\n", 
					nom, current_variable_index,nom.len()+1,nom.len()+1,nom, current_variable_index
				);
				match var_types[nom] {
					TypeElement::Entier => {
						instruction.body += &format!("%{}-{} = call i64 @demande_entier(i8* %{}-{}-nom)\n",
							nom, current_variable_index, nom, current_variable_index
						);
					}
					TypeElement::Texte => {
						instruction.body += &format!("%{}-{} = call i8* @demande_texte(i8* %{}-{}-nom)\n",
							nom, current_variable_index, nom, current_variable_index
						);
					}
					TypeElement::Booleen => {
						instruction.body += &format!("%{}-{} = call i1 @demande_booleen(i8* %{}-{}-nom)\n",
							nom, current_variable_index, nom, current_variable_index
						);
					}
				}
				Ok(instruction)
			}
			Commande::Modifie(nom, expression) => {
				let mut instruction = Instruction::default();				
				match var_types[nom] {
					TypeElement::Entier => {
						instruction.add(nombre::calcule_nombre(expression.clone(), var.clone())?);
						let current_expression_index = instruction.var[EXPRESSION_NOMBRE];
						let current_variable_index = instruction.var.entry(nom.to_string()).and_modify(|e| *e += 1).or_insert(0);
						instruction.body += &format!("%{}-{} = add i64 %{}-{}-fin, 0\n", 
							nom, current_variable_index, EXPRESSION_NOMBRE, current_expression_index
						);
					}
					TypeElement::Texte => {
						instruction.add(texte::calcule_texte(expression.clone(), var.clone())?);
						let current_expression_index = instruction.var[EXPRESSION_TEXTE];
						let current_variable_index = instruction.var.entry(nom.to_string()).and_modify(|e| *e += 1).or_insert(0);
						instruction.body += &format!("%{}-{} = getelementptr i8, i8* %{}-{}-fin, i32 0\n", 
							nom, current_variable_index, EXPRESSION_TEXTE, current_expression_index
						);
					}
					TypeElement::Booleen => {
						instruction.add(booleen::calcule_booleen(expression.clone(), var.clone())?);
						let current_expression_index = instruction.var[EXPRESSION_BOOLEEN];
						let current_variable_index = instruction.var.entry(nom.to_string()).and_modify(|e| *e += 1).or_insert(0);
						instruction.body += &format!("%{}-{} = add i1 %{}-{}-fin, 0\n", 
							nom, current_variable_index, EXPRESSION_BOOLEEN, current_expression_index
						);
					}
				};
				
				
				Ok(instruction)
			}
			Commande::Affiche(expression) => {
				let mut instruction = texte::calcule_texte(expression.to_vec(), var)?;
				let Some(current_texte_index) = instruction.var.get(EXPRESSION_TEXTE) else {
					return Err(ErreurMorgan::ManqueVariable(EXPRESSION_TEXTE.to_string()));
				};
				instruction.body += &format!("call i32 (i8*, ...) @printf(i8* %fmt_ptr, i8* %{}-{}-fin)\ncall i32 (i8*, ...) @printf(i8* %fmt_ptr, i8* %nouvelle_ligne)\n", 
					EXPRESSION_TEXTE, current_texte_index
				);
				Ok(instruction)
			}
		}
	}
}
