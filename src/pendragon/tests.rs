

//#[test]
//fn teste_definition_variable() {
//	let mut sophie = Pendragon::new();
//	let resultat = sophie.execute_phrase("Définis Element comme entier");
//	match resultat {
//		Ok(_) => {
//			assert_eq!(sophie.variables["Element"], Element::Entier(0), "Element mal définie");
//		}
//		Err(raison) => {
//			panic!("Définition de variable échouée : {}", raison);
//		}
//	}
//}
//
//#[test]
//fn teste_modification_variable() {
//	let mut sophie = Pendragon::new();
//	if let Err(raison) = sophie.execute_phrase("Définis Element comme entier") {
//		panic!("Définition de variable échouée : {}", raison);
//	}
//	let a = 2345678;
//	let resultat = sophie.execute_phrase(&format!("Modifie Element avec {} ", nombre::nombre_comme_texte(a)));
//	match resultat {
//		Ok(_) => {
//			assert_eq!(sophie.variables["Element"], Element::Entier(a), "Element mal modifiée");
//		}
//		Err(raison) => {
//			panic!("Modification de variable échouée : {}", raison);
//		}
//	}
//}
//
//#[test]
//fn teste_operation_variable() {
//	let mut sophie = Pendragon::new();
//	if let Err(raison) = sophie.execute_phrase("Définis Element comme entier") {
//		panic!("Définition de variable échouée : {}", raison);
//	}
//	let a = 2345678;
//	if let Err(raison) = sophie.execute_phrase(&format!("Modifie Element avec {} ", nombre::nombre_comme_texte(a))) {
//		panic!("Modification de variable échouée : {}", raison);
//	}
//	let b = 987654;
//	let resultat = sophie.operation(&format!("Element plus {}", nombre::nombre_comme_texte(b)));
//	match resultat {
//		Ok(nombre) => {
//				assert_eq!(nombre, a+b, "Echec de la somme d'un entier et d'une variable, attendais {}, a reçu {}", a+b, nombre);
//		}
//		Err(raison) => {
//			panic!("Opération de variable échouée : {}", raison);
//		}
//	}
//}

//#[test]
//fn teste_texte() {
//	let mut sophie = Pendragon::new();
//	if let Err(raison) = sophie.execute_phrase("Définis A comme entier") {
//		panic!("Définition de variable échouée : {}", raison);
//	}
//	let a = 2345678;
//	if let Err(raison) = sophie.execute_phrase(&format!("Modifie A avec {} ", nombre::nombre_comme_texte(a))) {
//		panic!("Modification de variable échouée : {}", raison);
//	}
//	if let Err(raison) = sophie.execute_phrase("Définis B comme texte") {
//		panic!("Définition de variable échouée : {}", raison);
//	}
//	if let Err(raison) = sophie.execute_phrase("Modifie B avec \"hello there\", \" general\", \" Kenobi\"") {
//		panic!("Modification de variable échouée : {}", raison);
//	}
//	let resultat = sophie.texte("\"Combo : \", B, \" / \", A plus ouvre la parenthèse un plus cinq ferme la parenthèse fois ouvre la parenthèse huit moins un ferme la parenthèse");
//	
//	match resultat {
//		Ok(texte) => assert_eq!(texte, "Combo : hello there general Kenobi / deux-millions-trois-cent-quarante-cinq-mille-sept-cent-vingt", "Texte mal calculé"),
//		Err(raison) => panic!("Calcul de texte échoué : {}", raison),
//	}
//}

// --------------------------------------------- anti-test

//#[test]
//fn teste_redefinition_variable() {
//	let mut sophie = Pendragon::new();
//	if let Err(raison) = sophie.execute_phrase("Définis Element comme entier") {
//		panic!("Définition de variable échouée : {}", raison);
//	};
//	let Err(raison) = sophie.execute_phrase("Définis Element comme texte") else {
//		panic!("Ne devrais pas pouvoir redéfinir une variable");
//	};
//	if let ErreurPendragon::MauvaisArgument(ref texte) = raison {
//		assert_eq!(texte, "la variable \"Element\" existe déjà", "Définition échouée avec erreur imprévue : {}", raison);
//	} else {
//		panic!("Définition échouée avec erreur imprévue : {}", raison);
//	}
//}
//
//#[test]
//fn teste_echec_modification() {
//	let mut sophie = Pendragon::new();
//	let resultat = sophie.execute_phrase("Modifie Element avec deux");
//	let Err(raison) = resultat else {
//		panic!("Ne devrais pas pouvoir modifier une variable non définie");
//	};
//	if let ErreurPendragon::VariableInconnue(nom) = raison {
//		assert_eq!(nom, "Element", "Mauvais nom de variable reconnu : {}", nom);
//	} else {
//		panic!("Modification échouée avec erreur imprévue : {}", raison);
//	}			
//}
//
//#[test]
//fn teste_majuscule_variable() {
//	let mut sophie = Pendragon::new();
//	let resultat = sophie.execute_phrase("Définis variable comme entier");
//	let Err(raison) = resultat else {
//		panic!("Ne devrais pas pouvoir definir une variable sans majuscule");
//	};
//	if let ErreurPendragon::MauvaisArgument(explication) = raison {
//		assert_eq!(explication, "il manque une majuscule à la variable", "Mauvaise explication : {}", explication);
//	} else {
//		panic!("Définition échouée avec erreur imprévue : {}", raison);
//	}
//}
//
//#[test]
//fn teste_point_phrase() {
//	let mut sophie = Pendragon::new();
//	let resultat = sophie.execute("Définis Element comme entier".into());
//	let Err(raison) = resultat else {
//		panic!("Ne devrais pas pouvoir faire de commande sans point à la fin");
//	};
//	let ErreurPendragon::ManquePoint = raison else {
//		panic!("Définition échouée avec erreur imprévue : {}", raison);
//	};
//}
