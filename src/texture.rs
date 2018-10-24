use vector::Vec3;

pub trait Texture {
    fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3;
}

#[derive(Copy, Clone)]
pub struct Constant {
    color: Vec3,
}

impl Texture for Constant {
    fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        self.color
    }
}

impl Constant {
    pub fn new(v: Vec3) -> Constant {
        Constant { color: v }
    }
}

#[derive(Copy, Clone)]
pub struct Checker {
    odd: Box<Texture>,
    even: Box<Texture>,
}

impl Texture for Checker {
    fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        let sines = f32::sin(f32::sin(10 * p.x()) * f32::sin(10 * p.y()) * f32::sin(10 * p.z()));
        if sines < 0.0 {
            self.odd.value(u, v, p);
        } else {
            self.even.value(u, v, p);
        }
    }
}

impl Checker {
    pub fn new(t0: &Box<Texture>, t1: &Box<Texture>) -> Checker {
        Checker {
            odd: t0,
            even: t1,
        }
    }
}
