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

    pub fn intersect(&self, r: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for obj in &self.objs {
            if obj.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *record = temp_rec;
            }
        }
        hit_anything
    }
}
