mod instances;
mod sequences;

use super::{AudioManagerSettings, Event};
use crate::{
	command::{Command, SoundCommand},
	metronome::Metronome,
	sequence::Sequence,
	sound::{Sound, SoundId},
	stereo_sample::StereoSample,
};
use indexmap::IndexMap;
use instances::Instances;
use ringbuf::{Consumer, Producer, RingBuffer};
use sequences::Sequences;

pub struct Backend<CustomEvent: Send + 'static> {
	dt: f64,
	sounds: IndexMap<SoundId, Sound>,
	command_queue: Vec<Command<CustomEvent>>,
	command_consumer: Consumer<Command<CustomEvent>>,
	event_producer: Producer<Event<CustomEvent>>,
	sounds_to_unload_producer: Producer<Sound>,
	sequences_to_unload_producer: Producer<Sequence<CustomEvent>>,
	metronome: Metronome,
	instances: Instances,
	sequences: Sequences<CustomEvent>,
}

impl<CustomEvent: Copy + Send + 'static> Backend<CustomEvent> {
	pub fn new(
		sample_rate: u32,
		settings: AudioManagerSettings,
		command_consumer: Consumer<Command<CustomEvent>>,
		event_producer: Producer<Event<CustomEvent>>,
		sounds_to_unload_producer: Producer<Sound>,
		sequences_to_unload_producer: Producer<Sequence<CustomEvent>>,
	) -> Self {
		Self {
			dt: 1.0 / sample_rate as f64,
			sounds: IndexMap::with_capacity(settings.num_sounds),
			command_queue: Vec::with_capacity(settings.num_commands),
			command_consumer,
			event_producer,
			sounds_to_unload_producer,
			sequences_to_unload_producer,
			metronome: Metronome::new(settings.metronome_settings),
			instances: Instances::new(settings.num_instances),
			sequences: Sequences::new(settings.num_sequences, settings.num_commands),
		}
	}

	pub fn standalone(
		sample_rate: u32,
		settings: AudioManagerSettings,
	) -> (
		Backend<CustomEvent>,
		Producer<Command<CustomEvent>>,
		Consumer<Event<CustomEvent>>,
		Consumer<Sound>,
		Consumer<Sequence<CustomEvent>>,
	) {
		let (command_producer, command_consumer) =
			RingBuffer::<Command<CustomEvent>>::new(settings.num_commands).split();
		let (event_producer, event_consumer) =
			RingBuffer::<Event<CustomEvent>>::new(settings.num_events).split();
		let (sounds_to_unload_producer, sounds_to_unload_consumer) =
			RingBuffer::<Sound>::new(settings.num_sounds).split();
		let (sequences_to_unload_producer, sequences_to_unload_consumer) =
			RingBuffer::<Sequence<CustomEvent>>::new(settings.num_sequences).split();
		let backend = Self::new(
			sample_rate,
			settings,
			command_consumer,
			event_producer,
			sounds_to_unload_producer,
			sequences_to_unload_producer,
		);
		(
			backend,
			command_producer,
			event_consumer,
			sounds_to_unload_consumer,
			sequences_to_unload_consumer,
		)
	}

	pub fn process_commands(&mut self) {
		while let Some(command) = self.command_consumer.pop() {
			self.command_queue.push(command);
		}
		for command in self.command_queue.drain(..) {
			match command {
				Command::Sound(command) => match command {
					SoundCommand::LoadSound(id, sound) => {
						self.sounds.insert(id, sound);
					}
					SoundCommand::UnloadSound(id) => {
						self.instances.stop_instances_of_sound(id, None);
						if let Some(sound) = self.sounds.remove(&id) {
							match self.sounds_to_unload_producer.push(sound) {
								Ok(_) => {}
								Err(sound) => {
									self.sounds.insert(id, sound);
								}
							}
						}
					}
				},
				Command::Metronome(command) => {
					self.metronome.run_command(command);
				}
				Command::Instance(command) => {
					self.instances.run_command(command);
				}
				Command::Sequence(command) => {
					self.sequences.run_command(command);
				}
				Command::EmitCustomEvent(event) => {
					match self.event_producer.push(Event::Custom(event)) {
						Ok(_) => {}
						Err(_) => {}
					}
				}
			}
		}
	}

	pub fn update_metronome(&mut self) {
		for interval in self.metronome.update(self.dt) {
			match self
				.event_producer
				.push(Event::MetronomeIntervalPassed(interval))
			{
				Ok(_) => {}
				Err(_) => {}
			}
		}
	}

	pub fn update_sequences(&mut self) {
		for command in self.sequences.update(
			self.dt,
			&self.metronome,
			&mut self.sequences_to_unload_producer,
		) {
			self.command_queue.push(command.into());
		}
	}

	pub fn process_instances(&mut self) -> StereoSample {
		self.instances.process(self.dt, &self.sounds)
	}

	pub fn process(&mut self) -> StereoSample {
		self.process_commands();
		self.update_metronome();
		self.update_sequences();
		self.process_instances()
	}
}
