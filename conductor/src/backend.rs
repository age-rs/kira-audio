use crate::{
	event::Command,
	manager::PlaySoundSettings,
	sound::Sound,
	sound_bank::{SoundBank, SoundId, TagId},
	stereo_sample::StereoSample,
	tag::Tag,
};
use ringbuf::Consumer;

pub struct Backend {
	dt: f32,
	sound_bank: SoundBank,
	command_consumer: Consumer<Command>,
}

impl Backend {
	pub fn new(
		sample_rate: u32,
		sound_bank: SoundBank,
		command_consumer: Consumer<Command>,
	) -> Self {
		Self {
			dt: 1.0 / sample_rate as f32,
			sound_bank,
			command_consumer,
		}
	}

	fn get_tag(&self, tag_id: TagId) -> &Tag {
		&self.sound_bank.tags[tag_id.index]
	}

	fn get_tag_mut(&mut self, tag_id: TagId) -> &mut Tag {
		&mut self.sound_bank.tags[tag_id.index]
	}

	fn get_sound(&self, sound_id: SoundId) -> &Sound {
		&self.sound_bank.sounds[sound_id.index]
	}

	fn get_sound_mut(&mut self, sound_id: SoundId) -> &mut Sound {
		&mut self.sound_bank.sounds[sound_id.index]
	}

	fn get_sound_volume(&self, sound_id: SoundId) -> f32 {
		let mut volume = 1.0;
		let sound = self.get_sound(sound_id);
		for tag_id in &sound.tags {
			volume *= self.get_tag(*tag_id).volume;
		}
		volume
	}

	fn play_sound(&mut self, sound_id: SoundId, settings: PlaySoundSettings) {
		self.get_sound_mut(sound_id).play(settings);
	}

	pub fn process(&mut self) -> StereoSample {
		while let Some(command) = self.command_consumer.pop() {
			match command {
				Command::PlaySound(id, settings) => {
					self.play_sound(id, settings);
				}
			}
		}
		let mut out = StereoSample::from_mono(0.0);
		let dt = self.dt;
		for i in 0..self.sound_bank.sounds.len() {
			let id = SoundId { index: i };
			let volume = self.get_sound_volume(id);
			let sound = self.get_sound_mut(id);
			out += sound.process(dt) * volume;
		}
		out
	}
}
