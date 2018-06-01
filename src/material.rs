use hitable::HitRecord;
use rand::{thread_rng, Rng};
use ray::Ray;
use vector::Vec3;

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian(Vec3),
    Metal(Vec3, f32),
    Dielectric(Vec3, f32),
}

impl Material {
    /* A generalized scatter function based on the type of material
     * that is specified for the surface. Currently there are three
     * options available are:
     * 1. Lambertian diffuse surface
     * 2. Metal surface with a tune-able fuzzy factor
     * 3. Dielectric surfaces with specular reflection
     * The return type of Option<Ray> allows us to indicate if ray was
     * reflected or not. In case of the metal material, the light might
     * not be reflected
     */
    pub fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<Ray> {
        match &self {
            Material::Lambertian(_) => self.lambertian(rec),
            Material::Metal(_, fuzz) => {
                // Going to check if fuzz was properly set
                // and give it an upper bound of 1.0
                let fuzz_factor = if *fuzz <= 1.0 { *fuzz } else { 1.0 };
                self.metal(ray, fuzz_factor, rec)
            }
            Material::Dielectric(_, ri) => self.dielectric(*ri, ray, rec),
        }
    }

    /*
     * Scatter function for a Lambertian diffuse surface.
     */
    fn lambertian(&self, rec: &HitRecord) -> Option<Ray> {
        let target = rec.p + rec.normal + self.random_unit_in_sphere();
        Some(Ray::new(rec.p, target - rec.p))
    }

    /*
     * Scatter function for a metal surface. We are able to adjust
     * the "roughess" of the surface through a fuzzy factor that
     * makes it so the surface scatters more light and the reflection
     * starts to become more diffuse.
     */
    fn metal(&self, ray: &Ray, fuzz: f32, rec: &HitRecord) -> Option<Ray> {
        let reflected = Vec3::unit_vec(ray.direction()).reflect(rec.normal);
        let fuzzed_reflector = reflected + self.random_unit_in_sphere() * fuzz;
        let scattered = Ray::new(rec.p, fuzzed_reflector);

        return if scattered.direction().dot(&rec.normal) > 0.0 {
            Some(scattered)
        } else {
            None
        };
    }

    // Calculates the next outgoing ray for a dielectric surface.
    fn dielectric(&self, ref_idx: f32, ray: &Ray, rec: &HitRecord) -> Option<Ray> {
        let reflected = ray.direction().reflect(rec.normal);
        let (outward_normal, ni_over_nt, cosine) = if ray.direction().dot(&rec.normal) > 0.0 {
            let outward_normal = rec.normal * -1.0;
            let ni_over_nt = ref_idx;
            let cosine = ref_idx * ray.direction().dot(&rec.normal) / ray.direction().length();
            (outward_normal, ni_over_nt, cosine)
        } else {
            let outward_normal = rec.normal;
            let ni_over_nt = 1.0 / ref_idx;
            let cosine = -(ray.direction().dot(&rec.normal)) / ray.direction().length();
            (outward_normal, ni_over_nt, cosine)
        };

        let refracted = self.refract(&ray.direction(), &outward_normal, ni_over_nt);
        let reflect_prob = if refracted.is_some() {
            self.schlick(cosine, ref_idx)
        } else {
            1.0
        };

        let mut rng = thread_rng();
        if rng.gen_range(0.0, 1.0) < reflect_prob {
            Some(Ray::new(rec.p, reflected))
        } else {
            Some(Ray::new(rec.p, refracted.unwrap()))
        }
    }

    /*
     * This is used in the diffuse and metal surface reflection calculations
     * to find a new random vector to reflect to
     */
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

    // Calculates the refraction angle if we are using a dielectric material
    fn refract(&self, v: &Vec3, normal: &Vec3, ni_over_nt: f32) -> Option<Vec3> {
        let uv = Vec3::unit_vec(*v);
        let dt = uv.dot(normal);
        let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
        return if discriminant > 0.0 {
            let r = (uv - *normal * dt) * ni_over_nt - (*normal * discriminant.sqrt());
            Some(r)
        } else {
            None
        };
    }

    // Calculates the Fresnel factor in a specular reflection
    fn schlick(&self, cosine: f32, ref_idx: f32) -> f32 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 *= r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }

    pub fn color(self) -> Vec3 {
        match self {
            Material::Lambertian(color) => color,
            Material::Metal(color, _) => color,
            Material::Dielectric(color, _) => color,
        }
    }
}
