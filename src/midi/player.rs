use bevy::prelude::*;

#[derive(Resource)]
pub struct MidiPlayer {
    pub position: f32,
    pub playing: bool,
}

#[derive(Resource)]
pub struct MidiFile {
    pub events: Vec<MidiPlaybackEvent>,
}

pub struct MidiPlaybackEvent {
    pub start: f32,
    pub duration: f32,
    pub key: u8,
    pub velocity: u8,
}
