use vector::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
    time: f32,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3, time: f32) -> Ray {
        Ray {
            origin,
            direction,
            time,
        }
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn time(&self) -> f32 {
        self.time
    }

    pub fn point_at_t(&self, t: f32) -> Vec3 {
        self.origin + (self.direction * t)
    }
}

#[test]
fn test_create() {
    let v1 = Vec3::new(0.5, 0.3, 0.0);
    let v2 = Vec3::new(0.0, 0.1, 0.0);
    let r = Ray::new(v1, v2, 0.0);
    assert_eq!(v1, r.origin());
    assert_eq!(v2, r.direction());
}

#[test]
fn test_point_at() {
    let v1 = Vec3::new(0.5, 0.3, 0.0);
    let v2 = Vec3::new(0.0, 0.1, 0.0);
    let r = Ray::new(v1, v2, 0.0);
    assert_eq!(r.point_at_t(1.0), r.origin() + r.direction());
    assert_ne!(r.point_at_t(2.0), r.origin() + r.direction());
}
