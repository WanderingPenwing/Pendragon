use super::*;

pub fn affiche_booleen(
    expression: Vec<Element>
) -> Result<Instruction, ErreurMorgan> {
	if expression.len() != 1 {
		return Err(ErreurMorgan::MauvaisArgument("expression booléene complexe".to_string()));
	}
	match expression[0] {
		Element::Booleen(booleen) => Ok(Instruction {body: format!("call void @affiche_booleen(i1 {})\n", booleen as i32)}),
		_ => Err(ErreurMorgan::MauvaisArgument("expression booléene complexe".to_string()))
	}
}
