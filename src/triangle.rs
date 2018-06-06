use hitable::{HitRecord, Hitable};
use material::Material;
use ray::Ray;
use vector::Vec3;

pub struct Triangle {
    v0: Vec3,
    v1: Vec3,
    v2: Vec3,
    material: Material,
}

impl Hitable for Triangle {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let epsilon = 0.0000001;
        let edge1 = self.v1 - self.v0;
        let edge2 = self.v2 - self.v0;
        let h = &r.direction().cross(&edge2);
        let a = edge1.dot(&h);

        if a > -epsilon && a < epsilon {
            return None
        }

        let f = 1.0 / a;
        let s = r.origin() - self.v0;
        let u = f * (s.dot(&h));
        if u < 0.0 || u > 1.0 {
            return None
        }

        let q = s.cross(&edge1);
        let v = f * r.direction().dot(&q);
        if v < 0.0 || u + v > 1.0 {
            return None
        }

        let t = f * edge2.dot(&q);
        if t_min < t && t < t_max {
            let mut record = HitRecord::new();
            record.p = r.point_at_t(t);
            record.normal = self.normal();
            record.material = self.material;
            return Some(record)
        }

        // There is a line intersection but not a ray intersection
        // according to the wikipedia page of the Moeller-Trumbore algorithm
        None
    }
}

impl Triangle {
    pub fn new(v0: Vec3, v1: Vec3, v2: Vec3, material: Material) -> Triangle {
        Triangle { v0, v1, v2, material }
    }

    fn normal(&self) -> Vec3 {
        let u = self.v1 - self.v0;
        let v = self.v2 - self.v0;

        let normal_x = (u.y() * v.z()) - (u.z() * v.y());
        let normal_y = (u.z() * v.x()) - (u.x() * v.z());
        let normal_z = (u.x() * v.y()) - (u.y() * v.x());

        Vec3::new(normal_x, normal_y, normal_z)
    }
}
