use super::*;

pub fn calcule_texte(
    expression: Vec<Element>,
) -> Result<Instruction, ErreurMorgan> {
    let mut pile: Vec<Element> = Vec::new();

    let mut instruction = Instruction::default();

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
            instruction.add(booleen::affiche_booleen(pile.clone())?);
            pile = Vec::new();
            continue;
        }
        if let TypeElement::Entier = element_pile.type_element() {
            instruction.add(nombre::affiche_nombre(pile.clone())?);
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
