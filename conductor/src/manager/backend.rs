use std::f32::consts::PI;

use crate::frame::Frame;

pub struct Backend {
	sample_rate: u32,
	phase: f32,
}

impl Backend {
	pub fn new(sample_rate: u32) -> Self {
		Self {
			sample_rate,
			phase: 0.0,
		}
	}

	pub fn process(&mut self) -> Frame {
		self.phase += 440.0 / (self.sample_rate as f32);
		Frame::from_mono(0.5 * (self.phase * 2.0 * PI).sin())
	}
}
