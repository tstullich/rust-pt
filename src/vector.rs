use std::ops;

#[derive(Debug)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl ops::Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 { x: self.x * other.x, y: self.y * other.y, z: self.z * other.z }
    }
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn add(&mut self, b: &Vec3) {
        self.x += b.x;
        self.y += b.y;
        self.z += b.z;
    }

    pub fn add_scalar(&mut self, scalar: i32) {
        self.x += scalar as f32;
        self.y += scalar as f32;
        self.z += scalar as f32;
    }

    // This assumes we are in a right-hand coordinate system
    pub fn cross(&self, b: &Vec3) -> Vec3 {
        let new_x = (self.y * b.z) - (self.z * b.y);
        let new_y= (self.z * b.x) - (self.x * b.z);
        let new_z = (self.x * b.y) - (self.y * b.x);

        Vec3 { x: new_x, y: new_y, z: new_z }
    }

    pub fn dot(&self, b: &Vec3) -> f32 {
        (self.x * b.x) + (self.y * b.y) + (self.z * b.z)
    }

    // Calculates the Euclidean length from (0, 0, 0)
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn multiply_scalar(&mut self, scalar: i32) {
        self.x *= scalar as f32;
        self.y *= scalar as f32;
        self.z *= scalar as f32;
    }

    pub fn normalize(&self) -> Vec3 {
        let mag = Vec3::length(self);
        Vec3 { x: self.x / mag, y: self.y / mag, z: self.z / mag }
    }

    pub fn subtract(&mut self, b: &Vec3) {
        self.x -= b.x;
        self.y -= b.y;
        self.z -= b.z;
    }

    pub fn subtract_scalar(&mut self, scalar: i32) {
        self.x -= scalar as f32;
        self.y -= scalar as f32;
        self.z -= scalar as f32;
    }
}
