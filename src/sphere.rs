use hitable::{Hitable, HitRecord};
use ray::Ray;
use vector::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().dot(&r.direction());
        let b = oc.dot(&r.direction());
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let sqt = discriminant.sqrt();
            let mut temp = (-b - sqt) / a;
            if t_min < temp && temp < t_max {
                record.t = temp;
                record.p = r.point_at_t(record.t);
                record.normal = (record.p - self.center) / self.radius;
                return true
            }
            temp = (-b + sqt) / a;
            if t_min < temp && temp < t_max {
                record.t = temp;
                record.p = r.point_at_t(record.t);
                record.normal = (record.p - self.center).normalize();
                return true
            }
        }
        false
    }
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere { center, radius }
    }
}
