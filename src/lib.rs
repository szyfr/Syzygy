

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
pub mod graphics;


//= Enumerations



//= Structure

pub struct Syzygy {

	//* Core */
	pub running: bool,

	//* Graphics */
	pub graphicsMode: graphics::GraphicsMode,


}

impl Syzygy {

	//* The creation of the game */
	pub fn create() -> Self {
		return Syzygy{
			running: true,

			graphicsMode: graphics::GraphicsMode::None,
		};
	}

	//* Draw Loop */
	pub fn draw(&self) -> &Self {
		return self;
	}

	//* Check if running */
	pub fn is_running(&self) -> bool {
		return self.running;
	}

}