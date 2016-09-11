// Naive game implementation in Rust
use std::ops::{Add, Sub, Mul};

fn main() {
    println!("Hello, world!");
}

/*
Entity
*/
enum EntityType {
    Zombie,
    Chicken,
    Exploder,
    TallCreepyThing
}

struct Entity {
    location: Vector,
    name: String,
    health: i32,
    speed: Vector,
}

impl Entity {
    fn new(location: Vector, entity_type: EntityType) -> Entity {
        match entity_type {
            EntityType::Zombie => Entity {
                location: location,
                name: "Zombie".to_string(),
                health: 50,
                speed: Vector::new(0.5, 0.0, 0.5) // slow, can't fly
            },
            EntityType::Chicken => Entity {
                location: location,
                name: "Chicken".to_string(),
                health: 25,
                speed: Vector::new(0.75, 0.25, 0.75) // can fly a bit
            },
            EntityType::Exploder => Entity {
                location: location,
                name: "Exploder".to_string(),
                health: 75,
                speed: Vector::new(0.75, 0.0, 0.75)
            },
            EntityType::TallCreepyThing => Entity {
                location: location,
                name: "Tall Creepy Thing".to_string(),
                health: 500,
                speed: Vector::new(1.0, 1.0, 1.0) // does what he wants
            }
        }
    }
}

/*
Block
*/
struct Block {
    location: Vector,
    name: String,
    durability: i32,
    textureid: usize,
    breakable: bool,
    visible: bool,
    block_type: i32
}

impl Block {
    //TODO Make new() more elegant
    fn new(location: Vector,
           name: String,
           durability: i32,
           textureid: usize,
           breakable: bool,
           visible: bool,
           block_type: i32) -> Block {
        Block {
            location: location,
            name: name,
            durability: durability,
            textureid: textureid,
            breakable: breakable,
            visible: visible,
            block_type: block_type
        }
    }
}

/*
Vector
*/
#[test]
fn test_vector() {
    let a = Vector::new(3.0, 2.0, 1.0);
    let b = Vector::new(6.5, 3.0, 5.5);
    assert_eq!(a + b, Vector::new(9.5, 5.0, 6.5));

    let a = Vector::new(3.0, 2.0, 1.0);
    let b = Vector::new(2.0, 4.0, 5.0);
    assert_eq!(a * b, Vector::new(6.0, 8.0, 5.0));

    let a = Vector::new(3.0, 3.0, 1.0);
    let b = Vector::new(2.0, 1.0, 2.0);
    assert_eq!(a - b, Vector::new(1.0, 2.0, -1.0));
}

#[derive(PartialEq, Debug)]
struct Vector {
    x: f32, // floats in C# are single-precision, so f32 is used here instead of f64
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
