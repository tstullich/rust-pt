use hitable::{HitRecord, Hitable};
use ray::Ray;

pub struct HitableList {
    objs: Vec<Box<Hitable>>,
}

impl HitableList {
    pub fn new() -> HitableList {
        let objs: Vec<Box<Hitable>> = Vec::new();
        HitableList { objs }
    }

    pub fn add(&mut self, obj: Box<Hitable>) {
        self.objs.push(obj);
    }

    pub fn intersect(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut temp_rec = None;
        let mut closest_so_far = t_max;
        for obj in &self.objs {
            let intersect_result = obj.hit(r, t_min, closest_so_far);
            match intersect_result {
                Some(hit) => {
                    closest_so_far = hit.t;
                    temp_rec = Some(hit);
                }
                None => (),
            }
        }
        temp_rec
    }
}
