use conductor::{
	command::{Command, InstanceCommand, SoundCommand},
	instance::{InstanceId, InstanceSettings},
	manager::{AudioManagerSettings, Backend},
	sound::{Sound, SoundId, SoundMetadata},
};
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
	let (
		mut backend,
		mut command_producer,
		event_consumer,
		sounds_to_unload_consumer,
		sequences_to_unload_consumer,
	) = Backend::<()>::standalone(
		48000,
		AudioManagerSettings {
			num_instances: 10000,
			num_commands: 10001,
			..Default::default()
		},
	);
	let sound = Sound::from_ogg_file(
		std::env::current_dir()
			.unwrap()
			.parent()
			.unwrap()
			.join("assets/cymbal.ogg"),
	)
	.unwrap();
	let sound_id = SoundId::new(sound.duration(), SoundMetadata::default());
	match command_producer.push(Command::Sound(SoundCommand::LoadSound(sound_id, sound))) {
		Ok(_) => {}
		Err(_) => panic!(),
	}
	for _ in 0..10000 {
		match command_producer.push(Command::Instance(InstanceCommand::PlaySound(
			sound_id,
			InstanceId::new(),
			InstanceSettings::default(),
		))) {
			Ok(_) => {}
			Err(_) => panic!(),
		}
	}
	backend.process_commands();
	c.bench_function("process_commands", |b| {
		b.iter(|| {
			backend.process_instances();
		});
	});
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
