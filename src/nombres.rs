use super::ErreurSophie;

const NOMS_UNITES: [&str; 10] = ["", "un", "deux", "trois", "quatre", "cinq", "six", "sept", "huit", "neuf"];
const NOMS_UNITES_DIX: [&str; 10] = ["dix", "onze", "douze", "treize", "quatorze", "quinze", "seize", "dix-sept", "dix-huit", "dix-neuf"];
const NOMS_DIZAINES: [&str; 9] = ["", "dix", "vingt", "trente", "quarante", "cinquante", "soixante", "x", "quatre-vingts"];
const NOMS_SEPARATEURS: [&str; 7] = ["", "mille", "million", "milliard", "billion", "billiard", "trillion"];
const UNION: &str = "-";

pub fn nombre_comme_texte(nombre: usize) -> String {
	if nombre == 0 {
		return "zéro".to_string()
	}
	if nombre >= 10usize.pow(18) {
		return "infini".to_string()
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
			continue
		}
		let pluriel: &str = if (groupes.len() - index - 1 > 1) && groupes[index] > 1 {
			"s"
		} else {
			""
		};
		if index < groupes.len() - 1 {
			let union = if index > 0 {UNION} else {""};
			let chiffre = if groupes.len() - index - 1 == 1 && groupes[index] == 1 { // un mille
				"".to_string()
			} else {
				petit_nombre_comme_texte(groupes[index]) + UNION
			};
			chaine += &format!("{}{}{}{}",
				union,
				chiffre,
				NOMS_SEPARATEURS[groupes.len() - index - 1],
				pluriel,
			);
		} else {
			let union = if index > 0 {UNION} else {""};
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
	
	let décalage_dizaine = if [1, 7, 9].contains(&dizaine) {1} else {0};
	
	let centaine_texte = if centaine > 1 {
		format!("{}{}cent", NOMS_UNITES[centaine], UNION)
	} else if centaine > 0 {
		"cent".to_string()
	} else {
		"".to_string()
	};

	let dizaine_union = if centaine > 0 && dizaine > 0 {
		UNION.to_string()
	} else {
		"".to_string()
	};

	let dizaine_texte = NOMS_DIZAINES[dizaine - décalage_dizaine];

	let séparation = if unité == 1 && ![0, 1, 8, 9].contains(&dizaine) {UNION.to_string() + "et"} else {"".to_string()};

	let unité_union = if nombre - unité > 0 && unité > 0 && nombre > 16 {
		UNION.to_string()
	} else {
		"".to_string()
	};
	let unité_texte = if [1, 7, 9].contains(&dizaine) {
		unité_union + NOMS_UNITES_DIX[unité]
	} else {
		unité_union + NOMS_UNITES[unité]
	};
	
	format!("{}{}{}{}{}", centaine_texte, dizaine_union, dizaine_texte, séparation, unité_texte)
}

pub fn texte_comme_nombre(texte: &str) -> Result<usize, ErreurSophie> {
	if texte == "zéro" {
		return Ok(0)
	}
	let pluriel = format!("s{}", UNION);
	let mut petits_nombres_texte: Vec<&str> = vec![];
	let mut texte_modifie = texte;
	for (index, separateur_texte) in NOMS_SEPARATEURS.iter().rev().enumerate() {
		if index == NOMS_SEPARATEURS.len() - 1 {
			continue
		}
		let mut un_mille: bool = false;
		if texte_modifie.starts_with("mille") && separateur_texte == &"mille" {
			un_mille = true;
		}
		let texte_separe: Vec<&str> = texte_modifie.split(separateur_texte).collect();
		if texte_separe.len() > 2 {
			return Err(ErreurSophie::OrthographeNombre(texte.to_string()))
		}
		if texte_separe.len() > 1 {
			let petit_nombre_texte = texte_separe[0]
				.trim_start_matches(&pluriel)
				.trim_start_matches(UNION)
				.trim_end_matches(UNION);
			if un_mille {
				petits_nombres_texte.push("un");
			} else {
				petits_nombres_texte.push(petit_nombre_texte);
			}
			texte_modifie = texte_separe[1].trim_start_matches(UNION);
		}
	}
	let petit_nombre_texte = texte_modifie
			.trim_start_matches(&pluriel)
			.trim_start_matches(UNION)
			.trim_end_matches(UNION);
	petits_nombres_texte.push(petit_nombre_texte);

	let mut nombre: usize = 0;

	for (index, petit_nombre_texte) in petits_nombres_texte.iter().enumerate() {
		let petit_nombre = texte_comme_petit_nombre(petit_nombre_texte)?;
		nombre += petit_nombre * 1000usize.pow((petits_nombres_texte.len() - index - 1) as u32);
	}
	
	Ok(nombre)
}

fn texte_comme_petit_nombre(texte: &str) -> Result<usize, ErreurSophie> {
	let elements: Vec<&str> = texte.split(UNION).collect();

	let mut nombre = 0;
	let mut dernier_chiffre_texte = "";

	for chiffre_texte in elements {
		if chiffre_texte == "cent" {
			if nombre == 0 {
				nombre = 1;
			}
			if nombre >= 100 {
				return Err(ErreurSophie::OrthographeNombre(texte.to_string()))
			}
			nombre *= 100;
			dernier_chiffre_texte = chiffre_texte;
			continue
		}
		if chiffre_texte == "vingts" {
			if dernier_chiffre_texte == "quatre" {
				nombre += 80 - 4;
				dernier_chiffre_texte = chiffre_texte;
				continue
			} else {
				return Err(ErreurSophie::OrthographeNombre(texte.to_string()))
			}
		}
		if let Some(chiffre) = NOMS_UNITES.iter().position(|&s| s == chiffre_texte) {
			if nombre%10 > 0 {
				return Err(ErreurSophie::OrthographeNombre(texte.to_string()))
			}
			nombre += chiffre;
			dernier_chiffre_texte = chiffre_texte;
			continue
		}
		if let Some(chiffre) = NOMS_DIZAINES.iter().position(|&s| s == chiffre_texte) {
			if nombre%100 > 0 && chiffre != 1 {
				return Err(ErreurSophie::OrthographeNombre(texte.to_string()))
			}
			nombre += chiffre*10;
			dernier_chiffre_texte = chiffre_texte;
			continue
		}
		if let Some(chiffre) = NOMS_UNITES_DIX.iter().position(|&s| s == chiffre_texte) {
			if nombre%10 > 0 {
				return Err(ErreurSophie::OrthographeNombre(texte.to_string()))
			}
			nombre += 10 + chiffre;
			dernier_chiffre_texte = chiffre_texte;
			continue
		}
		if chiffre_texte == "et" {
			continue
		}
		return Err(ErreurSophie::OrthographeNombre(texte.to_string()))
	}

    Ok(nombre)
}
