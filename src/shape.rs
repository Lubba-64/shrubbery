use crate::{
    algorithm_settings::AlgorithmSettings,
    attractor::{Attractor2d, Attractor3d},
    attractor_generator_settings::AttractorGeneratorSettings,
};
use glam::*;
use rand::Rng;

/// A shape to spawn attractors inside
pub trait Shape3d {
    fn generate(
        &self,
        pos: Vec3,
        attractors: &mut Vec<Attractor3d>,
        algorithm_settings: &AlgorithmSettings,
        generator_settings: &AttractorGeneratorSettings,
    );
}

/// x,y,z is total size
pub struct BoxShape3d(pub Vec3);

impl Shape3d for BoxShape3d {
    fn generate(
        &self,
        pos: Vec3,
        attractors: &mut Vec<Attractor3d>,
        algorithm_settings: &AlgorithmSettings,
        generator_settings: &AttractorGeneratorSettings,
    ) {
        let mut ideal_spacing =
            algorithm_settings.leaf_attraction_dist * 0.5 - algorithm_settings.kill_distance * 0.5;
        ideal_spacing *= 1.0 / generator_settings.density;

        let x_l = (self.0.x / ideal_spacing) as i32;
        let x_y = (self.0.y / ideal_spacing) as i32;
        let x_z = (self.0.z / ideal_spacing) as i32;

        let center_shape_offset = -vec3(self.0.x * 0.5, self.0.y * 0.5, self.0.z * 0.5);
        let start_pos = vec3(
            ideal_spacing * 0.5,
            ideal_spacing * 0.5,
            ideal_spacing * 0.5,
        );

        let scatter_distance = ideal_spacing * 0.5;
        let mut rng = rand::thread_rng();

        for x in 0..x_l {
            for y in 0..x_y {
                for z in 0..x_z {
                    let jitter = vec3(
                        x as f32 * ideal_spacing
                            + rng.gen_range(-scatter_distance..scatter_distance),
                        y as f32 * ideal_spacing
                            + rng.gen_range(-scatter_distance..scatter_distance),
                        z as f32 * ideal_spacing
                            + rng.gen_range(-scatter_distance..scatter_distance),
                    );
                    attractors.push(Attractor3d::new(
                        pos + start_pos + center_shape_offset + jitter,
                    ));
                }
            }
        }
    }
}

/// A shape to spawn attractors inside
pub trait Shape2d {
    fn generate(
        &self,
        pos: Vec2,
        attractors: &mut Vec<Attractor2d>,
        algorithm_settings: &AlgorithmSettings,
        generator_settings: &AttractorGeneratorSettings,
    );
}

/// x,y is total size
pub struct BoxShape2d(pub Vec2);

impl Shape2d for BoxShape2d {
    fn generate(
        &self,
        pos: Vec2,
        attractors: &mut Vec<Attractor2d>,
        algorithm_settings: &AlgorithmSettings,
        generator_settings: &AttractorGeneratorSettings,
    ) {
        let mut ideal_spacing =
            algorithm_settings.leaf_attraction_dist * 0.5 - algorithm_settings.kill_distance * 0.5;
        ideal_spacing *= 1.0 / generator_settings.density;

        let x_l = (self.0.x / ideal_spacing) as i32;
        let x_y = (self.0.y / ideal_spacing) as i32;

        let center_shape_offset = -vec2(self.0.x * 0.5, self.0.y * 0.5);
        let start_pos = vec2(ideal_spacing * 0.5, ideal_spacing * 0.5);

        let scatter_distance = ideal_spacing * 0.5;
        let mut rng = rand::thread_rng();

        for x in 0..x_l {
            for y in 0..x_y {
                let jitter = vec2(
                    x as f32 * ideal_spacing + rng.gen_range(-scatter_distance..scatter_distance),
                    y as f32 * ideal_spacing + rng.gen_range(-scatter_distance..scatter_distance),
                );
                attractors.push(Attractor2d::new(
                    pos + start_pos + center_shape_offset + jitter,
                ));
            }
        }
    }
}
