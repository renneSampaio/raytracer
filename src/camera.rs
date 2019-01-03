use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lens_radius: f32,
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        up: Vec3,
        fov: f32,
        aspect: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Camera {
        let lens_radius = aperture / 2.0;
        let half_height = (fov.to_radians() / 2.0).tan();
        let half_width = aspect * half_height;

        let origin = look_from;
        let w = (look_from - look_at).normalized();
        let u = up.cross(w).normalized();
        let v = w.cross(u);

        let lower_left_corner = origin
            - (half_width * focus_dist * u)
            - (half_height * focus_dist * v)
            - w * focus_dist;

        Camera {
            origin,
            lower_left_corner,
            horizontal: u * half_width * 2.0 * focus_dist,
            vertical: v * half_height * 2.0 * focus_dist,
            lens_radius,
            u,
            v,
            w,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32, rng: &mut rand::RngCore) -> Ray {
        let point_in_lens = self.lens_radius * Vec3::random_in_unit_disk(rng);
        let offset = self.u * point_in_lens.x() + self.v * point_in_lens.y();
        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + u * self.horizontal + v * self.vertical
                - self.origin
                - offset,
        }
    }
}
