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
		
		let main_start: &str = "\ndefine i32 @main() {\n%fmt_ptr = getelementptr [3 x i8], [3 x i8]* @format_str, i32 0, i32 0\n%nouvelle_ligne = getelementptr [3 x i8], [3 x i8]* @newline, i32 0, i32 0\n";
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
				let ir_type: &str = match type_element {
					TypeElement::Entier => "i64",
					TypeElement::Booleen => "i1",
					TypeElement::Texte => "i8",
				};
				instruction.body += &format!("%{}-0 = add {} 0, 0\n", nom,ir_type);
				Ok(instruction)
			}
			Commande::Demande(_nom) => {
				Ok(Instruction::default())
			}
			Commande::Modifie(nom, expression) => {
				let mut expression_type: &str = EXPRESSION_NOMBRE;
				let mut instruction = Instruction::default();
				let ir_type: &str = match var_types[nom] {
					TypeElement::Entier => {
						instruction.add(nombre::calcule_nombre(expression.clone(), var.clone())?);
						"i64"
					}
					TypeElement::Texte => {
						return Err(ErreurMorgan::MauvaisArgument("Variable texte pas implémentées".to_string()));
						//"i8"
						//expression_type = EXPRESSION_TEXTE
					}
					TypeElement::Booleen => {
						instruction.add(booleen::calcule_booleen(expression.clone(), var.clone())?);
						expression_type = EXPRESSION_BOOLEEN;
						"i1"
					}
				};
				let current_expression_index = instruction.var[expression_type];
				let current_variable_index = instruction.var.entry(nom.to_string()).and_modify(|e| *e += 1).or_insert(0);
				
				instruction.body += &format!("%{}-{} = add {} %{}-{}-fin, 0\n", 
					nom, current_variable_index, ir_type,
					expression_type, current_expression_index);
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
