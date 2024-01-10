use syzygy::Syzygy;



fn main() {
	
	let game = Syzygy::create();

	while game.is_running() {
		game.draw();
	}
}