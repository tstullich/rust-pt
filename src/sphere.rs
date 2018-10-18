use aabb::AABB;
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
        let r_dir = r.direction();
        let a = r_dir.dot(&r_dir);
        let b = oc.dot(&r_dir);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let sqt = discriminant.sqrt();
            let mut temp = (-b - sqt) / a;
            let mut record = HitRecord::new();
            if t_min < temp && temp < t_max {
                record.t = temp;
                record.p = r.point_at_t(temp);
                record.normal = (record.p - self.position) / self.radius;
                record.material = self.material;
                return Some(record);
            }
            temp = (-b + sqt) / a;
            if t_min < temp && temp < t_max {
                record.t = temp;
                record.p = r.point_at_t(temp);
                record.normal = (record.p - self.position) / self.radius;
                record.material = self.material;
                return Some(record);
            }
        }
        None
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        Some(AABB::new(self.position - Vec3::new(self.radius, self.radius, self.radius)), self.position +
             Vec3::new(self.radius, self.radius, self.radius))
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

pub struct MovingSphere {
    pub center0: Vec3,
    pub center1: Vec3,
    pub radius: f32,
    pub time0: f32,
    pub time1: f32,
    pub material: Material,
}

impl Hitable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin() - self.center(r.time());
        let r_dir = r.direction();
        let a = r_dir.dot(&r_dir);
        let b = oc.dot(&r_dir);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let sqt = discriminant.sqrt();
            let mut temp = (-b - sqt) / a;
            let mut record = HitRecord::new();
            if t_min < temp && temp < t_max {
                record.t = temp;
                record.p = r.point_at_t(temp);
                record.normal = (record.p - self.center(r.time())) / self.radius;
                record.material = self.material;
                return Some(record);
            }
            temp = (-b + sqt) / a;
            if t_min < temp && temp < t_max {
                record.t = temp;
                record.p = r.point_at_t(temp);
                record.normal = (record.p - self.center(r.time())) / self.radius;
                record.material = self.material;
                return Some(record);
            }
        }
        None
    }
}

impl MovingSphere {
    pub fn new(
        center0: Vec3,
        center1: Vec3,
        time0: f32,
        time1: f32,
        radius: f32,
        material: Material,
    ) -> MovingSphere {
        MovingSphere {
            center0,
            center1,
            time0,
            time1,
            radius,
            material,
        }
    }

    pub fn center(&self, time: f32) -> Vec3 {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}
