

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
pub mod graphics;
pub mod raylib;


//= Enumerations



//= Structure

pub struct Syzygy {

	/// Core
	pub running: bool,

	//* Graphics */
	pub graphicsEngine: graphics::GraphicsEngine,


}

impl Syzygy {

	/// The creation of the game
	pub fn create() -> Self {
		return Syzygy{
			running: true,

			graphicsEngine: graphics::GraphicsEngine::create(),
		};
	}

	/// Draw Loop
	pub fn draw(&self) -> &Self {
		print!("{}\n",self.graphicsEngine.mode);
		return self;
	}

	/// Check if running
	pub fn is_running(&self) -> bool {
		return self.running;
	}

	/// Change the graphics mode
	pub fn set_graphics_mode(mut self, mode: graphics::GraphicsMode) -> Self {
		self.graphicsEngine.mode = mode;

		return self;
	}

}