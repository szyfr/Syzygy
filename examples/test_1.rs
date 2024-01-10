

//= Imports
use syzygy::Syzygy;


//= Main

fn main() {
	
	let game = Syzygy::create()
		.set_graphics_mode(syzygy::graphics::GraphicsMode::Mode2D);

	while game.is_running() {
		game.draw();
	}
}