use crate::geometry::HitInfo;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Clone, Copy, Debug)]
pub enum Material {
    Lambertian { albedo: Vec3 },
    Metal { albedo: Vec3 },
}

impl Material {
    fn scatter_lambertian(
        albedo: Vec3,
        _: &Ray,
        hit: &HitInfo,
        rng: &mut rand::RngCore,
    ) -> Option<(Ray, Vec3)> {
        let target = hit.p + hit.normal + Vec3::random_in_unit_sphere(rng);
        let scatter = Ray::new(hit.p, target - hit.p);
        let attenuation = albedo;

        Some((scatter, attenuation))
    }

    fn scatter_metal(
        albedo: Vec3,
        ray: &Ray,
        hit: &HitInfo,
        _: &mut rand::RngCore,
    ) -> Option<(Ray, Vec3)> {
        let reflected = ray.direction.normalized().reflect(hit.normal);
        let scatter = Ray::new(hit.p, reflected);
        let attenuation = albedo;
        if scatter.direction.dot(hit.normal) > 0.0 {
            Some((scatter, attenuation))
        } else {
            None
        }
    }

    pub fn scatter(
        &self,
        ray: &Ray,
        hit: &HitInfo,
        rng: &mut rand::RngCore,
    ) -> Option<(Ray, Vec3)> {
        match *self {
            Material::Lambertian { albedo } => Material::scatter_lambertian(albedo, ray, hit, rng),
            Material::Metal { albedo } => Material::scatter_metal(albedo, ray, hit, rng),
        }
    }
}
