extern crate rand;

use aabb::AABB;
use hitable::{HitRecord, Hitable};
use hitable_list::HitableList;
use rand::{thread_rng, Rng};
use ray::Ray;

pub struct BvhNode {
    left: Box<Hitable>,
    right: Box<Hitable>,
    bb: AABB,
}

impl Hitable for BvhNode {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        // This if/else structure seems pretty messy.
        // Maybe this can be cleaned up a bit
        if self.bb.hit(ray, t_min, t_max) {
            let left_hit = self.left.hit(ray, t_min, t_max);
            let right_hit = self.right.hit(ray, t_min, t_max);

            if left_hit.is_some() && right_hit.is_some() {
                let left_rec = left_hit.unwrap();
                let right_rec = right_hit.unwrap();
                if left_rec.t < right_rec.t {
                    return Some(left_rec);
                } else {
                    return Some(right_rec);
                }
            } else if left_hit.is_some() {
                return Some(left_hit.unwrap());
            } else if right_hit.is_some() {
                return Some(right_hit.unwrap());
            } else {
                return None;
            }
        }
        None
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        // Going to make a simple copy here
        Some(self.bb.clone())
    }
}

impl BvhNode {
    //pub fn new(l: HitableList, n: i32, time0: f32, time1: f32) -> BvhNode {
    //    let mut rng = rand::thread_rng();
    //    let axis = rng.gen_range(0, 3);

    //    if axis == 0 {
    //    } else if axis == 1{
    //    } else {
    //    }
    //}

    fn sort_x(&self) {
    }
}
