use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, up: Vec3, fov: f32, aspect: f32) -> Camera {
        let half_height = (fov.to_radians() / 2.0).tan();
        let half_width = aspect * half_height;

        let origin = look_from;
        let w = (look_from - look_at).normalized();
        let u = up.cross(w).normalized();
        let v = w.cross(u);

        let lower_left_corner = origin - (half_width * u) - (half_height * v) - w;

        Camera {
            origin,
            lower_left_corner,
            horizontal: u * half_width * 2.0,
            vertical: v * half_height * 2.0,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + u * self.horizontal + v * self.vertical
                - self.origin,
        }
    }
}
