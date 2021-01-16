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

use super::{Arrangement, ArrangementId};

#[derive(Debug, Error)]
pub enum ArrangementHandleError {
	#[error("The backend cannot receive commands because it no longer exists")]
	BackendDisconnected,
}

#[derive(Clone)]
pub struct ArrangementHandle {
	id: ArrangementId,
	duration: f64,
	default_track: TrackIndex,
	semantic_duration: Option<f64>,
	default_loop_start: Option<f64>,
	command_sender: Sender<Command>,
}

impl ArrangementHandle {
	pub(crate) fn new(arrangement: &Arrangement, command_sender: Sender<Command>) -> Self {
		Self {
			id: arrangement.id(),
			duration: arrangement.duration(),
			default_track: arrangement.default_track(),
			semantic_duration: arrangement.semantic_duration(),
			default_loop_start: arrangement.default_loop_start(),
			command_sender,
		}
	}

	pub fn id(&self) -> ArrangementId {
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

	pub fn play(
		&mut self,
		settings: InstanceSettings,
	) -> Result<InstanceHandle, ArrangementHandleError> {
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
			.map_err(|_| ArrangementHandleError::BackendDisconnected)?;
		Ok(handle)
	}

	pub fn pause(&mut self, settings: PauseInstanceSettings) -> Result<(), ArrangementHandleError> {
		self.command_sender
			.send(InstanceCommand::PauseInstancesOf(self.id.into(), settings).into())
			.map_err(|_| ArrangementHandleError::BackendDisconnected)
	}

	pub fn resume(
		&mut self,
		settings: ResumeInstanceSettings,
	) -> Result<(), ArrangementHandleError> {
		self.command_sender
			.send(InstanceCommand::ResumeInstancesOf(self.id.into(), settings).into())
			.map_err(|_| ArrangementHandleError::BackendDisconnected)
	}

	pub fn stop(&mut self, settings: StopInstanceSettings) -> Result<(), ArrangementHandleError> {
		self.command_sender
			.send(InstanceCommand::StopInstancesOf(self.id.into(), settings).into())
			.map_err(|_| ArrangementHandleError::BackendDisconnected)
	}
}
