use aabb::AABB;
use hitable::Hitable;
use material::Material;

pub struct BVH_Node {
    left: Box<Hitable>,
    right: Box<Hitable>,
    bb: AABB,
}

impl Hitable for BVH_Node {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {

    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
    }
}

impl BVH_Node {
    pub fn new(l: Hitable, n: int, time0: f32, time1: f32) -> BVH_Node {

    }
}
