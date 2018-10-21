extern crate rand;

use aabb::AABB;
use hitable::{HitRecord, Hitable};
use hitable_list::HitableList;
use rand::Rng;
use ray::Ray;
use std::cmp::Ordering;

/// A class that encapsulates a bounding volume hierarchy
/// Using this class the render should see a significant
/// speedup when performing raycasts, by only considering
/// objects that are contained and intersect a given
/// volumes bounding box
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
                    // The left record is closer than the right one so we
                    // should prefer it
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
    /// The constructor method for the BVH tree. The idea here is that
    /// we randomly choose an axis to sort our hitable objects and then
    /// split the list in half. One half will be assigned to the left
    /// of a bounding box and vice versa. Then a recursive call is made
    /// and we try to continously split our box into smaller box until
    /// we terminate at a given n
    pub fn new(l: HitableList, n: i32, time0: f32, time1: f32) -> BvhNode {
        let mut rng = rand::thread_rng();

        // Randomly choose the x, y, or z axis to sort the hitable list over
        l.objs
            .sort_by(|a, b| BvhNode::sort(*a, *b, rng.gen_range(0, 3)).unwrap());

        let (_left, _right) = if n == 1 {
            (l.objs[0], l.objs[0])
        } else if n == 2 {
            (l.objs[0], l.objs[1])
        } else {
            (
                BvhNode::new(l, n / 2, time0, time1),
                BvhNode::new(&l.objs[n / 2..l.objs.len()], n - n / 2, time0, time1),
            )
        };

        let box_left = _left.bounding_box(0.0, 0.0);
        let box_right = _right.bounding_box(0.0, 0.0);

        if box_left.is_none() || box_right.is_none() {
            panic!("No bounding box in the constructor!");
        }

        let _bb = AABB::surrounding_box(box_left, box_right);

        BvhNode {
            left: _left,
            right: _right,
            bb: _bb,
        }
    }

    /// A function to sort our hitable objects. It is based on a partial
    /// ordering of our x/y/z values for two given hitables.
    fn sort(a: Box<Hitable>, b: Box<Hitable>, axis: i32) -> Option<Ordering> {
        let box_left = a.bounding_box(0.0, 0.0);
        let box_right = b.bounding_box(0.0, 0.0);

        if box_left.is_none() || box_right.is_none() {
            panic!("No bounding box in the constructor!");
        }

        return if axis == 0 {
            box_left
                .unwrap()
                .min()
                .x()
                .partial_cmp(&box_right.unwrap().min().x())
        } else if axis == 1 {
            box_left
                .unwrap()
                .min()
                .y()
                .partial_cmp(&box_right.unwrap().min().y())
        } else {
            box_left
                .unwrap()
                .min()
                .z()
                .partial_cmp(&box_right.unwrap().min().z())
        };
    }
}
