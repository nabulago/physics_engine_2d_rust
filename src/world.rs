use crate::vector2d::Vector2D;
use crate::rigidbody::RigidBody;
use crate::collision::{Contact, detect_collision};

pub struct World {
    pub bodies: Vec<RigidBody>,
    pub gravity: Vector2D,
    pub damping: f32,
    pub angular_damping: f32,
}

impl World {
    pub fn new() -> Self {
        Self {
            bodies: Vec::new(),
            gravity: Vector2D::new(0.0, 9.81 * 50.0), // Scaled for screen coordinates
            damping: 0.99,
            angular_damping: 0.99,
        }
    }
    
    pub fn add_body(&mut self, body: RigidBody) -> usize {
        self.bodies.push(body);
        self.bodies.len() - 1
    }
    
    pub fn step(&mut self, dt: f32) {
        // Clear forces
        for body in &mut self.bodies {
            body.force = Vector2D::zero();
            body.torque = 0.0;
        }
        
        // Apply gravity
        for body in &mut self.bodies {
            if !body.is_static {
                body.apply_force(self.gravity * body.mass);
            }
        }
        
        // Detect collisions
        let mut contacts = Vec::new();
        for i in 0..self.bodies.len() {
            for j in (i + 1)..self.bodies.len() {
                if let Some(contact) = detect_collision(&self.bodies[i], &self.bodies[j], i, j) {
                    contacts.push(contact);
                }
            }
        }
        
        // Resolve collisions
        for contact in &contacts {
            self.resolve_collision(contact);
        }
        
        // Integrate physics
        for body in &mut self.bodies {
            if !body.is_static {
                // Linear integration (Euler method)
                body.acceleration = body.force * body.inv_mass;
                body.velocity += body.acceleration * dt;
                body.velocity *= self.damping;
                body.position += body.velocity * dt;
                
                // Angular integration
                body.angular_acceleration = body.torque * body.inv_moment_of_inertia;
                body.angular_velocity += body.angular_acceleration * dt;
                body.angular_velocity *= self.angular_damping;
                body.angle += body.angular_velocity * dt;
            }
        }
    }
    
    fn resolve_collision(&mut self, contact: &Contact) {
        let body_a = &self.bodies[contact.body_a_index];
        let body_b = &self.bodies[contact.body_b_index];
        
        // Calculate relative velocity
        let relative_velocity = body_b.velocity - body_a.velocity;
        let velocity_along_normal = relative_velocity.dot(&contact.normal);
        
        // Don't resolve if velocities are separating
        if velocity_along_normal > 0.0 {
            return;
        }
        
        // Calculate restitution
        let e = (body_a.restitution + body_b.restitution) / 2.0;
        
        // Calculate impulse scalar
        let j = -(1.0 + e) * velocity_along_normal;
        let j = j / (body_a.inv_mass + body_b.inv_mass);
        
        // Apply impulse
        let impulse = contact.normal * j;
        
        // We need to borrow mutably, so we'll do this carefully
        let (inv_mass_a, inv_mass_b) = (body_a.inv_mass, body_b.inv_mass);
        
        if contact.body_a_index < contact.body_b_index {
            let (left, right) = self.bodies.split_at_mut(contact.body_b_index);
            left[contact.body_a_index].apply_impulse(impulse * -1.0);
            right[0].apply_impulse(impulse);
        } else {
            let (left, right) = self.bodies.split_at_mut(contact.body_a_index);
            left[contact.body_b_index].apply_impulse(impulse);
            right[0].apply_impulse(impulse * -1.0);
        }
        
        // Position correction to prevent sinking
        let correction = contact.normal * (contact.penetration * 0.8);
        let total_inv_mass = inv_mass_a + inv_mass_b;
        
        if total_inv_mass > 0.0 {
            let correction_a = correction * (inv_mass_a / total_inv_mass);
            let correction_b = correction * (inv_mass_b / total_inv_mass);
            
            if contact.body_a_index < contact.body_b_index {
                let (left, right) = self.bodies.split_at_mut(contact.body_b_index);
                left[contact.body_a_index].position -= correction_a;
                right[0].position += correction_b;
            } else {
                let (left, right) = self.bodies.split_at_mut(contact.body_a_index);
                left[contact.body_b_index].position += correction_b;
                right[0].position -= correction_a;
            }
        }
    }
}