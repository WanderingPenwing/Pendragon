use super::*;

pub fn calcule_texte(
    expression: Vec<Element>,
    variables: &HashMap<String, Element>,
) -> Result<String, ErreurPendragon> {
    let mut pile: Vec<Element> = Vec::new();
    let mut texte: String = String::new();

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
            texte += &booleen::affiche_booleen(pile.clone(), variables)?;
            pile = Vec::new();
            continue;
        }
        if let TypeElement::Entier = element_pile.type_element() {
            texte += &nombre::affiche_nombre(pile.clone(), variables)?;
            pile = Vec::new();
            continue;
        }
        match element_pile {
            Element::Texte(contenu) => texte += contenu,
            Element::Variable(nom, type_element) => {
                let Some(variable) = variables.get(nom) else {
                    return Err(ErreurPendragon::VariableInconnue(nom.into()));
                };
                let Element::Texte(contenu) = variable else {
                    return Err(ErreurPendragon::MauvaisType(
                        nom.into(),
                        type_element.nom(),
                        "texte".into(),
                    ));
                };
                texte += contenu;
            }
            autre => return Err(ErreurPendragon::MauvaisArgument(format!("{}", autre))),
        }
        pile = Vec::new();
    }
    Ok(texte)
}
