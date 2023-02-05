mod graphics;
use std::io;
use std::sync::mpsc;
use std::thread;

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
    let mut window = graphics::Graphics::new();
    let input = init_terminal_input();

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

        if input_str == "point" {
            window.render_objects.push(Box::new(graphics::RenderObject::new(
                100,
                100,
                sdl2::pixels::Color::RGB(255, 255, 0),
                graphics::Shape::Point,
            )
            ));
            println!("Added a point to the render queue");
            println!("{} objects in the render queue", window.render_objects.len());
            println!("Added with id: {}", window.render_objects[window.render_objects.len() - 1].id);
        }

        if input_str == "hexagon" {
            window.render_objects.push(Box::new(graphics::RenderObject::new(
                100,
                100,
                sdl2::pixels::Color::RGB(0, 255, 0),
                graphics::Shape::Polygon {
                    points: vec![
                        (0, 0),
                        (100, 0),
                        (150, 50),
                        (100, 100),
                        (0, 100),
                        (-50, 50),
                    ],
                },
            )
            ));
            println!("Added a hexagon to the render queue");
            println!("{} objects in the render queue", window.render_objects.len());
            println!("Added with id: {}", window.render_objects[window.render_objects.len() - 1].id);
        }
        
        if input_str == "square" {
            window.render_objects.push(Box::new(graphics::RenderObject::new(
                100,
                100,
                sdl2::pixels::Color::RGB(255, 0, 0),
                graphics::Shape::Polygon {
                    points: vec![
                        (0, 0),
                        (100, 0),
                        (100, 100),
                        (0, 100),
                    ],
                },
            )
            ));
            println!("Added a square to the render queue");
            println!("{} objects in the render queue", window.render_objects.len());
            println!("Added with id: {}", window.render_objects[window.render_objects.len() - 1].id);
        }

        window.update();
    }
}