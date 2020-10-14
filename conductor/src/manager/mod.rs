mod backend;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use ringbuf::{Consumer, Producer, RingBuffer};

use crate::sound::{Sound, SoundId};

use self::backend::Backend;

#[derive(Debug, Clone)]
pub struct AudioManagerSettings {
	pub max_sounds: usize,
}

impl Default for AudioManagerSettings {
	fn default() -> Self {
		Self { max_sounds: 100 }
	}
}

pub struct AudioManager {
	exit_message_producer: Producer<bool>,
	add_sound_producer: Producer<Sound>,
	add_sound_result_consumer: Consumer<Result<SoundId, Sound>>,
}

impl AudioManager {
	pub fn new(settings: AudioManagerSettings) -> Self {
		let (exit_message_producer, mut exit_message_consumer) = RingBuffer::new(1).split();
		let (add_sound_producer, mut add_sound_consumer) = RingBuffer::new(1).split();
		let (add_sound_result_producer, mut add_sound_result_consumer) = RingBuffer::new(1).split();
		std::thread::spawn(move || {
			let host = cpal::default_host();
			let device = host.default_output_device().unwrap();
			let mut supported_configs_range = device.supported_output_configs().unwrap();
			let config = supported_configs_range
				.next()
				.unwrap()
				.with_max_sample_rate()
				.config();
			let sample_rate = config.sample_rate.0;
			let channels = config.channels;
			if channels != 2 {
				panic!("Only stereo audio is supported");
			}
			let mut backend = Backend::new(
				sample_rate,
				&settings,
				add_sound_consumer,
				add_sound_result_producer,
			);
			let stream = device
				.build_output_stream(
					&config,
					move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
						for frame in data.chunks_mut(channels as usize) {
							let out = backend.process();
							frame[0] = out.left;
							frame[1] = out.right;
						}
					},
					|_| {},
				)
				.unwrap();
			stream.play().unwrap();
			loop {
				if exit_message_consumer.pop().is_some() {
					break;
				}
			}
		});
		Self {
			exit_message_producer,
			add_sound_producer,
			add_sound_result_consumer,
		}
	}
}

impl AudioManager {
	pub fn add_sound(&mut self, sound: Sound) -> Result<SoundId, Sound> {
		self.add_sound_producer.push(sound).unwrap();
		loop {
			if let Some(result) = self.add_sound_result_consumer.pop() {
				return result;
			}
		}
	}
}

impl Drop for AudioManager {
	fn drop(&mut self) {
		self.exit_message_producer.push(true).unwrap();
	}
}
