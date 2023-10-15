//! Plays audio using [cpal](https://crates.io/crates/cpal).

#![cfg_attr(docsrs, doc(cfg(feature = "cpal")))]

mod error;
pub use error::*;

/// Settings for the [`cpal`] backend.
#[derive(Default)]
pub struct CpalBackendSettings {
	/// The output audio device to use. If [`None`], the default output
	/// device will be used.
	pub device: Option<cpal::Device>,
}

#[cfg(target_arch = "wasm32")]
mod wasm;
#[cfg(target_arch = "wasm32")]
pub use wasm::CpalBackend;

#[cfg(not(target_arch = "wasm32"))]
mod desktop;
#[cfg(not(target_arch = "wasm32"))]
pub use desktop::CpalBackend;
