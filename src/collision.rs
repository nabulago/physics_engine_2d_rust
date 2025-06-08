use crate::vector2d::Vector2D;
use crate::rigidbody::RigidBody;
use crate::shape::Shape;

#[derive(Debug, Clone)]
pub struct Contact {
    pub point: Vector2D,
    pub normal: Vector2D,
    pub penetration: f32,
    pub body_a_index: usize,
    pub body_b_index: usize,
}

pub fn detect_collision(body_a: &RigidBody, body_b: &RigidBody,
                        index_a: usize, index_b: usize) -> Option<Contact> {
    match (&body_a.shape, &body_b.shape) {
        // (Shape::Circle { radius: r1, center: _ }, Shape::Circle { radius: r2, center: _ }) => {
        //     circle_circle_collision(body_a, body_b, *r1, *r2, index_a, index_b)
        // }
        (Shape::Circle { radius: r1 }, Shape::Circle { radius: r2, }) => {
            circle_circle_collision(body_a, body_b, *r1, *r2, index_a, index_b)
        }
        ,
        (Shape::Rectangle { .. }, Shape::Rectangle { .. }) => {
            // Simplified AABB collision for rectangles
            aabb_collition(body_a, body_b, index_a, index_b)
        },
        _ => { 
            // Circle-Rectangle collision (simplified)
            None // TODO: Implement Circle-Rectangle collision
        }
    }
}

fn circle_circle_collision(body_a: &RigidBody, body_b: &RigidBody,
                            r1: f32, r2: f32, index_a: usize, index_b: usize) -> Option<Contact> {
    
    let distance_vec = body_b.position - body_a.position;
    let distance = distance_vec.magnitude();
    let radii_sum = r1 + r2;

    if distance < radii_sum && distance > 0.0 {
        let normal = distance_vec.normalize();
        let penetration = radii_sum - distance;
        let contact_point = body_a.position + normal * r1;

        Some(Contact {
            point: contact_point,
            normal,
            penetration,
            body_a_index: index_a,
            body_b_index: index_b,
        })
    } else {
        None
    }
}

fn aabb_collition(body_a: &RigidBody, body_b: &RigidBody,
                index_a: usize, index_b: usize) -> Option<Contact> {
    let (w1, h1) = match &body_a.shape {
        Shape::Rectangle { width, height } => (*width, *height),
        _ => return None,
    };

    let (w2, h2) = match &body_b.shape {
        Shape::Rectangle { width, height } => (*width, *height),
        _ => return None,
    };

    let dx = body_b.position.x - body_a.position.x;
    let dy = body_b.position.y - body_a.position.y;

    let overlap_x = ( w1 + w2 ) / 2.0 - dx.abs();
    let overlap_y = ( h1 + h2 ) / 2.0 - dy.abs();

    if overlap_x > 0.0 && overlap_y > 0.0 {
        let (normal, penetration, contact_point) = if overlap_x < overlap_y {
            let normal = Vector2D::new(if dx > 0.0 {-1.0} else { 1.0 }, 0.0);
            let contact_x = if dx < 0.0 {
                body_a.position.x + w1 / 2.0
            } else {
                body_a.position.x - w1 / 2.0
            };
            (normal, overlap_x, Vector2D::new(contact_x, body_a.position.y))
        } else {
           let normal = Vector2D::new(0.0, if dy > 0.0 {-1.0} else { 1.0 });
           let contact_y = if dy > 0.0 {
                body_a.position.y + h1 / 2.0
            } else {
                body_a.position.y - h1 /2.0
            };
            (normal, overlap_y, Vector2D::new(body_a.position.x, contact_y))
        };
        Some(Contact {
            point: contact_point,
            normal,
            penetration,
            body_a_index: index_a,
            body_b_index: index_b,
        })
    } else {
        None
    }
}