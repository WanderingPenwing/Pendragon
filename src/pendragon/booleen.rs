use super::*;

impl Pendragon {
    pub fn elements_booleen(&self, arguments: &str) -> Result<Vec<Element>, ErreurPendragon> {
        let texte = arguments
            .replace("ouvre la parenthèse", "ouvre-la-parenthese")
            .replace("ferme la parenthèse", "ferme-la-parenthese")
            .replace("est égal à", "est-egal-a")
            .replace("est différent de", "est-different-de")
            .replace("est supérieur ou égal à", "est-superieur-ou-egal-a")
            .replace("est inférieur ou égal à", "est-inferieur-ou-egal-a")
            .replace("est supérieur à", "est-superieur-a")
            .replace("est inférieur à", "est-inferieur-a")
            .replace("divisé par", "divise-par");
        let elements_texte: Vec<&str> = texte.split(" ").collect();
        let mut expression: Vec<Element> = Vec::new();
        let mut pile_operateurs: Vec<Operateur> = Vec::new();

        let mut pile_inconnu: Vec<String> = Vec::new();
        let mut possible_comparaison: Option<Comparaison> = None;
        let mut precede_par_operation: bool = true;

        for (index, element) in elements_texte.iter().enumerate() {
            let element_precedent = if index > 0 {
                elements_texte[index - 1]
            } else {
                "le début"
            };
            let element: &str = element;
            match element {
                "vrai" => {
                    self.fin_comparaison(
                        "vrai",
                        &mut pile_inconnu,
                        &mut pile_operateurs,
                        &mut expression,
                        &mut possible_comparaison,
                    )?;
                    expression.push(Element::Booleen(true));
                    if !precede_par_operation {
                        return Err(ErreurPendragon::OrdreCalculBooleen(
                            "opérateur".into(),
                            element_precedent.into(),
                            "vrai".into(),
                        ));
                    }
                    precede_par_operation = false;
                    continue;
                }
                "faux" => {
                    self.fin_comparaison(
                        "faux",
                        &mut pile_inconnu,
                        &mut pile_operateurs,
                        &mut expression,
                        &mut possible_comparaison,
                    )?;
                    expression.push(Element::Booleen(false));
                    if !precede_par_operation {
                        return Err(ErreurPendragon::OrdreCalculBooleen(
                            "opérateur".into(),
                            element_precedent.into(),
                            "faux".into(),
                        ));
                    }
                    precede_par_operation = false;
                    continue;
                }
                "non" => {
                    self.fin_comparaison(
                        "non",
                        &mut pile_inconnu,
                        &mut pile_operateurs,
                        &mut expression,
                        &mut possible_comparaison,
                    )?;
                    pile_operateurs.push(Operateur::Non);
                    if !precede_par_operation {
                        return Err(ErreurPendragon::OrdreCalculBooleen(
                            "opérateur".into(),
                            element_precedent.into(),
                            "non".into(),
                        ));
                    }
                    continue;
                }
                "et" => {
                    self.fin_comparaison(
                        "et",
                        &mut pile_inconnu,
                        &mut pile_operateurs,
                        &mut expression,
                        &mut possible_comparaison,
                    )?;
                    while let Some(operateur) = pile_operateurs.last() {
                        if *operateur == Operateur::Non || *operateur == Operateur::Et {
                            expression.push(Element::Operateur(pile_operateurs.pop().unwrap()));
                        } else {
                            break;
                        }
                    }
                    pile_operateurs.push(Operateur::Et);
                }
                "ou" => {
                    self.fin_comparaison(
                        "ou",
                        &mut pile_inconnu,
                        &mut pile_operateurs,
                        &mut expression,
                        &mut possible_comparaison,
                    )?;
                    while let Some(operateur) = pile_operateurs.last() {
                        if *operateur == Operateur::Non
                            || *operateur == Operateur::Et
                            || *operateur == Operateur::Ou
                        {
                            expression.push(Element::Operateur(pile_operateurs.pop().unwrap()));
                        } else {
                            break;
                        }
                    }
                    pile_operateurs.push(Operateur::Ou);
                }
                "ouvre-la-parenthese" => {
                    if !precede_par_operation && !pile_inconnu.is_empty() {
                        return Err(ErreurPendragon::OrdreCalculBooleen(
                            "opérateur".into(),
                            element_precedent.into(),
                            "l'ouverture de la parenthèse".into(),
                        ));
                    }
                    pile_inconnu.push("ouvre-la-parenthese".into());
                    continue;
                }
                "ferme-la-parenthese" => {
                    if precede_par_operation {
                        return Err(ErreurPendragon::OrdreCalculBooleen(
                            "booleen".into(),
                            element_precedent.into(),
                            "la fermeture de la parenthèse".into(),
                        ));
                    }
                    let nombre_parenthese = compare_parentheses(&pile_inconnu);
                    if nombre_parenthese.0 > nombre_parenthese.1 {
                        pile_inconnu.push("ferme-la-parenthese".into());
                        continue;
                    }
                    self.fin_comparaison(
                        "ferme-la-parenthese",
                        &mut pile_inconnu,
                        &mut pile_operateurs,
                        &mut expression,
                        &mut possible_comparaison,
                    )?;
                    while let Some(operateur) = pile_operateurs.pop() {
                        if operateur == Operateur::ParentheseBooleen {
                            break;
                        }
                        expression.push(Element::Operateur(operateur));
                    }
                    continue;
                }
                autre => {
                    if format_de_variable(autre) {
                        if self
                            .programme
                            .variable_est_de_type(autre, TypeElement::Booleen)
                            .is_ok()
                        {
                            self.fin_comparaison(
                                autre,
                                &mut pile_inconnu,
                                &mut pile_operateurs,
                                &mut expression,
                                &mut possible_comparaison,
                            )?;
                            expression.push(Element::Variable(autre.into(), TypeElement::Booleen));
                        } else {
                            pile_inconnu.push(autre.into());
                        }
                    } else if let Ok(type_comparaison) = texte_comme_comparaison(autre) {
                        if let Some(comparaison) = possible_comparaison {
                            return Err(ErreurPendragon::ComparaisonInvalide(format!("il manque un operateur booleen entre les comparaisons '{}' et '{}'", comparaison, type_comparaison)));
                        }
                        let mut comparaison = Comparaison::nouvelle();
                        let nombre_parenthese = compare_parentheses(&pile_inconnu);
                        if pile_inconnu[0] == "ouvre-la-parenthese"
                            && nombre_parenthese.0 > nombre_parenthese.1
                        {
                            pile_inconnu.remove(0);
                            pile_operateurs.push(Operateur::ParentheseBooleen);
                        }
                        self.ajoute_comparaison_membre(&mut comparaison, &pile_inconnu.join(" "))?;
                        comparaison.ajoute_type(type_comparaison)?;
                        possible_comparaison = Some(comparaison);
                        pile_inconnu = Vec::new();
                    } else {
                        pile_inconnu.push(autre.into());
                    }
                    precede_par_operation = false;
                    continue;
                }
            }
            if precede_par_operation {
                return Err(ErreurPendragon::OrdreCalculBooleen(
                    "booleen".into(),
                    element_precedent.into(),
                    element.into(),
                ));
            }
            precede_par_operation = true;
        }
        if !pile_inconnu.is_empty() {
            let Some(mut comparaison) = possible_comparaison else {
                return Err(ErreurPendragon::BooleenInvalide(format!(
                    "[{}]",
                    pile_inconnu.join(",")
                )));
            };
            self.ajoute_comparaison_membre(&mut comparaison, &pile_inconnu.join(" "))?;
            expression.push(Element::Comparaison(comparaison.clone()));
        }

        while let Some(operateur) = pile_operateurs.pop() {
            expression.push(Element::Operateur(operateur));
        }

        Ok(expression)
    }

