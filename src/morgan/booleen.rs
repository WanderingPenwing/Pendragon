use super::*;

pub fn calcule_booleen(
	expression: Vec<Element>,
	var: HashMap<String, usize>,
) -> Result<Instruction, ErreurMorgan> {
	if expression.len() != 1 {
		return Err(ErreurMorgan::MauvaisArgument("expression booléene complexe".to_string()));
	}
	let Some(past_index) = var.get(EXPRESSION_BOOLEEN) else {
		return Err(ErreurMorgan::ManqueVariable(EXPRESSION_BOOLEEN.to_string()));
	};
	let current_index = past_index + 1;
	let mut expression_index: usize = 0;
	
	match expression[0] {
		Element::Booleen(booleen) => {
			Ok(Instruction {
				body: format!("%{}-{}-fin = add i1 {}, 0\n", EXPRESSION_BOOLEEN, current_index, booleen as i32),
				var: [(EXPRESSION_BOOLEEN.to_string(), current_index)].into_iter().collect(),
				declaration: String::new(),
			})
		},
		_ => Err(ErreurMorgan::MauvaisArgument("expression booléene complexe".to_string()))
	}
}
