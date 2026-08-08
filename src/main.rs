use std::f32;

use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use movemymidi::midi::{
    entity::{MidiAction, MidiEntity, MidiEventAction, MidiKey},
    player::{MidiFile, MidiPlaybackEvent, MidiPlayer},
};

const PRE_MOTION_DURATION: f32 = 0.1;

fn main() {
    App::new()
        .insert_resource(MidiPlayer {
            position: 0.0,
            playing: true,
        })
        .insert_resource(MidiFile {
            events: vec![
                MidiPlaybackEvent {
                    start: 5.0,
                    duration: 2.0,
                    key: 60,
                    velocity: 127,
                },
                MidiPlaybackEvent {
                    start: 8.0,
                    duration: 2.0,
                    key: 60,
                    velocity: 127,
                },
                MidiPlaybackEvent {
                    start: 11.0,
                    duration: 2.0,
                    key: 60,
                    velocity: 127,
                },
            ],
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, (setup, add_test_entity).chain())
        .add_systems(Update, (midi_playback, keyboard_input).chain())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
        PanOrbitCamera {
            button_orbit: MouseButton::Middle,
            button_pan: MouseButton::Middle,
            modifier_pan: Some(KeyCode::ShiftLeft),
            ..Default::default()
        },
    ));
}

fn keyboard_input(mut playback: ResMut<MidiPlayer>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Space) {
        playback.playing = !playback.playing
    }
}

fn midi_playback(
    time: Res<Time>,
    midi: Res<MidiFile>,
    mut playback: ResMut<MidiPlayer>,
    mut query: Query<(&MidiEntity, &MidiKey, &MidiAction, &mut Transform)>,
) {
    if !playback.playing {
        return;
    }

    let previous = playback.position;
    playback.position += time.delta_secs();

    let current_position = playback.position;

    for event in &midi.events {
        let note_start = event.start;
        let note_duration = event.duration;
        let note_end = event.start + event.duration;

        for (_, key, motion, mut transform) in &mut query {
            if event.key != key.0 {
                continue;
            }

            for action in &motion.actions {
                if ((note_start - current_position) <= PRE_MOTION_DURATION
                    && note_start > current_position)
                    || (previous < note_start && note_start <= current_position)
                {
                    match action {
                        MidiEventAction::Move(xyz) => transform.translation += xyz / 2.0,
                        MidiEventAction::Rotate(xyz) => {
                            transform.rotation +=
                                Quat::from_euler(EulerRot::XYZ, xyz.x, xyz.y, xyz.z) / 2.0
                        }
                        MidiEventAction::Scale(xyz) => transform.scale += xyz / 2.0,
                    }
                } else if ((note_end - current_position) <= PRE_MOTION_DURATION
                    && note_end > current_position)
                    || (previous < note_end && note_end <= current_position)
                {
                    match action {
                        MidiEventAction::Move(xyz) => transform.translation -= xyz / 2.0,
                        MidiEventAction::Rotate(xyz) => {
                            transform.rotation -=
                                Quat::from_euler(EulerRot::XYZ, xyz.x, xyz.y, xyz.z) / 2.0
                        }
                        MidiEventAction::Scale(xyz) => transform.scale -= xyz / 2.0,
                    }
                }
            }
        }
    }
}

fn add_test_entity(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        MidiEntity,
        MidiKey(60),
        MidiAction {
            actions: vec![
                MidiEventAction::Move(Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.1,
                }),
                MidiEventAction::Scale(Vec3 {
                    x: 1.1,
                    y: 1.1,
                    z: 1.1,
                }),
            ],
        },
        WorldAssetRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("model.glb"))),
        Transform::default(),
    ));
}
