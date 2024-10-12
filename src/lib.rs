pub mod algorithm_settings;
pub mod attractor;
pub mod attractor_generator_settings;
pub mod branch;
pub mod leaf_classifier;
pub mod shape;
pub mod shrubbery;
pub mod voxel;
pub use glam;
pub mod math_utils;

pub mod prelude {
    pub use crate::shrubbery::{Shrubbery2d, Shrubbery3d};
}

pub mod math {
    pub use glam::*;
}
