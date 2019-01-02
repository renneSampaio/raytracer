use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitInfo {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitInfo>;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitInfo> {
        let oc = ray.origin - self.center;
        let a = ray.direction.lenght_squared();
        let b = oc.dot(ray.direction);
        let c = oc.lenght_squared() - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let t = (-b - discriminant.sqrt()) / a;
            if t > t_min && t < t_max {
                let p = ray.point_at_parameter(t);
                let normal = (p - self.center) / self.radius;

                return Some(HitInfo { t, p, normal });
            }
            let t = (-b + discriminant.sqrt()) / a;
            if t > t_min && t < t_max {
                let p = ray.point_at_parameter(t);
                let normal = (p - self.center) / self.radius;

                return Some(HitInfo { t, p, normal });
            }
        }

        None
    }
}
