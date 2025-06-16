use physics_engine_2d::*;
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    let mut window = Window::new(
        "2D Physics Engine - Press SPACE to add objects, R to reset",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });
    
    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    
    let mut world = World::new();
    let mut renderer = Renderer::new(WIDTH, HEIGHT);
    
    // Create boundaries
    setup_boundaries(&mut world);
    
    // Add some initial objects
    setup_initial_scene(&mut world);
    
    let mut frame_count = 0;
    let dt = 1.0 / 60.0;
    
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Handle input
        handle_input(&window, &mut world);
        
        // Physics step
        world.step(dt);
        
        // Render
        renderer.draw_world(&world);
        
        // Draw UI
        draw_ui(&mut renderer, &world, frame_count);
        
        // Update window
        window
            .update_with_buffer(renderer.get_buffer(), WIDTH, HEIGHT)
            .unwrap();
        
        frame_count += 1;
    }
}

fn setup_boundaries(world: &mut World) {
    // Ground
    let ground = RigidBody::new(
        Vector2D::new(WIDTH as f32 / 2.0, HEIGHT as f32 - 25.0),
        Shape::rectangle(WIDTH as f32, 50.0),
        0.0, // Static
    );
    world.add_body(ground);
    
    // Left wall
    let left_wall = RigidBody::new(
        Vector2D::new(25.0, HEIGHT as f32 / 2.0),
        Shape::rectangle(50.0, HEIGHT as f32),
        0.0,
    );
    world.add_body(left_wall);
    
    // Right wall
    let right_wall = RigidBody::new(
        Vector2D::new(WIDTH as f32 - 25.0, HEIGHT as f32 / 2.0),
        Shape::rectangle(50.0, HEIGHT as f32),
        0.0,
    );
    world.add_body(right_wall);
    
    // Ceiling
    let ceiling = RigidBody::new(
        Vector2D::new(WIDTH as f32 / 2.0, 25.0),
        Shape::rectangle(WIDTH as f32, 50.0),
        0.0,
    );
    world.add_body(ceiling);
}

fn setup_initial_scene(world: &mut World) {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    
    // Add some circles
    for i in 0..3 {
        let mut circle = RigidBody::new(
            Vector2D::new(200.0 + i as f32 * 80.0, 100.0),
            Shape::circle(rng.gen_range(15.0..30.0)),
            1.0,
        );
        circle.restitution = rng.gen_range(0.3..0.9);
        world.add_body(circle);
    }
    
    // Add some rectangles
    for i in 0..2 {
        let mut rect = RigidBody::new(
            Vector2D::new(300.0 + i as f32 * 100.0, 200.0),
            Shape::rectangle(
                rng.gen_range(20.0..50.0),
                rng.gen_range(20.0..50.0)
            ),
            2.0,
        );
        rect.restitution = rng.gen_range(0.2..0.8);
        world.add_body(rect);
    }
}

fn handle_input(window: &Window, world: &mut World) {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    
    // Add new objects with SPACE
    if window.is_key_pressed(Key::Space, minifb::KeyRepeat::No) {
        let mouse_pos = window.get_mouse_pos(minifb::MouseMode::Clamp);
        
        if let Some((mx, my)) = mouse_pos {
            let shape = if rng.gen_bool(0.5) {
                Shape::circle(rng.gen_range(10.0..25.0))
            } else {
                Shape::rectangle(
                    rng.gen_range(15.0..40.0),
                    rng.gen_range(15.0..40.0)
                )
            };
            
            let mut body = RigidBody::new(
                Vector2D::new(mx, my),
                shape,
                rng.gen_range(0.5..2.0),
            );
            body.restitution = rng.gen_range(0.2..0.9);
            body.velocity = Vector2D::new(
                rng.gen_range(-100.0..100.0),
                rng.gen_range(-50.0..50.0)
            );
            world.add_body(body);
        }
    }
    
    // Reset with R
    if window.is_key_pressed(Key::R, minifb::KeyRepeat::No) {
        world.bodies.clear();
        setup_boundaries(world);
        setup_initial_scene(world);
    }
}

fn draw_ui(renderer: &mut Renderer, world: &World, frame: u32) {
    // Count dynamic bodies (excluding static boundaries)
    let dynamic_count = world.bodies.iter()
        .filter(|b| !b.is_static)
        .count();
    
    // Simple FPS counter
    let fps_text = format!("FPS: {}", if frame % 60 == 0 { 60 } else { 0 });
    let objects_text = format!("Objects: {}", dynamic_count);
    
    renderer.draw_text(&fps_text, 10, 10, 0xFFFFFF);
    renderer.draw_text(&objects_text, 10, 25, 0xFFFFFF);
    renderer.draw_text("SPACE: Add object", 10, HEIGHT as i32 - 40, 0xFFFFFF);
    renderer.draw_text("R: Reset", 10, HEIGHT as i32 - 25, 0xFFFFFF);
}