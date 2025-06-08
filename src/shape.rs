// use crate::vector2d::Vector2D;

#[derive(Debug, Clone)]
pub enum Shape {
    // Circle { radius: f32, center: Vector2D},
    Circle { radius: f32},
    Rectangle { width: f32, height: f32},
}

impl Shape {
    // pub fn circle(radius: f32, center: Vector2D) -> Self {
    //     Shape::Circle { radius, center }
    // }

    pub fn circle(radius: f32) -> Self {
        // Shape::Circle { radius, center }
        Shape::Circle { radius }
    }

    pub fn rectangle( width: f32, height: f32) -> Self {
        Shape::Rectangle { width , height }
    }

    pub fn area(&self) -> f32 {
        match self {
            // Self::Circle { radius, center: _ } => std::f32::consts::PI * radius * radius,
            Self::Circle { radius } => std::f32::consts::PI * radius * radius,
            Self::Rectangle { width, height } => width * height,
        }

    }

    pub fn moment_of_interia(&self, mass: f32) -> f32 {
        match self {
            // Shape::Circle { radius, center: _ } => 0.5 * mass * radius * radius,
            Shape::Circle { radius } => 0.5 * mass * radius * radius,
            Shape::Rectangle { width, height } => { mass * (width * width + height * height) / 12.0
            }
        }
    }
}