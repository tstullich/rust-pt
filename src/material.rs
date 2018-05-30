use hitable::HitRecord;
use rand::{thread_rng, Rng};
use ray::Ray;
use vector::Vec3;

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian(Vec3),
    Metal(Vec3),
}

impl Material {
    pub fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: & Vec3) -> Option<Ray> {
        match &self {
            Material::Lambertian(ref albedo) => self.lambertian(ray_in, rec, albedo),
            Material::Metal(ref albedo) => self.metal(ray_in, rec, albedo),
        }
    }

    fn random_unit_in_sphere(&self) -> Vec3 {
        let mut rng = thread_rng();
        let mut p = Vec3::new(
            rng.gen_range(0.0, 1.0),
            rng.gen_range(0.0, 1.0),
            rng.gen_range(0.0, 1.0),
        );

        while p.squared_length() >= 1.0 {
            let rand_x: f32 = rng.gen_range(0.0, 1.0);
            let rand_y: f32 = rng.gen_range(0.0, 1.0);
            let rand_z: f32 = rng.gen_range(0.0, 1.0);
            p = Vec3::new(rand_x, rand_y, rand_z) * 2.0;
            p = p - Vec3::new(1.0, 1.0, 1.0);
        }
        p
    }

    fn lambertian(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &Vec3) -> Option<Ray> {
        let target = rec.p + rec.normal + self.random_unit_in_sphere();
        // TODO Fix so albedo is value set from enum
        //*attenuation = Vec3::new(0.0, 0.0, 0.0);
        Some(Ray::new(rec.p, target - rec.p))
    }

    fn metal(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &Vec3) -> Option<Ray> {
        let reflected = Vec3::unit_vec(ray_in.direction()).reflect(rec.normal);
        let scattered = Ray::new(rec.p, reflected);
        // TODO Fix so albedo is value set from enum
        //*attenuation = Vec3::new(0.0, 0.0, 0.0);

        return if scattered.direction().dot(&rec.normal) > 0.0 {
            Some(scattered)
        } else {
            None
        }
    }

    pub fn attenuation(&self) -> &Vec3 {
        match &self {
            Material::Lambertian(ref albedo) => albedo,
            Material::Metal(ref albedo) => albedo,
        }
    }
}
