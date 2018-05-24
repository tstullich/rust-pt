use hitable::HitRecord;
use rand::{thread_rng, Rng};
use ray::Ray;
use vector::Vec3;

pub trait Material {
    fn scatter(self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let target = rec.p + rec.normal + Lambertian::random_unit_in_sphere();
        *scattered = Ray::new(rec.p, target - rec.p);
        *attenuation = self.albedo;
        true
    }
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo }
    }

    pub fn random_unit_in_sphere() -> Vec3 {
        let mut rng = thread_rng();
        let mut p = Vec3::new(rng.gen_range(0.0, 1.0),
                              rng.gen_range(0.0, 1.0),
                              rng.gen_range(0.0, 1.0));

        while p.squared_length() >= 1.0 {
            let rand_x: f32 = rng.gen_range(0.0, 1.0);
            let rand_y: f32 = rng.gen_range(0.0, 1.0);
            let rand_z: f32 = rng.gen_range(0.0, 1.0);
            p = Vec3::new(rand_x, rand_y, rand_z) * 2.0;
            p = p - Vec3::new(1.0, 1.0, 1.0);
        }
        p
    }
}

pub struct Metal {
    albedo: Vec3,
}

impl Material for Metal {
    fn scatter(self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let reflected = Vec3::unit_vec(ray_in.direction()).reflect(rec.normal);
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;

        scattered.direction().dot(&rec.normal) > 0.0
    }
}

impl Metal {
    pub fn new(albedo: Vec3) -> Metal {
        Metal { albedo }
    }
}
