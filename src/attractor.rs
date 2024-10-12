use glam::{Vec2, Vec3};

pub struct Attractor3d {
    pub pos: Vec3,
    pub reached: bool,
}

impl Attractor3d {
    pub fn new(pos: Vec3) -> Self {
        Self {
            pos,
            reached: false,
        }
    }
}

pub struct Attractor2d {
    pub pos: Vec2,
    pub reached: bool,
}

impl Attractor2d {
    pub fn new(pos: Vec2) -> Self {
        Self {
            pos,
            reached: false,
        }
    }
}
