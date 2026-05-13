use bevy::prelude::*;

#[derive(Event)]
pub struct ParticleSpawnEvent {
    pub particle_type: ParticleType,
    pub position: Vec2,
    pub velocity: Option<Vec2>,
    pub count: usize,
}

#[derive(Clone)]
pub enum ParticleType {
    Explosion,
    Impact,
}
