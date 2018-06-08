use std::{cmp, ops};

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

/*
 * This is a general utility function to allow us to
 * perform various forms of vector arithmetic. The
 * functions below should be pretty self-explanatory
 */
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

impl ops::Add<Vec3> for f32 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self + other.x,
            y: self + other.y,
            z: self + other.z,
        }
    }
}

/*
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

/*
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

impl ops::Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, i: usize) -> &f32 {
        if i == 0 {
            &self.x
        } else if i == 1 {
            &self.y
        } else if i == 2 {
            &self.z
        } else {
            panic!("Invalid index for Vec3");
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

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

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

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
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

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn cross(&self, b: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * b.z - self.z * b.y,
            y: -(self.x * b.z - self.z * b.x),
            z: self.x * b.y - self.y * b.x,
        }
    }

    pub fn dot(&self, b: &Vec3) -> f32 {
        (self.x * b.x) + (self.y * b.y) + (self.z * b.z)
    }

    // Calculates the Euclidean length
    pub fn length(&self) -> f32 {
        self.squared_length().sqrt()
    }

    pub fn reflect(&self, normal: Vec3) -> Vec3 {
        let r = normal * (self.dot(&normal) * 2.0);
        self.clone() - r
    }

    pub fn squared_length(&self) -> f32 {
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

#[test]
fn test_arithmetic() {
    let v1 = Vec3::new(1.0, 0.0, 1.0);
    let v2 = Vec3::new(1.0, 1.0, 1.0);
    assert_eq!(v1 - v2, Vec3::new(0.0, -1.0, 0.0));
    assert_eq!(v1 * v2, Vec3::new(1.0, 0.0, 1.0));
    assert_eq!(v1 + v2, Vec3::new(2.0, 1.0, 2.0));
    assert_eq!(v1 / v2, Vec3::new(1.0, 0.0, 1.0));

    // Scalar arithmetic
    assert_eq!(3.0 * v1, Vec3::new(3.0, 0.0, 3.0));
    assert_eq!(3.0 + v1, Vec3::new(4.0, 3.0, 4.0));

    // Sign change
    assert_eq!(-v1, Vec3::new(-1.0, 0.0, -1.0));

    // Indexing
    let v3 = Vec3::new(1.0, 2.0, 3.0);
    assert_eq!(v3[0], 1.0);
    assert_eq!(v3[1], 2.0);
    assert_eq!(v3[2], 3.0);
}

#[test]
fn test_functionality() {
    let v1 = Vec3::new(1.0, 0.0, 1.0);
    let v2 = Vec3::new(1.0, 1.0, 1.0);

    // Dot product
    assert_eq!(v1.dot(&v2), 2.0);

    // Cross product
    assert_eq!(v1.cross(&v2), Vec3::new(-1.0, 0.0, 1.0));

    // Lengths
    assert_eq!(v1.length(), 1.4142135);
    assert_eq!(v2.squared_length(), 3.0);
    assert_eq!(Vec3::unit_vec(v1), Vec3::new(0.70710677, 0.0, 0.70710677));
}
