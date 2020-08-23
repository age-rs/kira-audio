use crate::{
	instance::{InstanceId, InstanceSettings},
	sequence::{Sequence, SequenceId},
	sound::{Sound, SoundId},
	tempo::Tempo,
	tween::Tween,
};

pub enum SoundCommand {
	LoadSound(SoundId, Sound),
	UnloadSound(SoundId),
}

#[derive(Debug, Copy, Clone)]
pub enum InstanceCommand<Id> {
	PlaySound(SoundId, Id, InstanceSettings),
	SetInstanceVolume(Id, f64, Option<Tween>),
	SetInstancePitch(Id, f64, Option<Tween>),
	PauseInstance(Id, Option<Tween>),
	ResumeInstance(Id, Option<Tween>),
	StopInstance(Id, Option<Tween>),
	PauseInstancesOfSound(SoundId, Option<Tween>),
	ResumeInstancesOfSound(SoundId, Option<Tween>),
	StopInstancesOfSound(SoundId, Option<Tween>),
}

#[derive(Debug, Copy, Clone)]
pub enum MetronomeCommand {
	SetMetronomeTempo(Tempo),
	StartMetronome,
	PauseMetronome,
	StopMetronome,
}

pub enum SequenceCommand<CustomEvent> {
	StartSequence(SequenceId, Sequence<CustomEvent>),
	MuteSequence(SequenceId),
	UnmuteSequence(SequenceId),
}

pub enum Command<CustomEvent> {
	Sound(SoundCommand),
	Instance(InstanceCommand<InstanceId>),
	Metronome(MetronomeCommand),
	Sequence(SequenceCommand<CustomEvent>),
	EmitCustomEvent(CustomEvent),
}
