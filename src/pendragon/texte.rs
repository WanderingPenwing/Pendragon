use super::Pendragon;
use super::ErreurPendragon;
use super::Element;
use super::nombre;
use super::booleen;

impl Pendragon {
	pub fn texte(&self, arguments: &str) -> Result<String, ErreurPendragon> {
		let liste_arguments: Vec<&str> = arguments.split(',').collect();

		let mut texte = "".to_string();

		for argument in liste_arguments {
			let argument: &str = argument.trim();
			if argument.starts_with('"') {
				if argument.ends_with('"') {
					texte += &argument[1..argument.len()-1];
				} else {
					return Err(ErreurPendragon::TexteInvalide("guillemet mal refermé".into()))
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
			
			let Element::Texte(contenu) = variable else {
				return Err(ErreurPendragon::MauvaisType(argument.into(), variable.type_element().nom(), "texte".into()))
			};
			
			texte += &contenu;
		}
		Ok(texte)
	}
}