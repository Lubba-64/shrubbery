use glam::{vec2, Vec2, Vec3};

/// rotate a vec2 position around the origin of 0,0
pub fn rotate_point(pos: Vec2, radians: f32) -> Vec2 {
    let (cos_theta, sin_theta) = (radians.cos(), radians.sin());
    let out = vec2(
        cos_theta * pos.x - sin_theta * pos.y,
        sin_theta * pos.x + cos_theta * pos.y,
    );
    out
}

/// return the shortest distance from a vec3 to a line with a star and end pos
pub fn dist_to_line(pos: Vec3, line_start: Vec3, line_end: Vec3) -> f32 {
    let ab = line_end - line_start;
    let ac = pos - line_start;
    if ac.dot(ab) <= 0.0 {
        return ac.length();
    }
    let bv = pos - line_end;
    if bv.dot(ab) >= 0.0 {
        return bv.length();
    }
    ab.cross(ac).length() / ab.length()
}

fn vec2_cross(vec1: Vec2, vec2: Vec2) -> f32 {
    vec1.x * vec2.y - vec1.y * vec2.x
}

/// return the shortest distance from a vec2 to a line with a star and end pos
pub fn dist_to_line_2d(pos: Vec2, line_start: Vec2, line_end: Vec2) -> f32 {
    let ab = line_end - line_start;
    let ac = pos - line_start;
    if ac.dot(ab) <= 0.0 {
        return ac.length();
    }
    let bv = pos - line_end;
    if bv.dot(ab) >= 0.0 {
        return bv.length();
    }
    vec2_cross(ab, ac) / ab.length()
}
