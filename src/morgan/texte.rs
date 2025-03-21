use super::*;

pub fn calcule_texte(
	expression: Vec<Element>,
	var: HashMap<String, usize>,
) -> Result<Instruction, ErreurMorgan> {
	let mut pile: Vec<Element> = Vec::new();

	let mut instruction = Instruction::new(var);
	let current_index = instruction.var.entry(EXPRESSION_TEXTE.to_string()).and_modify(|e| *e += 1).or_insert(0).clone();
	let mut expression_index: usize = 0;
	instruction.body += &format!("%{}-{}-0 = getelementptr [1 x i8], [1 x i8]* @vide, i32 0, i32 0\n", EXPRESSION_TEXTE, current_index);

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
			expression_index += 1;
			instruction.body += &format!("%{}-{}-{} = call i8* @texte_booleen(i1 %{}-{}-fin)\n", 
				EXPRESSION_TEXTE, current_index, expression_index, 
				EXPRESSION_BOOLEEN, current_bool_index
			);
			expression_index += 1;
			instruction.body += &format!("%{}-{}-{} = call i8* @concat_strings(i8* %{}-{}-{}, i8* %{}-{}-{})\n", 
				EXPRESSION_TEXTE, current_index, expression_index, 
				EXPRESSION_TEXTE, current_index, expression_index-2, 
				EXPRESSION_TEXTE, current_index, expression_index-1
			);
			pile = Vec::new();
			continue;
		}
		if let TypeElement::Entier = element_pile.type_element() {
			instruction.add(nombre::calcule_nombre(pile.clone(), instruction.var.clone())?);
			let Some(current_numb_index) = instruction.var.get(EXPRESSION_NOMBRE) else {
				return Err(ErreurMorgan::ManqueVariable(EXPRESSION_NOMBRE.to_string()));
			};
			expression_index += 1;
			instruction.body += &format!("%{}-{}-{} = call i8* @texte_nombre(i64 %{}-{}-fin)\n", 
				EXPRESSION_TEXTE, current_index, expression_index, 
				EXPRESSION_NOMBRE, current_numb_index
			);
			expression_index += 1;
			instruction.body += &format!("%{}-{}-{} = call i8* @concat_strings(i8* %{}-{}-{}, i8* %{}-{}-{})\n", 
				EXPRESSION_TEXTE, current_index, expression_index, 
				EXPRESSION_TEXTE, current_index, expression_index-2, 
				EXPRESSION_TEXTE, current_index, expression_index-1
			);
			pile = Vec::new();
			continue;
		}
		match element_pile {
			Element::Texte(_contenu) => return Err(ErreurMorgan::MauvaisArgument("texte pas géré".to_string())),
//			Element::Variable(nom, type_element) => {
//				let current_variable_index = instruction.var[nom];
//				let call = match type_element {
//					TypeElement::Entier => "call void @affiche_nombre(i64",
//					TypeElement::Booleen => "call void @affiche_booleen(i1",
//					TypeElement::Texte => return Err(ErreurMorgan::MauvaisArgument("var texte pas implémenté".to_string())),
//				};
//				instruction.body += &format!("{} %{}-{})\n", call, nom, current_variable_index);
//			}
			autre => return Err(ErreurMorgan::MauvaisArgument(format!("{}", autre))),
		}
		pile = Vec::new();
	}
	instruction.body += &format!("%{}-{}-fin = getelementptr i8, i8* %{}-{}-{}, i32 0\n", 
		EXPRESSION_TEXTE, current_index, EXPRESSION_TEXTE, current_index, expression_index
	);
		
	Ok(instruction)
}
