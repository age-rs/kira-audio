use std::io::stdin;

use conductor::manager::AudioManager;

fn main() {
	let mut input = String::new();
	println!("starting audio manager");
	{
		let _manager = AudioManager::new();
		stdin().read_line(&mut input).unwrap();
	}
	println!("stopping audio manager");
	stdin().read_line(&mut input).unwrap();
}
