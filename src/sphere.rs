use hitable::{Hitable, HitRecord};
use ray::Ray;
use vector::Vec3;

struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Hitable for Sphere {
    fn hit(self, r: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().dot(&r.direction());
        let b = oc.dot(&r.direction()) * 2.0;
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - (a * c * 4.0);

        if discriminant > 0.0 {
            let mut temp = (-b - discriminant.sqrt()) / a;
            if t_min < temp && temp < t_max {
                record.t = temp;
                record.p = r.point_at_t(record.t);
                record.normal = (record.p - self.center) / self.radius;
                return true
            }
            temp = (-b + (b * b - a * c).sqrt()) / a;
            if t_min < temp && temp < t_max {
                record.t = temp;
                record.p = r.point_at_t(record.t);
                record.normal = (record.p - self.center) / self.radius;
                return true
            }
        }
        return false
    }
}

impl Sphere {
    fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere {center, radius}
    }
}
