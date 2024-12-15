use super::*;
use crate::nombre::*;

pub fn affiche_nombre(
    expression: Vec<Element>,
    variables: &HashMap<String, Element>,
) -> Result<String, ErreurPendragon> {
    let nombre = calcule_nombre(expression.clone(), variables)?;
    Ok(nombre_comme_texte(nombre))
}

pub fn calcule_nombre(
    expression: Vec<Element>,
    variables: &HashMap<String, Element>,
) -> Result<usize, ErreurPendragon> {
    let mut pile: Vec<usize> = Vec::new();

    for element in expression {
        if let Element::Entier(nombre) = element {
            pile.push(nombre);
            continue;
        }
        if let Element::Variable(nom, _) = element {
            let Some(variable) = variables.get(&nom) else {
                return Err(ErreurPendragon::VariableInconnue(nom));
            };
            if let Element::Entier(nombre) = variable {
                pile.push(*nombre);
                continue;
            } else {
                return Err(ErreurPendragon::MauvaisType(
                    nom,
                    variable.type_element().nom(),
                    "entier".into(),
                ));
            }
        }
        let Element::Operateur(ref operateur) = element else {
            return Err(ErreurPendragon::MauvaisArgument(format!(
                "{}, attendais un opérateur",
                element
            )));
        };
        let Some(nombre_a) = pile.pop() else {
            return Err(ErreurPendragon::CalculEntier("la pile est vide".into()));
        };
        let Some(nombre_b) = pile.pop() else {
            return Err(ErreurPendragon::CalculEntier("la pile est vide".into()));
        };
        match operateur {
            Operateur::Plus => {
                pile.push(nombre_b + nombre_a);
            }
            Operateur::Moins => {
                if nombre_b < nombre_a {
                    return Err(ErreurPendragon::CalculEntier(format!(
                        "a essayé de soustraire '{}' à '{}'",
                        nombre_comme_texte(nombre_a),
                        nombre_comme_texte(nombre_b)
                    )));
                }
                pile.push(nombre_b - nombre_a);
            }
            Operateur::Fois => {
                pile.push(nombre_b * nombre_a);
            }
            Operateur::Divise => {
                pile.push(nombre_b / nombre_a);
            }
            _ => {
                return Err(ErreurPendragon::MauvaisArgument(format!(
                    "'{}', attendais un opérateur d'entiers",
                    element
                )))
            }
        }
    }
    if pile.len() > 1 {
        return Err(ErreurPendragon::CalculEntier(
            "la pile n'est pas vide".into(),
        ));
    }
    Ok(pile[0])
}

pub fn nombre_comme_texte(nombre: usize) -> String {
    if nombre == 0 {
        return "zéro".to_string();
    }
    if nombre >= 10usize.pow(18) {
        return "infini".to_string();
    }
    let mut groupes: Vec<usize> = vec![];
    let mut nombre = nombre;
    while nombre > 0 {
        groupes.insert(0, nombre % 1000);
        nombre /= 1000;
    }
    let mut chaine: String = "".to_string();

    for index in 0..groupes.len() {
        if groupes[index] == 0 {
            continue;
        }
        let pluriel: &str = if (groupes.len() - index - 1 > 1) && groupes[index] > 1 {
            "s"
        } else {
            ""
        };
        if index < groupes.len() - 1 {
            let union = if index > 0 { UNION } else { "" };
            let chiffre = if groupes.len() - index - 1 == 1 && groupes[index] == 1 {
                // un mille
                "".to_string()
            } else {
                petit_nombre_comme_texte(groupes[index]) + UNION
            };
            chaine += &format!(
                "{}{}{}{}",
                union,
                chiffre,
                NOMS_SEPARATEURS[groupes.len() - index - 1],
                pluriel,
            );
        } else {
            let union = if index > 0 { UNION } else { "" };
            chaine += union;
            chaine += &petit_nombre_comme_texte(groupes[index]);
        }
    }
    chaine
}

fn petit_nombre_comme_texte(nombre: usize) -> String {
    let nombre = nombre.clamp(0, 999);
    let centaine = nombre / 100;
    let dizaine = (nombre % 100) / 10;
    let unité = nombre % 10;

    let centaine_texte = if centaine > 1 {
        format!("{}{}cent", NOMS_UNITES[centaine], UNION)
    } else if centaine > 0 {
        "cent".to_string()
    } else {
        "".to_string()
    };

    let décalage_dizaine = if [1, 7, 9].contains(&dizaine) { 1 } else { 0 };
    let dizaine_texte = NOMS_DIZAINES[dizaine - décalage_dizaine];
    let séparation = if unité == 1 && ![0, 1, 8, 9].contains(&dizaine) {
        UNION.to_string() + "et"
    } else {
        "".to_string()
    };
    let unité_texte = if [1, 7, 9].contains(&dizaine) {
        NOMS_UNITES_DIX[unité]
    } else {
        NOMS_UNITES[unité]
    };

    let mut texte_nombre = format!(
        "{}{}{}{}{}{}",
        centaine_texte, UNION, dizaine_texte, séparation, UNION, unité_texte
    );

    while texte_nombre.contains("--") {
        texte_nombre = texte_nombre.replace("--", "-");
    }
    if texte_nombre.starts_with("-") {
        texte_nombre = texte_nombre[1..texte_nombre.len()].to_string();
    }
    if texte_nombre.ends_with("-") {
        texte_nombre = texte_nombre[0..texte_nombre.len() - 1].to_string();
    }
    texte_nombre
}
