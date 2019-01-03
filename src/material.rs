use crate::geometry::HitInfo;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Clone, Copy, Debug)]
pub enum Material {
    Lambertian { albedo: Vec3 },
    Metal { albedo: Vec3, fuzz: f32 },
    Dieletric { ref_idx: f32 },
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
        fuzz: f32,
        rng: &mut rand::RngCore,
    ) -> Option<(Ray, Vec3)> {
        let reflected = ray.direction.normalized().reflect(hit.normal);
        let scatter = Ray::new(hit.p, reflected + (fuzz * Vec3::random_in_unit_sphere(rng)));
        let attenuation = albedo;
        if scatter.direction.dot(hit.normal) > 0.0 {
            Some((scatter, attenuation))
        } else {
            None
        }
    }

    fn scatter_dieletric(
        ray: &Ray,
        hit: &HitInfo,
        ref_idx: f32,
        rng: &mut rand::RngCore,
    ) -> Option<(Ray, Vec3)> {
        let reflected = ray.direction.normalized().reflect(hit.normal);
        let attenuation = Vec3::new(1.0, 1.0, 1.0);

        let ni_over_nt: f32;
        let outward_normal: Vec3;

        if ray.direction.dot(hit.normal) > 0.0 {
            outward_normal = -hit.normal;
            ni_over_nt = ref_idx;
        } else {
            outward_normal = hit.normal;
            ni_over_nt = 1.0 / ref_idx;
        }

        if let Some(refracted) = ray.direction.refract(&outward_normal, ni_over_nt) {
            Some((Ray::new(hit.p, refracted), attenuation))
        } else {
            Some((Ray::new(hit.p, reflected), attenuation))
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
            Material::Metal { albedo, fuzz } => {
                Material::scatter_metal(albedo, ray, hit, fuzz, rng)
            }
            Material::Dieletric { ref_idx } => Material::scatter_dieletric(ray, hit, ref_idx, rng),
        }
    }
}
