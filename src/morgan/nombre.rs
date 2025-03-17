use super::*;

pub fn affiche_nombre(
    expression: Vec<Element>
) -> Result<Instruction, ErreurMorgan> {
	if expression.len() != 1 {
		return Err(ErreurMorgan::MauvaisArgument("expression numérique complexe".to_string()));
	}
	match expression[0] {
		Element::Entier(nombre) => Ok(Instruction {body: format!("call void @affiche_nombre(i64 {})\n", nombre)}),
		_ => Err(ErreurMorgan::MauvaisArgument("expression numérique complexe".to_string()))
	}
}
