pub mod svf;

use std::sync::atomic::{AtomicUsize, Ordering};

use crate::stereo_sample::StereoSample;

static NEXT_EFFECT_INDEX: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct EffectId {
	index: usize,
}

impl EffectId {
	pub(crate) fn new() -> Self {
		let index = NEXT_EFFECT_INDEX.fetch_add(1, Ordering::Relaxed);
		Self { index }
	}
}

pub trait Effect {
	fn process(&mut self, dt: f64, input: StereoSample) -> StereoSample;
}