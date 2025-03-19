use super::*;

pub fn calcule_nombre(
	expression: Vec<Element>,
	var: HashMap<String, usize>,
) -> Result<Instruction, ErreurMorgan> {
	if expression.len() != 1 {
		return Err(ErreurMorgan::MauvaisArgument("expression numérique complexe".to_string()));
	}
	let Some(past_index) = var.get(EXPRESSION_NOMBRE) else {
		return Err(ErreurMorgan::ManqueVariable(EXPRESSION_NOMBRE.to_string()));
	};
	let current_index = past_index + 1;
	let mut expression_index: usize = 0;
	
	match expression[0] {
		Element::Entier(nombre) => {
			Ok(Instruction {
				body: format!("%{}-{}-fin = add i64 {}, 0\n", EXPRESSION_NOMBRE, current_index, nombre),
				var: [(EXPRESSION_NOMBRE.to_string(), current_index)].into_iter().collect(),
				declaration: String::new(),
			})
		},
		_ => Err(ErreurMorgan::MauvaisArgument("expression numérique complexe".to_string()))
	}
}
