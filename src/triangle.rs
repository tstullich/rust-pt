use hitable::{HitRecord, Hitable};
use vector::Vec3;

#[derive(Debug)]
pub struct Triangle {
    v0: Vec3,
    v1: Vec3,
    v2: Vec3,
}

impl Triangle {
    pub fn new(v0: Vec3, v1: Vec3, v2) -> Triangle {
        Triangle{ v0, v1, v2 }
    }
}
