use std::{error::Error, fmt::Display};

use cpal::{BuildStreamError, PlayStreamError, SupportedStreamConfigsError};
use lewton::VorbisError;

#[derive(Debug)]
pub enum AudioError {
	NoDefaultOutputDevice,
	SupportedStreamConfigsError(SupportedStreamConfigsError),
	NoSupportedAudioConfig,
	BuildStreamError(BuildStreamError),
	PlayStreamError(PlayStreamError),
	CommandQueueFull,
	UnsupportedChannelConfiguration,
	UnsupportedAudioFileFormat,
	InvalidSequenceLoopPoint,
	IoError(std::io::Error),
	Mp3Error(minimp3::Error),
	VariableMp3SampleRate,
	UnknownMp3SampleRate,
	OggError(VorbisError),
	FlacError(claxon::Error),
	WavError(hound::Error),
}

impl Display for AudioError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			AudioError::NoDefaultOutputDevice => {
				f.write_str("Cannot find the default audio output device")
			}
			AudioError::SupportedStreamConfigsError(error) => f.write_str(&format!("{}", error)),
			AudioError::NoSupportedAudioConfig => f.write_str("No supported audio configurations"),
			AudioError::BuildStreamError(error) => f.write_str(&format!("{}", error)),
			AudioError::PlayStreamError(error) => f.write_str(&format!("{}", error)),
			AudioError::CommandQueueFull => {
				f.write_str("Cannot send a command to the audio thread because the queue is full")
			}
			AudioError::UnsupportedChannelConfiguration => {
				f.write_str("Only mono and stereo audio is supported")
			}
			AudioError::UnsupportedAudioFileFormat => {
				f.write_str("Only .mp3, .ogg, .flac, and .wav files are supported")
			}
			AudioError::InvalidSequenceLoopPoint => {
				f.write_str("The loop point of a sequence cannot be at the very end")
			}
			AudioError::IoError(error) => f.write_str(&format!("{}", error)),
			AudioError::Mp3Error(error) => f.write_str(&format!("{}", error)),
			AudioError::VariableMp3SampleRate => {
				f.write_str("mp3s with variable sample rates are not supported")
			}
			AudioError::UnknownMp3SampleRate => {
				f.write_str("Could not get the sample rate of the mp3")
			}
			AudioError::OggError(error) => f.write_str(&format!("{}", error)),
			AudioError::FlacError(error) => f.write_str(&format!("{}", error)),
			AudioError::WavError(error) => f.write_str(&format!("{}", error)),
		}
	}
}

impl Error for AudioError {}

impl From<std::io::Error> for AudioError {
	fn from(error: std::io::Error) -> Self {
		Self::IoError(error)
	}
}

impl From<minimp3::Error> for AudioError {
	fn from(error: minimp3::Error) -> Self {
		Self::Mp3Error(error)
	}
}

impl From<VorbisError> for AudioError {
	fn from(error: VorbisError) -> Self {
		Self::OggError(error)
	}
}

impl From<claxon::Error> for AudioError {
	fn from(error: claxon::Error) -> Self {
		Self::FlacError(error)
	}
}

impl From<hound::Error> for AudioError {
	fn from(error: hound::Error) -> Self {
		Self::WavError(error)
	}
}

impl From<SupportedStreamConfigsError> for AudioError {
	fn from(error: SupportedStreamConfigsError) -> Self {
		Self::SupportedStreamConfigsError(error)
	}
}

impl From<BuildStreamError> for AudioError {
	fn from(error: BuildStreamError) -> Self {
		Self::BuildStreamError(error)
	}
}

impl From<PlayStreamError> for AudioError {
	fn from(error: PlayStreamError) -> Self {
		Self::PlayStreamError(error)
	}
}

pub type AudioResult<T> = Result<T, AudioError>;
