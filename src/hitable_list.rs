use hitable::{Hitable, HitRecord};
use ray::Ray;
use sphere::Sphere;
use vector::Vec3;

pub struct HitableList {
    objs: Vec<Box<Hitable>>,
}

impl Hitable for HitableList {
    fn hit(self, r: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool {
        let temp_rec = HitRecord::new();
        let hit_anything = false;
        let closest_so_far = t_max;
        for obj in self.objs {
            if obj.hit(r, t_min, closest_so_far, temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *record = temp_rec;
            }
        }
        return hit_anything
    }
}

impl HitableList {
    pub fn new() -> HitableList {
        let objs: Vec<Box<Hitable>> = Vec::new();
        HitableList { objs }
    }

    pub fn add(self, obj: Box<Hitable>) {
        self.objs.push(obj);
    }

    fn create_objects(self) {
        self.objs.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
        self.objs.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));
    }
}
