use bevy::prelude::*;

#[derive(Resource)]
pub struct MidiPlayer {
    pub current_position: f32,
    pub last_position: f32,
    pub playing: bool,
}

impl MidiPlayer {
    pub fn reset(&mut self) {
        self.current_position = 0.0;
        self.last_position = 0.0;
    }
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
