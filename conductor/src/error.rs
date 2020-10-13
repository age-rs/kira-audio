use std::{error::Error, fmt::Display};

use lewton::VorbisError;

#[derive(Debug)]
pub enum ConductorError {
	IoError(std::io::Error),
	VorbisError(VorbisError),
	UnsupportedChannelConfiguration,
}

impl Display for ConductorError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ConductorError::IoError(error) => f.write_str(&format!("{}", error)),
			ConductorError::VorbisError(error) => f.write_str(&format!("{}", error)),
			ConductorError::UnsupportedChannelConfiguration => {
				f.write_str("Unsupported channel configuration")
			}
		}
	}
}

impl Error for ConductorError {}

impl From<std::io::Error> for ConductorError {
	fn from(error: std::io::Error) -> Self {
		Self::IoError(error)
	}
}

impl From<VorbisError> for ConductorError {
	fn from(error: VorbisError) -> Self {
		Self::VorbisError(error)
	}
}

pub type ConductorResult<T> = Result<T, ConductorError>;
