use std::sync::mpsc;
use std::thread;

use physics_sim::{graphics::{RenderType, RenderObject}, physics::{self, PhysicsObject, ColliderType}, Vector};

fn init_terminal_input() -> mpsc::Receiver<String> {
    // Channel to send input from the input thread to the main loop
    let (tx, rx) = mpsc::channel();

    // Start the input thread
    thread::spawn(move || {
        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            tx.send(input.trim().to_string()).unwrap();
        }
    });

    return rx;
}

fn main() {
    let time_step = 120;
    let mut simulation = physics_sim::Simulation::new(
        (1000000 / time_step) as i32,
        physics::Physics::new(
            Vector::new(0.0, -9.807)
        )
    );
    let input = init_terminal_input();

    // vertical line at x, y 0, 0 
    simulation.objects.push(
        physics_sim::SimObject::new(
            10.0,
            50.0,
            0.0, 
            Some(RenderObject::new(
                sdl2::pixels::Color::RGB(255, 0, 0),
                /*RenderType::Polygon { 
                    vertices: vec![
                        (0, 0),
                        (0, 100),
                        (100, 150),
                        (100, 50),
                        (0,0)
                    ],
                }*/
                RenderType::Line { magnitude: (12.0) }
            )),
            Some(PhysicsObject::new(
                ColliderType::Circle { radius: 15.0 },
                1.0,
                Vector::new(0.0, 0.0),
                Vector::new(0.0, 0.0))
            ),
            None,
            Some(|obj: &mut physics_sim::SimObject| {
                obj.rotation += 0.1;
                let render_obj_mut = obj.get_render_object_mut().unwrap();
                match render_obj_mut.render_type {
                    RenderType::Line { magnitude } => {
                        render_obj_mut.render_type = RenderType::Line { magnitude: magnitude + 0.05 };
                    },
                    _ => {},
                }
            })
        )
    );
    
    loop {
        // remove the newline character at the end of the input
        let mut input_str = String::new();

        match input.try_recv() {
            Ok(str) => {
                input_str = str;
            },
            Err(mpsc::TryRecvError::Empty) => {
                // do nothing
            },
            Err(mpsc::TryRecvError::Disconnected) => {
                println!("Input thread disconnected")
            },
        }

        simulation.update();
    }
}