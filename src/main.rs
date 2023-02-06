use std::sync::mpsc;
use std::thread;

use physics_sim::Simulation;
use physics_sim::graphics::RenderType;

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
    let mut simulation = physics_sim::Simulation::new((1000000 / 100) as i32,);
    let input = init_terminal_input();

    // vertical line at x, y 0, 0 with empty physics object
    simulation.objects.push(
        physics_sim::SimObject::new(
            100.0,
            100.0,
            0.0, 
            physics_sim::graphics::RenderObject::new(
                sdl2::pixels::Color::RGB(255, 0, 0),
                physics_sim::graphics::RenderType::Line {
                    magnitude: 15.0,
                }
            ),
            None,
            Some(|obj: &mut physics_sim::SimObject| {
                obj.rotation += 0.1;
                obj.x += 0.1;
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

    simulation.objects.push(
        physics_sim::SimObject::new(
            100.0,
            100.0,
            90.0, 
            physics_sim::graphics::RenderObject::new(
                sdl2::pixels::Color::RGB(0, 255, 0),
                physics_sim::graphics::RenderType::Line {
                    magnitude: 15.0,
                }
            ),
            None,
            None
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