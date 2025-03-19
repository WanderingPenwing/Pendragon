use super::*;

pub fn calcule_texte(
	expression: Vec<Element>,
	var: HashMap<String, usize>,
) -> Result<Instruction, ErreurMorgan> {
	let mut pile: Vec<Element> = Vec::new();

	let mut instruction = Instruction::new(var);
	
//	let Some(past_index) = instruction.var.get(EXPRESSION_TEXTE) else {
//		return Err(ErreurMorgan::ManqueVariable(EXPRESSION_TEXTE.to_string()));
//	}
//	let current_index = past_index + 1;
//	let mut expression_index: usize = 0;

	for element in expression {
		let Element::Operateur(ref operateur) = element else {
			pile.push(element);
			continue;
		};
		let Operateur::Puis = operateur else {
			pile.push(element);
			continue;
		};
		let Some(element_pile) = pile.last() else {
			continue;
		};
		if let TypeElement::Booleen = element_pile.type_element() {
			instruction.add(booleen::calcule_booleen(pile.clone(), instruction.var.clone())?);
			let Some(current_bool_index) = instruction.var.get(EXPRESSION_BOOLEEN) else {
				return Err(ErreurMorgan::ManqueVariable(EXPRESSION_BOOLEEN.to_string()));
			};
			instruction.body += &format!("call void @affiche_booleen(i1 %{}-{}-fin)\n", EXPRESSION_BOOLEEN, current_bool_index);
			pile = Vec::new();
			continue;
		}
		if let TypeElement::Entier = element_pile.type_element() {
			instruction.add(nombre::calcule_nombre(pile.clone(), instruction.var.clone())?);
			let Some(current_numb_index) = instruction.var.get(EXPRESSION_NOMBRE) else {
				return Err(ErreurMorgan::ManqueVariable(EXPRESSION_NOMBRE.to_string()));
			};
			instruction.body += &format!("call void @affiche_nombre(i64 %{}-{}-fin)\n", EXPRESSION_NOMBRE, current_numb_index);
			pile = Vec::new();
			continue;
		}
		match element_pile {
			Element::Texte(_contenu) => {},
			Element::Variable(_nom, _type_element) => {}
			autre => return Err(ErreurMorgan::MauvaisArgument(format!("{}", autre))),
		}
		pile = Vec::new();
	}
	instruction.body += "call void @nouvelle_ligne()\n";
	Ok(instruction)
}
