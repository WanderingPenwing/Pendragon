

pub enum MotCle {
	Definis,
	Modifie,
	Affiche,
	Demande,
	Si,
	Sinon,
	NotaBene,
	Plus,
	Moins,
	Fois,
	Divise,
	Et,
	Ou,
	Non,
	OuvreParenthese,
	FermeParenthese,
	Puis,
	Alinea,
	RetourLigne,
}

pub impl MotCle {
	fn comme_texte(&self) -> String {
		match Self {
			Self::Definis,
			Self::Modifie,
			Self::Affiche,
			Self::Demande,
			Self::Si,
			Self::Sinon,
			Self::NotaBene,
			Plus,
			Moins,
			Fois,
			Divise,
			Et,
			Ou,
			Non,
			OuvreParenthese,
			FermeParenthese,
			Puis,
			Alinea,
			RetourLigne,
		}
	}
	
	fn depuis_texte(texte: &str) -> Self {
		match texte {
		
		}
	}