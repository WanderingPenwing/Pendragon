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
pub const BLOC: &str = "bloc";

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
	pub fn add_bloc(&mut self, instruction: Self) {
		self.declaration += &instruction.declaration;
		self.declaration += "\n";
		self.declaration += &instruction.body;
		self.declaration += "\n";
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
		let mut main_instruction = Instruction::default();

		for phrase in &self.contenu {
			match phrase {
				Phrase::Commande(commande) => {
					main_instruction.add(commande.traduit(main_instruction.var.clone(), main_instruction.var_types.clone())?);
				}
				Phrase::Bloc(bloc) => {
					main_instruction.add(bloc.traduit(main_instruction.var.clone(), main_instruction.var_types.clone())?);
				}
			}
		}
		
		let main_start: &str = "\ndefine i32 @main() {\n\t%fmt_ptr = getelementptr [3 x i8], [3 x i8]* @format_str, i32 0, i32 0\n\t%nouvelle_ligne = getelementptr [2 x i8], [2 x i8]* @newline, i32 0, i32 0\n";
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
				instruction.body += &format!("\t%{}-0 = {}\n", nom, default_value);
				Ok(instruction)
			}
			Commande::Demande(nom) => {
				let mut instruction = Instruction::new(var);
				let current_variable_index = instruction.var.entry(nom.to_string()).and_modify(|e| *e += 1).or_insert(0);
				instruction.declaration += &format!("@{}-{}-nom = private unnamed_addr constant [{} x i8] c\"{}\\00\"\n", nom, current_variable_index, nom.len()+1, nom);
				instruction.body += &format!("\t%{}-{}-nom = getelementptr [{} x i8], [{} x i8]* @{}-{}-nom, i32 0, i32 0\n", 
					nom, current_variable_index,nom.len()+1,nom.len()+1,nom, current_variable_index
				);
				match var_types[nom] {
					TypeElement::Entier => {
						instruction.body += &format!("\t%{}-{} = call i64 @demande_entier(i8* %{}-{}-nom)\n",
							nom, current_variable_index, nom, current_variable_index
						);
					}
					TypeElement::Texte => {
						instruction.body += &format!("\t%{}-{} = call i8* @demande_texte(i8* %{}-{}-nom)\n",
							nom, current_variable_index, nom, current_variable_index
						);
					}
					TypeElement::Booleen => {
						instruction.body += &format!("\t%{}-{} = call i1 @demande_booleen(i8* %{}-{}-nom)\n",
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
						instruction.body += &format!("\t%{}-{} = add i64 %{}-{}-fin, 0\n", 
							nom, current_variable_index, EXPRESSION_NOMBRE, current_expression_index
						);
					}
					TypeElement::Texte => {
						instruction.add(texte::calcule_texte(expression.clone(), var.clone())?);
						let current_expression_index = instruction.var[EXPRESSION_TEXTE];
						let current_variable_index = instruction.var.entry(nom.to_string()).and_modify(|e| *e += 1).or_insert(0);
						instruction.body += &format!("\t%{}-{} = getelementptr i8, i8* %{}-{}-fin, i32 0\n", 
							nom, current_variable_index, EXPRESSION_TEXTE, current_expression_index
						);
					}
					TypeElement::Booleen => {
						instruction.add(booleen::calcule_booleen(expression.clone(), var.clone())?);
						let current_expression_index = instruction.var[EXPRESSION_BOOLEEN];
						let current_variable_index = instruction.var.entry(nom.to_string()).and_modify(|e| *e += 1).or_insert(0);
						instruction.body += &format!("\t%{}-{} = add i1 %{}-{}-fin, 0\n", 
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
				instruction.body += &format!("\tcall i32 (i8*, ...) @printf(i8* %fmt_ptr, i8* %{}-{}-fin)\n\tcall i32 (i8*, ...) @printf(i8* %fmt_ptr, i8* %nouvelle_ligne)\n", 
					EXPRESSION_TEXTE, current_texte_index
				);
				Ok(instruction)
			}
		}
	}
}

impl Bloc {
	fn traduit_contenu(&self, var: HashMap<String, usize>, var_types: HashMap<String, TypeElement>) -> Result<Instruction, ErreurMorgan> {
		let mut instruction = Instruction::new(var);
		println!("traduit contenu {:?}",var_types);
		instruction.var_types = var_types;
		println!("traduit contenu instruction {:?}",instruction.var_types);
		instruction.add(booleen::calcule_booleen(self.condition.clone(), instruction.var.clone())?);
		let current_condition_index = instruction.var[EXPRESSION_BOOLEEN];
		instruction.body += &format!("\tbr i1 %{}-{}-fin, label %continue, label %stop\n", EXPRESSION_BOOLEEN, current_condition_index);
		instruction.body += "stop:\n";
		instruction.body += &self.return_expression(instruction.var.clone(), instruction.var_types.clone(), 0)?;
		instruction.body += "continue:\n";
		for phrase in &self.contenu {
			match phrase {
				Phrase::Commande(commande) => {
					instruction.add(commande.traduit(instruction.var.clone(), instruction.var_types.clone())?);
				}
				Phrase::Bloc(bloc) => {
					instruction.add_bloc(bloc.traduit(instruction.var.clone(), instruction.var_types.clone())?);
				}
			}
		}
		Ok(instruction)
	}
	
	fn return_expression(&self, var: HashMap<String, usize>, var_types: HashMap<String, TypeElement>, return_index: usize) -> Result<String, ErreurMorgan> {
		let mut expression = String::new();
		let mut result_index: usize = 0;
		println!("expression {:?}",var_types);
		let return_type = self.return_type(var_types.clone())?;
		for variable in self.variables_externes.iter() {
			let Some(current_var_index) = var.get(variable) else {
				return Err(ErreurMorgan::ManqueVariable(format!("{}:var_index in return_exp",variable)));
			};
			let Some(type_var) = var_types.get(variable) else {
				return Err(ErreurMorgan::ManqueVariable(format!("{}:var_type in return_exp",variable)));
			};
			let last_result: String = if result_index == 0 {
				"undef".into()
			} else {
				format!("%result-{}-{}", return_index, result_index-1)
			};
			expression += &format!("\t%result-{}-{} = insertvalue {} {}, {} %{}-{}, {}\n", 
				return_index, result_index, return_type, last_result, type_var.type_ir(), variable, current_var_index, result_index
			);
			result_index += 1;
		}
		println!("expression {:?}",self.variables_externes);
		expression += &format!("\tret {} %result-{}-{}\n", return_type, return_index, result_index-1);
		Ok(expression)
	}
	
	fn return_type(&self, var_types: HashMap<String, TypeElement>) -> Result<String, ErreurMorgan> {
		let mut function_type = "{".to_string();
		for variable in self.variables_externes.iter() {
			let Some(type_var) = var_types.get(variable) else {
				println!("{:?}",var_types);
				return Err(ErreurMorgan::ManqueVariable(format!("{}:var_type in return_type",variable)));
			};
			let comma: &str = if &function_type == "{" {
				""
			} else {
				","
			};
			function_type += &format!("{} {}",comma, type_var.type_ir());
		}
		function_type += " }";
		Ok(function_type)
	}
	
	fn traduit(&self, var: HashMap<String, usize>, var_types: HashMap<String, TypeElement>) -> Result<Instruction, ErreurMorgan> {
		let mut instruction = Instruction::new(var);
		let current_index = instruction.var.entry(BLOC.to_string()).and_modify(|e| *e += 1).or_insert(0).clone();
		let mut result_index: usize = 0;
		println!("traduit {:?}",var_types);
		let return_type = self.return_type(var_types.clone())?;
		
		let mut input: String = String::new();
		for variable in self.variables_externes.iter() {
			let Some(current_var_index) = instruction.var.get(variable) else {
				return Err(ErreurMorgan::ManqueVariable(format!("{}:var_index in traduit",variable)));
			};
			let Some(type_var) = var_types.get(variable) else {
				return Err(ErreurMorgan::ManqueVariable(format!("{}:var_type in traduit",variable)));
			};
			let comma: &str = if &input == "" {
				""
			} else {
				","
			};
			input += &format!("{} {} %{}-{}", comma, type_var.type_ir(), variable, current_var_index);
		}
		instruction.body += &format!("\t%result-bloc-{} = call {} @bloc-{}({})\n",
			current_index, return_type, current_index, input
		);
		
		let mut contenu = Instruction::default();
		contenu.body += &format!("define {} @bloc-{}({}) ", return_type, current_index, input);
		contenu.body += "{\nentry:\n\t%fmt_ptr = getelementptr [3 x i8], [3 x i8]* @format_str, i32 0, i32 0\n\t%nouvelle_ligne = getelementptr [2 x i8], [2 x i8]* @newline, i32 0, i32 0\n";
		contenu.add(self.traduit_contenu(instruction.var.clone(), var_types.clone())?);
		if self.repete {
			let mut output: String = String::new();
			for variable in self.variables_externes.iter() {
				let Some(current_var_index) = contenu.var.get(variable) else {
					return Err(ErreurMorgan::ManqueVariable(format!("{}:var_index in traduit",variable)));
				};
				let Some(type_var) = var_types.get(variable) else {
					return Err(ErreurMorgan::ManqueVariable(format!("{}:var_type in traduit",variable)));
				};
				let comma: &str = if &output == "" {
					""
				} else {
					","
				};
				output += &format!("{} {} %{}-{}", comma, type_var.type_ir(), variable, current_var_index);
			}
			contenu.body += &format!("\t%result-fin = call {} @bloc-{}({})\n\t ret {} %result-fin\n", return_type, current_index, output, return_type);
		} else {
			contenu.body += &self.return_expression(instruction.var.clone(), var_types.clone(), 1)?;
		}
		contenu.body += "}\n";
		instruction.add_bloc(contenu);
		
		for variable in self.variables_externes.iter() {
			let current_var_index = instruction.var.entry(variable.to_string()).and_modify(|e| *e += 1).or_insert(0).clone();
			instruction.body += &format!("\t%{}-{} = extractvalue {} %result-bloc-{}, {}\n", 
				variable, current_var_index, return_type, current_index, result_index
			);
			result_index += 1;
		}
		Ok(instruction)
	}
}

impl Element {
	pub fn recupere_variables(&self) -> Vec<String> {
		let mut variables: Vec<String> = vec![];
		match self {
			Self::Variable(nom, _type) => {
				variables.push(nom.to_string());
			}
			Self::Comparaison(comparaison) => {
				for element in comparaison.membre_a.iter() {
					variables.extend(element.recupere_variables());
				}
				for element in comparaison.membre_b.iter() {
					variables.extend(element.recupere_variables());
				}
			}
			_ => {},
		}
		variables
	}
}
