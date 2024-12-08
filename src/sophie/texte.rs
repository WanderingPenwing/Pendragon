use super::Sophie;
use super::ErreurSophie;
use super::Variable;
use super::nombre;
use super::booleen;

impl Sophie {
	pub fn texte(&self, arguments: &str) -> Result<String, ErreurSophie> {
		let liste_arguments: Vec<&str> = arguments.split(',').collect();

		let mut texte = "".to_string();

		for argument in liste_arguments {
			let argument: &str = argument.trim();
			if argument.starts_with('"') {
				if argument.ends_with('"') {
					texte += &argument[1..argument.len()-1];
				} else {
					return Err(ErreurSophie::TexteInvalide("guillemet mal refermé".into()))
				}
				continue;
			}			
			
			if let Ok(nombre) = self.operation(argument) {
				texte += &nombre::nombre_comme_texte(nombre);
				continue;
			}
			
			if let Ok(booleen) = self.condition(argument) {
				texte += &booleen::booleen_comme_texte(booleen);
				continue;
			}
			
			let variable = self.recupere_variable(argument)?;
			
			let Variable::Texte(contenu) = variable else {
				return Err(ErreurSophie::MauvaisType(argument.into(), variable.nom_type(), "texte".into()))
			};
			
			texte += &contenu;
		}
		Ok(texte)
	}
}