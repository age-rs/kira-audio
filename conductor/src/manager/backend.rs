use generational_arena::Arena;
use ringbuf::{Consumer, Producer};

use crate::{frame::Frame, sound::Sound, sound::SoundId};

use super::AudioManagerSettings;

pub struct Backend {
	sample_rate: u32,
	sounds: Arena<Sound>,
	add_sound_consumer: Consumer<Sound>,
	add_sound_result_producer: Producer<Result<SoundId, Sound>>,
}

impl Backend {
	pub fn new(
		sample_rate: u32,
		settings: &AudioManagerSettings,
		add_sound_consumer: Consumer<Sound>,
		add_sound_result_producer: Producer<Result<SoundId, Sound>>,
	) -> Self {
		Self {
			sample_rate,
			sounds: Arena::with_capacity(settings.max_sounds),
			add_sound_consumer,
			add_sound_result_producer,
		}
	}

	fn add_sounds(&mut self) {
		if let Some(sound) = self.add_sound_consumer.pop() {
			match self.sounds.try_insert(sound) {
				Ok(index) => self
					.add_sound_result_producer
					.push(Ok(SoundId(index)))
					.unwrap(),
				Err(sound) => self.add_sound_result_producer.push(Err(sound)).unwrap(),
			}
		}
	}

	pub fn process(&mut self) -> Frame {
		self.add_sounds();
		Frame::from_mono(0.0)
	}
}
