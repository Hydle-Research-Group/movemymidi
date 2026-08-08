use bevy::prelude::*;

#[derive(Component)]
pub struct MidiEntity;

#[derive(Component)]
pub struct MidiKey(pub u8);

pub enum MidiEventAction {
    Move(Vec3),
    Rotate(Vec3),
    Scale(Vec3),
}

#[derive(Component)]
pub struct MidiAction {
    pub actions: Vec<MidiEventAction>,
}
