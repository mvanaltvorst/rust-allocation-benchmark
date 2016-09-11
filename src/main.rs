use std::ops::{Add, Sub, Mul};

// Naive game implementation in Rust
fn main() {
    println!("Hello, world!");
}

struct Vector {
    x: f32, // float's in C# are single-precision, so f32 is used here instead of f64's
    y: f32,
    z: f32
}

impl Vector {
    fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector {
            x: x,
            y: y,
            z: z
        }
    }
    fn get_distance(a: Vector, b: Vector) -> f32 {
        let s = a - b;
        (s.x * s.x + s.y * s.y + s.z * s.z).sqrt()
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, _rhs: Vector) -> Vector {
        Vector {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, _rhs: Vector) -> Vector {
        Vector {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z
        }
    }
}

impl Mul for Vector {
    type Output = Vector;

    fn mul(self, _rhs: Vector) -> Vector {
        Vector {
            x: self.x * _rhs.x,
            y: self.y * _rhs.y,
            z: self.z * _rhs.z
        }
    }
}
