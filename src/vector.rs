use std::{cmp, ops};

#[derive(Debug, Copy, Clone)]
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

/**
 * Function to divide one vector with another
 */
impl ops::Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        if other.x == 0.0 || other.y == 0.0 || other.z == 0.0 {
            panic!("Denominator is 0!");
        }
        Vec3 { x: self.x / other.x, y: self.y / other.y, z: self.z / other.z }
    }
}

/**
 * Function to divide a vector with a scalar
 */
impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, scalar: f32) -> Vec3 {
        if scalar == 0.0 {
            panic!("Denominator is 0!");
        }
        Vec3 { x: self.x / scalar, y: self.y / scalar, z: self.z / scalar }
    }
}

impl cmp::PartialEq for Vec3 {
    fn eq(&self, other: &Vec3) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

/**
 * Implements the dot product of two vectors
 */
impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 { x: self.x * other.x, y: self.y * other.y, z: self.z * other.z }
    }
}

/**
 * Implements scalar multiplication
 * TODO Make this a binary operator so
 * we can do <scalar> * <Vec3> or <Vec3> * <scalar>
 */
impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f32) -> Vec3 {
        Vec3 { x: self.x * scalar, y: self.y * scalar, z: self.z * scalar }
    }
}

/**
 * Some of the functions below are mutable and therefore modify the
 * calling class.
 */
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

    // Calculates the Euclidean length
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn multiply_scalar(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
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

    pub fn unit_vec(self) -> Vec3 {
        self / self.length()
    }

    pub fn x(self) -> f32 {
        self.x
    }

    pub fn y(self) -> f32 {
        self.y
    }

    pub fn z(self) -> f32 {
        self.z
    }
}
