use aabb::AABB;
use hitable::{HitRecord, Hitable};
use ray::Ray;

pub struct HitableList {
    objs: Vec<Box<Hitable>>,
}

/*
 * A list that holds our intersectable objects. I tried to make
 * this as generic as possible for now but I am sure I can make this
 * better once I know more about Rust
 */
impl HitableList {
    pub fn new() -> HitableList {
        let objs: Vec<Box<Hitable>> = Vec::new();
        HitableList { objs }
    }

    pub fn push(&mut self, obj: Box<Hitable>) {
        self.objs.push(obj);
    }

    /*
     * A function to find the object that is closest to the current view point
     */
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

    pub fn len(&self) -> usize {
        self.objs.len()
    }

    pub fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        if self.objs.len() < 1 {
            return None;
        }
        let first_true = self.objs[0].bounding_box(t0, t1);

        let mut bb = if first_true.is_none() {
            None
        } else {
            first_true
        };

        if bb.is_none() {
            return bb
        }

        // TODO Check this for correctness at some point
        for i in 1..self.objs.len() {
            let temp = self.objs[0].bounding_box(t0, t1);
            if temp.is_some() {
                bb = Some(AABB::surrounding_box(bb.unwrap(), temp.unwrap()));
            } else {
                return None;
            }
        }

        return bb;
    }
}

#[test]
fn test_intersection() {
    let mut list = HitableList::new();
    list.push(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, 0.0),
        1.0,
        Material::Lambertian(Vec3::new(0.0, 0.0, 0.0)),
    )));

    // Setting up a ray that is in front of the sphere going directly into it
    let ray = Ray::new(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, -1.0), 0.0);
    assert!(list.intersect(&ray, 0.001, 10.0).is_some());

    // Setting up a ray that is not going to hit the sphere
    let ray = Ray::new(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 5.0, 0.0), 0.0);
    assert!(list.intersect(&ray, 0.001, 10.0).is_none());

    // Going to test the same thing with a MovingSphere
    list.objs.clear();

    list.push(Box::new(MovingSphere::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
        0.0,
        1.0,
        1.0,
        Material::Lambertian(Vec3::new(0.0, 0.0, 0.0)),
    )));

    let ray = Ray::new(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, -1.0), 0.0);
    assert!(list.intersect(&ray, 0.001, 10.0).is_some());

    let ray = Ray::new(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 5.0, 0.0), 0.0);
    assert!(list.intersect(&ray, 0.001, 10.0).is_none());
}
