use conductor::{manager::AudioManager, manager::AudioManagerSettings, sound::Sound};

fn main() {
	let mut manager = AudioManager::new(AudioManagerSettings::default());
	for _ in 0..2 {
		println!(
			"{:?}",
			manager.add_sound(
				Sound::from_ogg_file(
					std::env::current_dir()
						.unwrap()
						.join("assets/test_song.ogg"),
				)
				.unwrap()
			)
		);
	}
}
