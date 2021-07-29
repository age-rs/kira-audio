use std::sync::Arc;

use atomic_arena::Controller;

use crate::{
	error::PlaySoundError,
	manager::{
		backend::context::Context,
		command::{producer::CommandProducer, Command, InstanceCommand},
	},
};

use super::{
	data::SoundData,
	instance::{handle::InstanceHandle, settings::InstanceSettings, Instance, InstanceId},
	SoundId, SoundShared,
};

pub struct SoundHandle {
	pub(crate) context: Arc<Context>,
	pub(crate) id: SoundId,
	pub(crate) data: Arc<dyn SoundData>,
	pub(crate) shared: Arc<SoundShared>,
	pub(crate) instance_controller: Controller,
	pub(crate) command_producer: CommandProducer,
}

impl SoundHandle {
	pub fn id(&self) -> SoundId {
		self.id
	}

	pub fn data(&self) -> &Arc<dyn SoundData> {
		&self.data
	}

	pub fn play(&mut self, settings: InstanceSettings) -> Result<InstanceHandle, PlaySoundError> {
		let id = InstanceId(
			self.instance_controller
				.try_reserve()
				.map_err(|_| PlaySoundError::InstanceLimitReached)?,
		);
		let instance = Instance::new(&self.context, self.id, &self.data, settings);
		let handle = InstanceHandle {
			id,
			context: self.context.clone(),
			command_producer: self.command_producer.clone(),
		};
		self.command_producer
			.push(Command::Instance(InstanceCommand::Add(id, instance)))?;
		Ok(handle)
	}
}

impl Drop for SoundHandle {
	fn drop(&mut self) {
		self.shared.mark_for_removal();
	}
}
