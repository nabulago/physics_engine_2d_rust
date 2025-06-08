use crate::vector2d::Vector2D;
use crate::shape::Shape;

#[derive(Debug, Clone)]
pub struct RigidBody {
    /**
    Rigid body state specific details
    position:
    */
    pub position: Vector2D, 
    pub velocity: Vector2D,
    pub acceleration: Vector2D,
    pub angle: f32,                 // Rotation in radius
    pub angular_velocity: f32,
    pub angular_acceleration: f32,

    pub mass: f32,
    pub inv_mass: f32,              // 1/mass for efficiency
    pub moment_of_inertia: f32,
    pub inv_moment_of_inertia: f32,

    pub shape: Shape,
    pub restitution: f32,           // Bounciness (0-1)
    pub friction: f32,              // friction coefficient
    pub is_static: bool,            // immovable object

    pub force: Vector2D,            // Accumulated Forces
    pub torque: f32,                // Accumulated Torque
}

impl RigidBody {
    pub fn new(position: Vector2D, shape: Shape, mass: f32) -> Self {
        let inv_mass = if mass == 0.0 {0.0} else { 1.0 / mass };
        let moment_of_inertia = shape.moment_of_interia(mass);
        let inv_moment_of_inertia: f32 = if moment_of_inertia == 0.0 { 
            0.0 
        } else {
            1.0 / moment_of_inertia
        };

        Self { 
            position, 
            velocity: Vector2D::zero(),
            acceleration: Vector2D::zero(),
            angle: 0.0, 
            angular_velocity: 0.0,
            angular_acceleration: 0.0,
            mass,
            inv_mass,
            moment_of_inertia,
            inv_moment_of_inertia,
            shape,
            restitution:0.8,
            friction: 0.3,
            is_static: mass == 0.0,
            force: Vector2D::zero(),
            torque: 0.0,
        }
    }
    
    pub fn apply_force(&mut self, force: Vector2D) {
        if !self.is_static {
            self.force += force;
        }
    }

    pub fn apply_force_at_point(&mut self, force: Vector2D, point: Vector2D)
    {
        if !self.is_static {
            self.force += force;
            let r = point - self.position;
            self.torque += r.cross(&force);
        }
    }

    pub fn apply_impulse(&mut self, impulse: Vector2D) {
        if !self.is_static {
            self.velocity += impulse * self.inv_mass;
        }
    }

    pub fn apply_angular_impulse(&mut self, impulse: f32) {
        if !self.is_static {
            self.angular_velocity += impulse * self.inv_moment_of_inertia;
        }
    }
}