pub mod physics;
pub mod graphics;

use std::{time};

use physics::{PhysicsObject, Physics};
use uuid::Uuid;

pub struct Simulation {
    pub objects: Vec<SimObject>,
    pub time_step: i32,
    pub graphics: graphics::Graphics,
    pub physics: physics::Physics,
    last_update_time: time::Instant,
    last_fixed_update_time: time::Instant,
    delta_time: time::Duration,
    fixed_delta_time: time::Duration,
}

impl Simulation {
    pub fn new(time_step: i32, physics: Physics) -> Simulation {
        if time_step <= 0 { panic!("Time step must be greater than 0"); }

        Simulation {
            objects: Vec::new(),
            graphics: graphics::Graphics::new(),
            physics: physics,
            time_step: time_step,
            last_update_time: time::Instant::now(),
            last_fixed_update_time: time::Instant::now(),
            delta_time: time::Duration::from_secs(0),
            fixed_delta_time: time::Duration::from_secs(0),
        }
    }

    pub fn update(&mut self) {
        self.graphics.update(&mut self.objects);

        // Update all objects if they have update
        for object in &mut self.objects {
            if let Some(update) = object.update {
                update(object);
            }
        }

        if time::Instant::now().duration_since(self.last_fixed_update_time).as_micros() >= self.time_step as u128 {
            self.fixed_update();
            self.last_fixed_update_time = time::Instant::now();
        }
        
        self.delta_time = time::Instant::now().duration_since(self.last_update_time);

        self.last_update_time = time::Instant::now();
    }

    pub fn fixed_update(&mut self) {
        //self.physics.update(&mut self.objects);
        // Update all objects if they have fixed_update
        for object in &mut self.objects {
            if let Some(fixed_update) = object.fixed_update {
                fixed_update(object); 
            }
        }

        self.physics.update(&mut self.objects, self.fixed_delta_time);

        self.fixed_delta_time = time::Instant::now().duration_since(self.last_fixed_update_time);
        self.last_fixed_update_time = time::Instant::now();
    }
}

pub struct SimObject {
    pub id: u128,
    pub x: f64,
    pub y: f64,
    pub rotation: f64,
    pub render_object: Option<graphics::RenderObject>,
    pub physics_object: Option<physics::PhysicsObject>,
    pub update: Option<fn(&mut SimObject)>,
    pub fixed_update: Option<fn(&mut SimObject)>,
}

/// Set update to none if no update function is needed
impl SimObject {
    pub fn new(x: f64, y: f64, rotation: f64, render_object: Option<graphics::RenderObject>, physics_object: Option<physics::PhysicsObject>, update: Option<fn(&mut SimObject)>, fixed_update: Option<fn(&mut SimObject)>) -> SimObject {
        SimObject {
            id: Uuid::new_v4().as_u128(),
            x: x,
            y: y,
            rotation: rotation,
            render_object: render_object,
            physics_object: physics_object,
            update: update,
            fixed_update: fixed_update,
        }
    }

    pub fn get_render_object_mut(&mut self) -> Result<&mut graphics::RenderObject, &str> {
        // check if render_object is None
        if let Some(render_object) = &mut self.render_object {
            Ok(render_object)
        } else {
            Err("Render object is None. Object ID: {self.id}")
        }
    }

    pub fn get_render_object(&self) -> Result<&graphics::RenderObject, &str> {
        // check if render_object is None
        if let Some(render_object) = &self.render_object {
            Ok(render_object)
        } else {
            Err("Render object is None. Object ID: {self.id}")
        }
    }
}

#[derive(Clone, Copy)]
pub struct Vector {
    x: f64,
    y: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64) -> Vector {
        Vector {
            x: x,
            y: y,
        }
    }

    pub fn get_mag(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn get_angle(&self) -> f64 {
        self.y.atan2(self.x)
    }

    pub fn get_unit(&self) -> Vector {
        let mag = self.get_mag();
        Vector {
            x: self.x / mag,
            y: self.y / mag,
        }
    }

    pub fn get_normalized(&mut self) -> Vector {
        let mag = self.get_mag();
        Vector {
            x: self.x / mag,
            y: self.y / mag,
        }
    }
}

impl std::ops::Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, scalar: f64) -> Vector {
        Vector {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl std::ops::Div<f64> for Vector {
    type Output = Vector;

    fn div(self, scalar: f64) -> Vector {
        Vector {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

impl std::ops::AddAssign for Vector {
    fn add_assign(&mut self, other: Vector) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl std::ops::SubAssign for Vector {
    fn sub_assign(&mut self, other: Vector) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl std::ops::MulAssign<f64> for Vector {
    fn mul_assign(&mut self, scalar: f64) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

impl std::ops::DivAssign<f64> for Vector {
    fn div_assign(&mut self, scalar: f64) {
        self.x /= scalar;
        self.y /= scalar;
    }
}