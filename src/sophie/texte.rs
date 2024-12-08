use super::Sophie;
use super::ErreurSophie;
use super::Variable;
use super::nombres;

impl Sophie {
	pub fn texte(&self, arguments: &str) -> Result<String, ErreurSophie> {
		let liste_arguments: Vec<&str> = arguments.split(',').collect();

		let mut texte = "".to_string();

		for argument in liste_arguments {
			let argument: &str = argument.trim();
			if argument.starts_with('"') {
				if argument.ends_with('"') {
					texte += &argument[1..argument.len()-1];
				}
			} else {
				let resultat: String = if argument.contains(" ") {
					nombres::nombre_comme_texte(self.operation(argument)?)
				} else {
					if !self.variables.contains_key(argument) {
						return Err(ErreurSophie::VariableInconnue(argument.into()))
					}
					match self.variables[argument] {
						Variable::Entier(nombre) => nombres::nombre_comme_texte(nombre),
						Variable::Texte(ref contenu) => contenu.to_string(),
					}
				};
				texte += &resultat;
			}
		}
		Ok(texte)
	}
}