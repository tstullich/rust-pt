use ray::Ray;
use vector::Vec3;

pub struct AABB {
    min: Vec3,
    max: Vec3,
}

impl AABB {
    pub fn new(a: Vec3, b: Vec3) -> AABB {
        AABB {
            min: a,
            max: b,
        }
    }

    pub fn min(&self) -> Vec3 {
        self.min
    }

    pub fn max(&self) -> Vec3 {
        self.max
    }

    pub fn hit(&self, ray: &Ray, mut tmin: f32, mut tmax: f32) -> bool {
        for i in 0..3 {
            let inv_d: f32 = 1.0 / ray.direction()[i];
            let mut t0 = (self.min()[i] - ray.origin()[i]) * inv_d;
            let mut t1 = (self.max()[i] - ray.origin()[i]) * inv_d;
            if inv_d < 1.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            if t0 > tmin {
                tmin = t0;
            } else {
                tmin = tmin;
            }

            tmin = t0.max(tmin);
            tmax = t1.min(tmax);

            if tmax <= tmin {
                return false
            }
        }
        true
    }
}
