use super::*;

pub fn calcule_booleen(
	expression: Vec<Element>,
	var: HashMap<String, usize>,
) -> Result<Instruction, ErreurMorgan> {
	let mut instruction = Instruction::new(var);
	let current_index = instruction.var.entry(EXPRESSION_BOOLEEN.to_string()).and_modify(|e| *e += 1).or_insert(0);
	let mut expression_index: usize = 0;
	
	for element in expression {
		if let Element::Booleen(booleen) = element {
			expression_index += 1;
			instruction.body += &format!("%{}-{}-{} = add i1 {}, 0\n", EXPRESSION_BOOLEEN, current_index, expression_index, booleen as i32);
			continue
		}
		let Element::Operateur(ref operateur) = element else {
			return Err(ErreurMorgan::MauvaisArgument(format!(
				"{}, attendais un opérateur",
				element
			)));
		};
		match operateur {
			Operateur::Non => {
				expression_index += 1;
				instruction.body += &format!("%{}-{}-{} = xor i1 %{}-{}-{}, true\n", 
					EXPRESSION_BOOLEEN, current_index, expression_index, 
					EXPRESSION_BOOLEEN, current_index, expression_index-1);
			}
			Operateur::Et => {
				expression_index += 1;
				instruction.body += &format!("%{}-{}-{} = and i1 %{}-{}-{}, %{}-{}-{}\n", 
					EXPRESSION_BOOLEEN, current_index, expression_index, 
					EXPRESSION_BOOLEEN, current_index, expression_index-1,
					EXPRESSION_BOOLEEN, current_index, expression_index-2);
			}
			Operateur::Ou => {
				expression_index += 1;
				instruction.body += &format!("%{}-{}-{} = or i1 %{}-{}-{}, %{}-{}-{}\n", 
					EXPRESSION_BOOLEEN, current_index, expression_index, 
					EXPRESSION_BOOLEEN, current_index, expression_index-1,
					EXPRESSION_BOOLEEN, current_index, expression_index-2);
			}
			_ => {
				return Err(ErreurMorgan::MauvaisArgument(format!(
					"{}, attendais un opérateur booléen",
					element
				)))
			}
		}
	}
	if expression_index > 0 {
		instruction.body += &format!("%{}-{}-fin = add i1 %{}-{}-{}, 0\n", 
			EXPRESSION_BOOLEEN, current_index, 
			EXPRESSION_BOOLEEN, current_index, expression_index);
	}
	
	Ok(instruction)
}
