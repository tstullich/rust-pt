use hitable::{HitRecord, Hitable};
use material::Material;
use ray::Ray;
use vector::Vec3;

pub struct Sphere {
    pub position: Vec3,
    pub radius: f32,
    pub material: Material,
}

/*
 * My simple implementation for a sphere object. Going to check if I
 * can improve the performance of the intersection test code somehow.
 */
impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin() - self.position;
        let a = r.direction().dot(&r.direction());
        let b = oc.dot(&r.direction());
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let sqt = discriminant.sqrt();
            let mut temp = (-b - sqt) / a;
            let mut record = HitRecord::new();
            if t_min < temp && temp < t_max {
                record.t = temp;
                record.p = r.point_at_t(record.t);
                record.normal = (record.p - self.position) / self.radius;
                record.material = self.material;
                return Some(record);
            }
            temp = (-b + sqt) / a;
            if t_min < temp && temp < t_max {
                record.t = temp;
                record.p = r.point_at_t(record.t);
                record.normal = (record.p - self.position) / self.radius;
                record.material = self.material;
                return Some(record);
            }
        }
        None
    }
}

impl Sphere {
    pub fn new(position: Vec3, radius: f32, material: Material) -> Sphere {
        Sphere {
            position,
            radius,
            material,
        }
    }
}
