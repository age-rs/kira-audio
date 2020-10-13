use std::f32::consts::PI;

use cpal::{
	traits::{DeviceTrait, HostTrait, StreamTrait},
	Sample,
};
use ringbuf::{Producer, RingBuffer};

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
			let sample_rate = config.sample_rate;
			let channels = config.channels;
			let mut phase = 0.0;
			let stream = device
				.build_output_stream(
					&config,
					move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
						for frame in data.chunks_mut(channels as usize) {
							phase += 440.0 / (sample_rate.0 as f32);
							let out = 0.5 * (phase * 2.0 * PI).sin();
							for sample in frame {
								*sample = Sample::from(&out);
							}
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
