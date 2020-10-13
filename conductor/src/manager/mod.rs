mod backend;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use ringbuf::{Producer, RingBuffer};

use self::backend::Backend;

pub struct AudioManager {
	exit_message_producer: Producer<bool>,
}

impl AudioManager {
	pub fn new() -> Self {
		let (exit_message_producer, mut exit_message_consumer) = RingBuffer::new(1).split();
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
			let mut backend = Backend::new(sample_rate);
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
		}
	}
}

impl Drop for AudioManager {
	fn drop(&mut self) {
		self.exit_message_producer.push(true).unwrap();
	}
}
