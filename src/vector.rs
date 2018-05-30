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
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
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
        Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
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
        Vec3 {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
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
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
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
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ops::Sub<f32> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: f32) -> Vec3 {
        Vec3 {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
        }
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

    pub fn dot(&self, b: &Vec3) -> f32 {
        (self.x * b.x) + (self.y * b.y) + (self.z * b.z)
    }

    // Calculates the Euclidean length
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn reflect(&self, b: Vec3) -> Vec3 {
        let r = b * (self.dot(&b) * 2.0);
        self.clone() - r
    }

    pub fn set_x(mut self, x: f32) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }

    pub fn squared_length(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn unit_vec(v: Vec3) -> Vec3 {
        v / v.length()
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }
}
