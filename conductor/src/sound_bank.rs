use crate::{
	sound::{Sound, SoundSettings},
	tag::Tag,
};
use std::{error::Error, path::Path};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct SoundId {
	pub(crate) index: usize,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TagId {
	pub(crate) index: usize,
}

pub struct SoundBank {
	pub(crate) sounds: Vec<Sound>,
	pub(crate) tags: Vec<Tag>,
}

impl SoundBank {
	pub fn new() -> Self {
		Self {
			sounds: vec![],
			tags: vec![],
		}
	}

	pub fn load(
		&mut self,
		path: &Path,
		settings: SoundSettings,
	) -> Result<SoundId, Box<dyn Error>> {
		let id = SoundId {
			index: self.sounds.len(),
		};
		self.sounds.push(Sound::from_ogg_file(path, settings)?);
		Ok(id)
	}

	pub fn add_tag(&mut self, tag: Tag) -> TagId {
		let id = TagId {
			index: self.tags.len(),
		};
		self.tags.push(tag);
		id
	}
}
