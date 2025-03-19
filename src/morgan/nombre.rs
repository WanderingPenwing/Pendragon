use super::*;

pub fn calcule_nombre(
	expression: Vec<Element>,
	var: HashMap<String, usize>,
) -> Result<Instruction, ErreurMorgan> {
	let mut instruction = Instruction::new(var);
	let current_index = instruction.var.entry(EXPRESSION_NOMBRE.to_string()).and_modify(|e| *e += 1).or_insert(0).clone();
	let mut expression_index: usize = 0;
	
	for element in expression {
		if let Element::Entier(nombre) = element {
			expression_index += 1;
			instruction.body += &format!("%{}-{}-{} = add i64 {}, 0\n", EXPRESSION_NOMBRE, current_index, expression_index, nombre);
			continue;
		}
		if let Element::Variable(nom, _) = element {
			expression_index += 1;
			let Some(&current_var_index) = instruction.var.get(&nom) else {
				return Err(ErreurMorgan::ManqueVariable(nom.to_string()));
			};
			instruction.body += &format!("%{}-{}-{} = add i64 %{}-{}, 0\n", 
				EXPRESSION_NOMBRE, current_index, expression_index,
				nom, current_var_index);
			continue
		}
		let Element::Operateur(ref operateur) = element else {
			return Err(ErreurMorgan::MauvaisArgument(format!(
				"{}, attendais un opérateur",
				element
			)));
		};
		let operation: &str = match operateur {
			Operateur::Plus => "add",
			Operateur::Moins => "sub",
			Operateur::Fois => "mul",
			Operateur::Divise => "sdiv",
			_ => "",
		};
		expression_index += 1;
		instruction.body += &format!("%{}-{}-{} = {} i64 %{}-{}-{}, %{}-{}-{}\n", 
			EXPRESSION_NOMBRE, current_index, expression_index, operation,
			EXPRESSION_NOMBRE, current_index, expression_index-2,
			EXPRESSION_NOMBRE, current_index, expression_index-1);
	}
	if expression_index > 0 {
		instruction.body += &format!("%{}-{}-fin = add i64 %{}-{}-{}, 0\n", 
			EXPRESSION_NOMBRE, current_index, 
			EXPRESSION_NOMBRE, current_index, expression_index);
	}
	
	Ok(instruction)
}
