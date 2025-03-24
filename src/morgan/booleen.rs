use super::*;

pub fn calcule_booleen(
	expression: Vec<Element>,
	var: HashMap<String, usize>,
) -> Result<Instruction, ErreurMorgan> {
	let mut instruction = Instruction::new(var);
	let current_index = instruction.var.entry(EXPRESSION_BOOLEEN.to_string()).and_modify(|e| *e += 1).or_insert(0).clone();
	let mut expression_index: usize = 0;
	let mut booleens: Vec<usize> = Vec::new();
	
	for element in expression {
		if let Element::Booleen(booleen) = element {
			expression_index += 1;
			instruction.body += &format!("\t%{}-{}-{} = add i1 {}, 0\n", EXPRESSION_BOOLEEN, current_index, expression_index, booleen as i32);
			booleens.push(expression_index);
			continue
		}
		if let Element::Variable(nom, _) = element {
			expression_index += 1;
			let Some(&current_var_index) = instruction.var.get(&nom) else {
				return Err(ErreurMorgan::ManqueVariable(nom.to_string()));
			};
			instruction.body += &format!("\t%{}-{}-{} = add i1 %{}-{}, 0\n", 
				EXPRESSION_BOOLEEN, current_index, expression_index,
				nom, current_var_index);
			booleens.push(expression_index);
			continue
		}
		if let Element::Comparaison(comparaison) = element {
			instruction.add(comparaison.instructions(instruction.var.clone())?);
			let Some(&current_comparaison_index) = instruction.var.get(COMPARAISON) else {
				return Err(ErreurMorgan::ManqueVariable(COMPARAISON.to_string()));
			};
			expression_index += 1;
			instruction.body += &format!("\t%{}-{}-{} = add i1 %{}-{}, 0\n", EXPRESSION_BOOLEEN, current_index, expression_index, COMPARAISON, current_comparaison_index);
			booleens.push(expression_index);
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
				instruction.body += &format!("\t%{}-{}-{} = xor i1 %{}-{}-{}, true\n", 
					EXPRESSION_BOOLEEN, current_index, expression_index, 
					EXPRESSION_BOOLEEN, current_index, booleens[booleens.len()-1]);
				booleens.pop();
			}
			Operateur::Et => {
				expression_index += 1;
				instruction.body += &format!("\t%{}-{}-{} = and i1 %{}-{}-{}, %{}-{}-{}\n", 
					EXPRESSION_BOOLEEN, current_index, expression_index, 
					EXPRESSION_BOOLEEN, current_index, booleens[booleens.len()-2],
					EXPRESSION_BOOLEEN, current_index, booleens[booleens.len()-1]);
				booleens.pop();
				booleens.pop();
			}
			Operateur::Ou => {
				expression_index += 1;
				instruction.body += &format!("\t%{}-{}-{} = or i1 %{}-{}-{}, %{}-{}-{}\n", 
					EXPRESSION_BOOLEEN, current_index, expression_index, 
					EXPRESSION_BOOLEEN, current_index, booleens[booleens.len()-2],
					EXPRESSION_BOOLEEN, current_index, booleens[booleens.len()-1]);
				booleens.pop();
				booleens.pop();
			}
			_ => {
				return Err(ErreurMorgan::MauvaisArgument(format!(
					"{}, attendais un opérateur booléen",
					element
				)))
			}
		}
		booleens.push(expression_index);
	}
	if expression_index > 0 {
		instruction.body += &format!("\t%{}-{}-fin = add i1 %{}-{}-{}, 0\n", 
			EXPRESSION_BOOLEEN, current_index, 
			EXPRESSION_BOOLEEN, current_index, expression_index);
	}
	
	Ok(instruction)
}

impl Comparaison {
	pub fn instructions(&self, var: HashMap<String, usize>,) -> Result<Instruction, ErreurMorgan> {
		let Some(ref comparaison) = self.type_comparaison else {
			return Err(ErreurMorgan::ComparaisonInvalide(
				"la comparaison n'a pas de type".into(),
			));
		};
		let Some(element) = self.membre_a.first() else {
			return Err(ErreurMorgan::ComparaisonInvalide(
				"il n'y a pas de premier membre".into(),
			));
		};
		
		let mut instruction = Instruction::new(var);
		let current_index = instruction.var.entry(COMPARAISON.to_string()).and_modify(|e| *e += 1).or_insert(0).clone();
		
		match element.type_element() {
			TypeElement::Entier => {
				instruction.add(nombre::calcule_nombre(self.membre_a.clone(), instruction.var.clone())?);
				let membre_a_index = instruction.var[EXPRESSION_NOMBRE];
				instruction.add(nombre::calcule_nombre(self.membre_b.clone(), instruction.var.clone())?);
				let membre_b_index = instruction.var[EXPRESSION_NOMBRE];
				
				let comparaison_ir: &str = match comparaison {
					TypeComparaison::Egal => "eq",
					TypeComparaison::Different => "ne",
					TypeComparaison::SuperieurEgal => "sge",
					TypeComparaison::InferieurEgal => "sle",
					TypeComparaison::Superieur => "sgt",
					TypeComparaison::Inferieur => "slt",
				};
				
				instruction.body += &format!("\t%{}-{} = icmp {} i64 %{}-{}-fin, %{}-{}-fin\n", 
					COMPARAISON, current_index, comparaison_ir, EXPRESSION_NOMBRE, membre_a_index, EXPRESSION_NOMBRE, membre_b_index
				);
				
			}
			TypeElement::Texte => {
//				return Err(ErreurMorgan::ComparaisonInvalide("texte pas implémenté".to_string()));
				instruction.add(texte::calcule_texte(self.membre_a.clone(), instruction.var.clone())?);
				let membre_a_index = instruction.var[EXPRESSION_TEXTE];
				instruction.add(texte::calcule_texte(self.membre_b.clone(), instruction.var.clone())?);
				let membre_b_index = instruction.var[EXPRESSION_TEXTE];
				
				let egalite: usize = match comparaison {
					TypeComparaison::Egal => 1,
					TypeComparaison::Different => 0,
					_ => return Err(ErreurMorgan::ComparaisonInvalide(format!("{} de textes", comparaison))),
				};
				
				instruction.body += &format!("\t%{}-{} = call i1 @compare_texte(i8* %{}-{}-fin, i8* %{}-{}-fin, i1 {})\n", 
					COMPARAISON, current_index, EXPRESSION_TEXTE, membre_a_index, EXPRESSION_TEXTE, membre_b_index, egalite
				);
				
				
			}
			TypeElement::Booleen => {
				instruction.add(booleen::calcule_booleen(self.membre_a.clone(), instruction.var.clone())?);
				let membre_a_index = instruction.var[EXPRESSION_BOOLEEN];
				instruction.add(booleen::calcule_booleen(self.membre_b.clone(), instruction.var.clone())?);
				let membre_b_index = instruction.var[EXPRESSION_BOOLEEN];
				
				let comparaison_ir: &str = match comparaison {
					TypeComparaison::Egal => "eq",
					TypeComparaison::Different => "ne",
					_ => return Err(ErreurMorgan::ComparaisonInvalide(format!("{} de booléens", comparaison))),
				};
				
				instruction.body += &format!("\t%{}-{} = icmp {} i1 %{}-{}-fin, %{}-{}-fin\n", 
					COMPARAISON, current_index, comparaison_ir, EXPRESSION_BOOLEEN, membre_a_index, EXPRESSION_BOOLEEN, membre_b_index
				);
			}
		};
		Ok(instruction)
	}
}