    pub fn fin_comparaison(
        &self,
        _element: &str,
        pile_inconnu: &mut Vec<String>,
        pile_operateurs: &mut Vec<Operateur>,
        expression: &mut Vec<Element>,
        possible_comparaison: &mut Option<Comparaison>,
    ) -> Result<(), ErreurPendragon> {
        if pile_inconnu.len() == 1 && pile_inconnu[0] == "ouvre-la-parenthese" {
            pile_operateurs.push(Operateur::ParentheseBooleen);
            *pile_inconnu = Vec::new();
        }
        if pile_inconnu.is_empty() {
            return Ok(());
        }
        let Some(ancienne_comparaison) = possible_comparaison else {
            return Err(ErreurPendragon::BooleenInvalide(format!(
                "[{}]",
                pile_inconnu.join(",")
            )));
        };
        let mut comparaison = ancienne_comparaison.clone();
        self.ajoute_comparaison_membre(&mut comparaison, &pile_inconnu.join(" "))?;
        expression.push(Element::Comparaison(comparaison.clone()));
        *pile_inconnu = Vec::new();
        *possible_comparaison = None;
        Ok(())
    }

    pub fn ajoute_comparaison_membre(
        &self,
        comparaison: &mut Comparaison,
        texte: &str,
    ) -> Result<(), ErreurPendragon> {
        let mut membre: Vec<Element> = vec![];
        match self.elements_nombre(texte) {
            Ok(elements_nombre) => membre = elements_nombre,
            Err(raison) => {
                if let ErreurPendragon::OrdreCalculEntier(_, _, _) = raison {
                    return Err(raison);
                }
            }
        }
        if membre.is_empty() {
            match self.elements_booleen(texte) {
                Ok(elements_booleen) => membre = elements_booleen,
                Err(raison) => {
                    if let ErreurPendragon::OrdreCalculBooleen(_, _, _) = raison {
                        return Err(raison);
                    }
                }
            }
        }
        if membre.is_empty() {
            if let Ok(elements_texte) = self.elements_texte(texte) {
                membre = elements_texte;
            }
        }
        let Some(element) = membre.first() else {
            return Err(ErreurPendragon::MauvaisArgument(texte.to_string()));
        };
        if comparaison.type_comparaison.is_none() {
            comparaison.membre_a = membre;
            return Ok(());
        }
        let Some(element_de_comparaison) = comparaison.membre_a.first() else {
            return Err(ErreurPendragon::ComparaisonInvalide(
                "il n'y a pas de premier membre".into(),
            ));
        };
        if element_de_comparaison.type_element() != element.type_element() {
            return Err(ErreurPendragon::MauvaisType(
                format!("{}", element),
                element.type_element().nom(),
                element_de_comparaison.type_element().nom(),
            ));
        }
        comparaison.membre_b = membre;
        Ok(())
    }
}

