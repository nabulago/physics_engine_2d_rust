use crate::vector2d::Vector2D;
use crate::{rigidbody::RigidBody, shape::Shape};
use crate::world::World;

pub struct Renderer {
    pub width: usize,
    pub height: usize,
    buffer: Vec<u32>,
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![0; width * height],
        }
    }

    pub fn clear(&mut self, color: u32) {
        self.buffer.fill(color);
    }

    pub fn get_buffer(&self) -> &[u32] {
        &self.buffer
    }

    pub fn draw_world(&mut self, world: &World) {
        // Clear with dark blue background
        self.clear(0x001122);

        // Draw all bodies
        for body in &world.bodies {
            self.draw_body(body);
        }
    }

    fn draw_body(&mut self, body: &RigidBody) {
        let color = if body.is_static {
            0x444444 // Gray for static bodies
        }
        else {
            0xFF6B35 // Orange for dynamic bodies
        };

        match &body.shape {
            Shape::Circle { radius } => {
                self.draw_circle(body.position, *radius, color);
            },
            Shape::Rectangle { width, height } => {
            self.draw_rectangle(body.position, *width, *height, color);
            }
        }
    }

    fn draw_circle(&mut self, center: Vector2D, radius: f32, color: u32) {
        let cx = center.x as i32;
        let cy = center.y as i32;
        let r = radius as i32;
        
        // Simple circle drawing using midpoint circle algorithm
        for y in -r..=r {
            for x in -r..=r {
                if x * x + y * y <= r * r {
                    let px = cx + x;
                    let py = cy + y;
                    self.set_pixel(px, py, color);
                }
            }
        }
        
        // Draw a small center dot
        self.set_pixel(cx, cy, 0xFFFFFF);
    }
    
    fn draw_rectangle (&mut self, center: Vector2D, width: f32, height: f32, color: u32) {
        let cx = center.x as i32;
        let cy = center.y as i32;
        let w = (width / 2.0) as i32;
        let h = (height / 2.0) as i32;

        // Draw filled rectangles
        for y in -h..=h {
            for x in -w..=w {
                let px = cx + x;
                let py = cy + y;
                self.set_pixel(px, py, color);
            }
        }

        // Draw center dot
        self.set_pixel(cx, cy, 0xFFFFFF);
    }

    fn set_pixel(&mut self, x: i32, y:i32, color: u32) {
        if x >= 0 && x < self.width as i32 && y>=0 && y < self.height as i32 {
            let index = (y as usize) * self.width + (x as usize);
            self.buffer[index] = color;
        }
    }

    // Helper method to draw text (simple bitmap font)
    pub fn draw_text(&mut self, text: &str, x:i32, y:i32, color:u32) {
        // Simple 5x6 bitmap font for numbers and basic letters
        for(i, ch) in text.chars().enumerate() {
            self.draw_char(ch, x + (i as i32)*6, y, color);
        }
    }

    fn draw_char(&mut self, ch: char, x: i32, y:i32, color: u32) {
        // Simple bitmap patterns for digits
        let pattern = match ch {
            '0' => [
                0b01110,
                0b10001,
                0b10001,
                0b10001,
                0b10001,
                0b10001,
                0b01110,
            ],
            '1' => [
                0b00100,
                0b01100,
                0b00100,
                0b00100,
                0b00100,
                0b00100,
                0b01110,
            ],
            '2' => [
                0b01110,
                0b10001,
                0b00001,
                0b00110,
                0b01000,
                0b10000,
                0b11111,
            ],
            '3' => [
                0b01110,
                0b10001,
                0b00001,
                0b00110,
                0b00001,
                0b10001,
                0b01110,
            ],
            'F' => [
                0b11111,
                0b10000,
                0b10000,
                0b11110,
                0b10000,
                0b10000,
                0b10000,
            ],
            'P' => [
                0b11110,
                0b10001,
                0b10001,
                0b11110,
                0b10000,
                0b10000,
                0b10000,
            ],
            'S' => [
                0b01111,
                0b10000,
                0b10000,
                0b01110,
                0b00001,
                0b00001,
                0b11110,
            ],
            ':' => [
                0b00000,
                0b00100,
                0b00000,
                0b00000,
                0b00000,
                0b00100,
                0b00000,
            ],
            ' ' => [0; 7],
            _ => [
                0b11111,
                0b10001,
                0b10001,
                0b10001,
                0b10001,
                0b10001,
                0b11111,
            ],
        };

        for (row, &bits) in pattern.iter().enumerate() {
            for col in 0..5 {
                if (bits >> (4 - col)) & 1 == 1 {
                    self.set_pixel(x + col, y + row as i32, color);
                }
            }
        }
    }

}