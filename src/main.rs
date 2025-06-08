// use std::io::{self, Stdin};
// use std::io;
use physics_engine_2d::*;

// fn main() {
//     println!("New 2d Physiscs Engine");
//     // println!("Guess the number");

//     // let mut guess = String::new();

//     // io::stdin()
//     //     .read_line(&mut guess)
//     //     .expect("Failed to read line");

//     // println!("You guessed: {}", guess);
    
//     let vec  = vector2d::Vector2D::new(10.,20.);
//     println!("Vector: {}",vec);

// }

fn main() {
    let mut world = World::new();
    
    // Create ground (static body)
    let ground = RigidBody::new(
        Vector2D::new(400.0, 550.0),
        Shape::rectangle(800.0, 100.0),
        0.0, // mass = 0 makes it static
    );
    world.add_body(ground);
    
    // Create falling circles
    for i in 0..5 {
        let circle = RigidBody::new(
            Vector2D::new(300.0 + i as f32 * 50.0, 100.0),
            Shape::circle(20.0),
            1.0,
        );
        world.add_body(circle);
    }
    
    // Create a box
    let mut box_body = RigidBody::new(
        Vector2D::new(500.0, 200.0),
        Shape::rectangle(40.0, 40.0),
        2.0,
    );
    box_body.restitution = 0.6;
    world.add_body(box_body);
    
    // Simulation loop
    let dt = 1.0 / 60.0; // 60 FPS
    for frame in 0..1000 {
        world.step(dt);
        
        // Print positions every 30 frames
        if frame % 30 == 0 {
            println!("Frame {}", frame);
            for (i, body) in world.bodies.iter().enumerate() {
                if !body.is_static {
                    println!("  Body {}: pos({:.1}, {:.1}) vel({:.1}, {:.1})", 
                           i, body.position.x, body.position.y, 
                           body.velocity.x, body.velocity.y);
                }
            }
            println!();
        }
    }
}