fn compare_parentheses(strings: &[String]) -> (usize, usize) {
    let ouvre_count = strings
        .iter()
        .filter(|s| *s == "ouvre-la-parenthese")
        .count();
    let ferme_count = strings
        .iter()
        .filter(|s| *s == "ferme-la-parenthese")
        .count();

    (ouvre_count, ferme_count)
}

pub fn texte_comme_booleen(texte: &str) -> Result<Element, ErreurPendragon> {
    match texte {
        "vrai" => Ok(Element::Booleen(true)),
        "faux" => Ok(Element::Booleen(false)),
        _ => Err(ErreurPendragon::BooleenInvalide(texte.into())),
    }
}

pub fn texte_comme_comparaison(texte: &str) -> Result<TypeComparaison, ErreurPendragon> {
    match texte {
        "est-egal-a" => Ok(TypeComparaison::Egal),
        "est-different-de" => Ok(TypeComparaison::Different),
        "est-superieur-ou-egal-a" => Ok(TypeComparaison::SuperieurEgal),
        "est-inferieur-ou-egal-a" => Ok(TypeComparaison::InferieurEgal),
        "est-superieur-a" => Ok(TypeComparaison::Superieur),
        "est-inferieur-a" => Ok(TypeComparaison::Inferieur),
        _ => Err(ErreurPendragon::ComparaisonInvalide(format!(
            "\"{}\" n'est pas un type de comparaison",
            texte
        ))),
    }
}
