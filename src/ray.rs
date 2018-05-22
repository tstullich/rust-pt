use vector::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    // TODO Think about making a copy of a and b instead of moving
    pub fn new(a: Vec3, b: Vec3) -> Ray {
        Ray { origin: a, direction: b }
    }

    pub fn origin(self) -> Vec3 {
        self.origin
    }

    pub fn direction(self) -> Vec3 {
        self.direction
    }

    pub fn point_at_t(self, t: f32) -> Vec3 {
        self.origin + (self.direction * t)
    }
}

#[test]
fn test_create() {
    let v1 = Vec3::new(0.5, 0.3, 0.0);
    let v2 = Vec3::new(0.0, 0.1, 0.0);
    let r = Ray::new(v1, v2);
    assert_eq!(v1, r.origin());
}
