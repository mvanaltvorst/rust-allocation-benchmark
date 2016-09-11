// Naive game implementation in Rust
extern crate time;

use time::PreciseTime;
use std::ops::{Add, Sub, Mul};

static NUM_BLOCKS: usize = 65535;
static NUM_ENTITIES: usize = 1000;
static CHUNK_COUNT: usize = 100;

fn main() {
    // TODO: Implement timing
    println!("Loading World...");
    let start = PreciseTime::now();
    let mut game = Game::new();
    let end = PreciseTime::now();
    println!("FINISHED!");
    println!("Load Time: {}s", start.to(end));

    loop {
        let start = PreciseTime::now();
        let playerMovement = Vector::new(0.1, 0.0, 0.0);
        game.player_location = game.player_location + playerMovement;
        game.update_chunks();
        let end = PreciseTime::now();
        println!("{}ms", start.to(end).num_milliseconds());
    }
}

/*
Game
*/
struct Game {
    chunks: Vec<Chunk>,
    player_location: Vector,
    chunk_counter: f32
}

impl Game {
    fn new() -> Game {
        let mut chunk_counter = 0.0f32;
        let chunks = {
            let mut chunks = Vec::with_capacity(CHUNK_COUNT);
            for i in 0..CHUNK_COUNT {
                chunks.push(Chunk::new(Vector::new(chunk_counter, 0.0, 0.0)));
                chunk_counter += 1.0;
            }
            chunks
        };

        Game {
            chunks: chunks,
            player_location: Vector::new(0.0, 0.0, 0.0),
            chunk_counter: chunk_counter
        }
    }

    fn update_chunks(&mut self) {
        let mut amount_of_chunks_removed = 0;
        for chunk in self.chunks.iter_mut() {
            chunk.process_entites();
        }

        let player_location = self.player_location.clone();

        self.chunks.retain(| x | {
            let chunkDistance = Vector::get_distance(x.location, player_location);
            if chunkDistance > CHUNK_COUNT as f32 {
                amount_of_chunks_removed += 1;
                return false;
            }
            true
        });

        for _ in 0..amount_of_chunks_removed {
            self.chunks.push(Chunk::new(Vector::new(self.chunk_counter, 0.0, 0.0)));
            self.chunk_counter += 1.0;
        }
    }
}

/*
Chunk
*/
struct Chunk {
    blocks: Vec<Block>,
    entities: Vec<Entity>,
    location: Vector
}

impl Chunk {
    fn new(location: Vector) -> Chunk {
        let blocks = {
            let mut blocks = Vec::with_capacity(NUM_BLOCKS);
            for i in 0..NUM_BLOCKS {
                let converted_i = i as f32;
                let block = Block::new(
                    Vector::new(converted_i, converted_i, converted_i),
                    format!("Block: {}", i).to_string(),
                    100,
                    1,
                    true,
                    true,
                    1
                );
                blocks.push(block);
            }
            blocks
        };

        let entities = {
            let mut entities = Vec::with_capacity(NUM_ENTITIES);
            for i in 0..(NUM_ENTITIES / 4) {
                let converted_i = i as f32;
                entities.push(Entity::new(
                    Vector::new(converted_i, converted_i, converted_i),
                    EntityType::Chicken
                ));
                entities.push(Entity::new(
                    Vector::new(converted_i + 2.0, converted_i, converted_i),
                    EntityType::Chicken
                ));
                entities.push(Entity::new(
                    Vector::new(converted_i + 3.0, converted_i, converted_i),
                    EntityType::Chicken
                ));
                entities.push(Entity::new(
                    Vector::new(converted_i + 4.0, converted_i, converted_i),
                    EntityType::Chicken
                ));
            }
            entities
        };


        Chunk {
            location: location,
            blocks: blocks,
            entities: entities
        }
    }

    fn process_entites(&mut self) {
        for entity in self.entities.iter_mut() {
            entity.update_position();
        }
    }
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

    fn update_position(&mut self) {
        let movementVector = Vector::new(1.0, 1.0, 1.0) * self.speed.clone();
        self.location = self.location.clone() + movementVector;
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

#[derive(PartialEq, Debug, Clone, Copy)]
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
