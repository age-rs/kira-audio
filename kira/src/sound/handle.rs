use flume::Sender;
use thiserror::Error;

use crate::{
	command::{Command, InstanceCommand},
	instance::{
		handle::InstanceHandle, Instance, InstanceSettings, PauseInstanceSettings,
		ResumeInstanceSettings, StopInstanceSettings,
	},
	mixer::TrackIndex,
};

use super::{Sound, SoundId};

#[derive(Debug, Error)]
pub enum SoundHandleError {
	#[error("The backend cannot receive commands because it no longer exists")]
	BackendDisconnected,
}

#[derive(Clone)]
pub struct SoundHandle {
	id: SoundId,
	duration: f64,
	default_track: TrackIndex,
	semantic_duration: Option<f64>,
	default_loop_start: Option<f64>,
	command_sender: Sender<Command>,
}

impl SoundHandle {
	pub(crate) fn new(sound: &Sound, command_sender: Sender<Command>) -> Self {
		Self {
			id: sound.id(),
			duration: sound.duration(),
			default_track: sound.default_track(),
			semantic_duration: sound.semantic_duration(),
			default_loop_start: sound.default_loop_start(),
			command_sender,
		}
	}

	pub fn id(&self) -> SoundId {
		self.id
	}

	pub fn duration(&self) -> f64 {
		self.duration
	}

	pub fn default_track(&self) -> TrackIndex {
		self.default_track
	}

	pub fn semantic_duration(&self) -> Option<f64> {
		self.semantic_duration
	}

	pub fn default_loop_start(&self) -> Option<f64> {
		self.default_loop_start
	}

	pub fn play(&mut self, settings: InstanceSettings) -> Result<InstanceHandle, SoundHandleError> {
		let id = settings.id;
		let instance = Instance::new(
			self.id.into(),
			self.duration,
			None,
			settings.into_internal(self.duration, self.default_loop_start, self.default_track),
		);
		let handle = InstanceHandle::new(id, instance.public_state(), self.command_sender.clone());
		self.command_sender
			.send(InstanceCommand::Play(id, instance).into())
			.map_err(|_| SoundHandleError::BackendDisconnected)?;
		Ok(handle)
	}

	pub fn pause(&mut self, settings: PauseInstanceSettings) -> Result<(), SoundHandleError> {
		self.command_sender
			.send(InstanceCommand::PauseInstancesOf(self.id.into(), settings).into())
			.map_err(|_| SoundHandleError::BackendDisconnected)
	}

	pub fn resume(&mut self, settings: ResumeInstanceSettings) -> Result<(), SoundHandleError> {
		self.command_sender
			.send(InstanceCommand::ResumeInstancesOf(self.id.into(), settings).into())
			.map_err(|_| SoundHandleError::BackendDisconnected)
	}

	pub fn stop(&mut self, settings: StopInstanceSettings) -> Result<(), SoundHandleError> {
		self.command_sender
			.send(InstanceCommand::StopInstancesOf(self.id.into(), settings).into())
			.map_err(|_| SoundHandleError::BackendDisconnected)
	}
}
