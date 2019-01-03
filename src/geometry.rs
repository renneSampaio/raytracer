use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Clone, Copy)]
pub struct HitInfo {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitInfo>;
}

#[derive(Copy, Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Material) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
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
                let material = self.material;

                return Some(HitInfo {
                    t,
                    p,
                    normal,
                    material,
                });
            }
            let t = (-b + discriminant.sqrt()) / a;
            if t > t_min && t < t_max {
                let p = ray.point_at_parameter(t);
                let normal = (p - self.center) / self.radius;
                let material = self.material;

                return Some(HitInfo {
                    t,
                    p,
                    normal,
                    material,
                });
            }
        }

        None
    }
}

impl Hitable for [Box<Hitable>] {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitInfo> {
        let mut closest_so_far = t_max;
        let mut hit: Option<HitInfo> = None;

        for hitable in self.iter() {
            if let Some(h) = hitable.hit(ray, t_min, closest_so_far) {
                hit = Some(h);
                closest_so_far = h.t;
            }
        }

        hit
    }
}
