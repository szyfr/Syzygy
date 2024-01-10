

//= Imports
use std::{fmt::Display, collections::HashMap};
use crate::raylib;


//= Enumerations

pub enum GraphicsMode {
	None,
	Mode2D,
	Mode3D,
}
impl Display for GraphicsMode {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match *self {
			GraphicsMode::None   => write!(f, "None"),
			GraphicsMode::Mode2D => write!(f, "Mode2D"),
			GraphicsMode::Mode3D => write!(f, "Mode3D"),
		}
	}
}


//= Structure

pub struct GraphicsEngine {
	pub mode: GraphicsMode,

	pub fonts:		HashMap<String, raylib::fonts::Font>,
	pub textures:	HashMap<String, raylib::textures::Texture>,
	pub models:		HashMap<String, raylib::models::Model>,
	pub shaders:	HashMap<String, raylib::shaders::Shader>,
}

impl GraphicsEngine {
	
	/// 
	pub fn create() -> Self {
		//* Fonts */
		let mut fonts: HashMap<String, raylib::fonts::Font> = HashMap::new();
		fonts.insert("default".to_string(), raylib::fonts::Font::load_default());

		return GraphicsEngine{
			mode: GraphicsMode::None,

			fonts:		HashMap::new(),
			textures:	HashMap::new(),
			models:		HashMap::new(),
			shaders:	HashMap::new(),
		}
	}

}