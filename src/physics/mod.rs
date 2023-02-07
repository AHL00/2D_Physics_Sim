use crate::*;

pub struct Physics {
    gravity: Vector,

}

impl Physics {
    pub fn new(gravity: Vector) -> Physics {
        Physics {
            gravity: gravity,
        }
    }

    pub fn update(&mut self, objects: &mut Vec<SimObject>, fixed_delta_time: time::Duration) {
        for object in objects {
            // get needed data from object
            let mut old_pos = Vector::new(0.0, 0.0);
            old_pos.x = object.x;
            old_pos.y = object.y;

            //update physics object
            let mut new_pos = Vector::new(0.0, 0.0);
            match object.physics_object {
                Some(ref mut phys_obj) => {
                    if phys_obj.mass <= 0.0 { panic!("Mass must be greater than 0"); }

                    // Process forces
                    phys_obj.forces[0] = self.calc_gravity_force(phys_obj.mass);
                    
                    // Apply forces
                    for i in 0..phys_obj.forces.len() {
                        phys_obj.acceleration = self.calc_accel(phys_obj.forces[i], phys_obj.mass);
                    }

                    // Update positions and velocities
                    phys_obj.velocity = self.calc_velo(phys_obj.acceleration, phys_obj.velocity, fixed_delta_time);

                    let displacement = self.calc_displacement(phys_obj.velocity, fixed_delta_time);
                    //println!("Displacement: {:?} {:?}", displacement.x, displacement.y);
                    //println!("Gravity force: {:?} {:?}", phys_obj.forces[0].x, phys_obj.forces[0].y);
                    new_pos = old_pos + displacement;
                },
                None => {},
            }

            // update object here
            object.x = new_pos.x;
            object.y = new_pos.y;
        }
    }

    fn calc_gravity_force(&mut self, mass: f64) -> Vector {
        // F = ma, so F = mg
        self.gravity * mass
    }

    fn calc_accel(&mut self, force: Vector, mass: f64) -> Vector {
        // F = ma, so F / m = a
        force / mass
    }

    fn calc_velo(&mut self, accel: Vector, old_velo: Vector, delta_time: time::Duration) -> Vector {
        // v = u + a * t
        old_velo + (accel * (delta_time.as_secs_f64()))
    }

    fn calc_displacement(&mut self, velo: Vector, delta_time: time::Duration) -> Vector {
        // s = vt, so s = v * t
        velo * (delta_time.as_secs_f64())
    }
}

pub struct PhysicsObject {
    // forces[0] is always gravity
    collider: ColliderType,
    mass: f64,
    velocity: Vector,
    acceleration: Vector,
    forces: Vec<Vector>,
}

impl PhysicsObject {
    pub fn new(collider: ColliderType, mass: f64, velocity: Vector, acceleration: Vector) -> PhysicsObject {
        let mut forces: Vec<Vector> = Vec::new();
        forces.push(Vector::new(0.0, 0.0));
        PhysicsObject {
            collider: collider,
            mass: mass,
            velocity: velocity,
            acceleration: acceleration,
            forces: forces,
        }
    }
}

pub enum ColliderType {
    Circle {
        radius: f64,
    },
    Rectangle {
        width: f64,
        height: f64,
    },
    Line {
        length: f64,
    },
}

