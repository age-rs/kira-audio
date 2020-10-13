use std::io::stdin;

use conductor::{manager::AudioManager, sound::Sound};

fn main() {
	let sound = Sound::from_ogg_file(
		std::env::current_dir()
			.unwrap()
			.join("assets/test_song.ogg"),
	)
	.unwrap();
}
