pub mod physics;
pub mod graphics;

use std::time;

use uuid::Uuid;

pub struct Simulation {
    pub objects: Vec<SimObject>,
    pub time_step: i32,
    pub graphics: graphics::Graphics,
    //pub physics: physics::Physics,
    last_update_time: time::Instant,
    last_fixed_update_time: time::Instant,
    delta_time: time::Duration,
    fixed_delta_time: time::Duration,
}

impl Simulation {
    pub fn new(time_step: i32) -> Simulation {
        if time_step <= 0 { panic!("Time step must be greater than 0"); }

        Simulation {
            objects: Vec::new(),
            graphics: graphics::Graphics::new(),
            time_step: time_step,
            //physics: physics::Physics::new(),
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
        //obj.update.map(|update| update(&mut obj));

        if time::Instant::now().duration_since(self.last_fixed_update_time).as_micros() >= self.time_step as u128 {
            self.fixed_update();
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

        //obj.fixed_update.map(|fixed_update| fixed_update(&mut obj));

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
    pub update: Option<fn(&mut SimObject)>,
    pub fixed_update: Option<fn(&mut SimObject)>,
}

/// Set update to none if no update function is needed
impl SimObject {
    pub fn new(x: f64, y: f64, rotation: f64, render_object: graphics::RenderObject, update: Option<fn(&mut SimObject)>, fixed_update: Option<fn(&mut SimObject)>) -> SimObject {
        SimObject {
            id: Uuid::new_v4().as_u128(),
            x: x,
            y: y,
            rotation: rotation,
            render_object: Some(render_object),
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