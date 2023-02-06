use sdl2::{self, render::Canvas};
use std::time;

use crate::SimObject;

pub struct Graphics {
    pub sdl_context: sdl2::Sdl,
    pub video_subsystem: sdl2::VideoSubsystem,
    pub canvas: Canvas<sdl2::video::Window>,
    pub camera: Camera,
    last_update: time::Instant,
    current_second_total_frame_time: time::Duration,
    current_second_update_count: i32,
}

impl Graphics {
    pub fn new() -> Graphics {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("Rust Game", 800, 600)
            .position_centered()
            .allow_highdpi()
            .resizable()
            .opengl()
            .build()
            .unwrap();
        
        let mut canvas = window.into_canvas().build().unwrap();

        let camera = Camera::new(0, 0, 1.0);

        Graphics {
            sdl_context: sdl_context,
            video_subsystem: video_subsystem,
            canvas: canvas,
            last_update: time::Instant::now(),
            camera: camera,
            current_second_total_frame_time: time::Duration::from_secs(0),
            current_second_update_count: 0,
        }
    }

    pub fn update(&mut self, sim_objects: &mut Vec<SimObject>) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();

        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => {
                    println!("Quit");
                },
                _ => {}
            }
        }

        self.canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        self.canvas.clear();

        // render the shapes
        for sim_object in sim_objects.iter_mut() {
            match sim_object.get_render_object() {
                Ok(render_object) => {
                    //println!("Rendering object: {:?}", sim_object.id);
                    render_object.render_type.render(
                        &mut self.canvas,
                        render_object.color, 
                        sim_object.rotation, 
                        false, 
                        (sim_object.x, sim_object.y),
                    );
                }
                Err(error) => {
                    // Handle the error here
                    println!("Error: {}", error);
                }
            }
        }

        self.canvas.present();

        // add the frame time until it reaches 1 second, then print the fps
        if self.current_second_total_frame_time.as_secs() >= 1 {
            let avg_frametime = self.current_second_total_frame_time.as_micros() / self.current_second_update_count as u128;
            println!("{} fps",  1000000 / avg_frametime);
            println!("{} ms per frame", avg_frametime as f64 / 1000.0);
            println!("------------------------------------");
            self.current_second_total_frame_time = time::Duration::from_secs(0);
            self.current_second_update_count = 0;
        }

        let frame_time = time::Instant::now().duration_since(self.last_update);
        self.current_second_total_frame_time += frame_time;
        self.current_second_update_count += 1;
        self.last_update = time::Instant::now();
    }
}

pub struct Camera {
    pub x: i32,
    pub y: i32,
    pub zoom: f32,
}

impl Camera {
    pub fn new(x: i32, y: i32, zoom: f32) -> Camera {
        Camera {
            x: x,
            y: y,
            zoom: zoom,
        }
    }
}

pub struct RenderObject {
    pub color: sdl2::pixels::Color,
    pub render_type: RenderType,
}

impl RenderObject {
    pub fn new(color: sdl2::pixels::Color, render_type: RenderType) -> RenderObject {
        RenderObject {
            color: color,
            render_type: render_type,
        }
    }
}

pub enum RenderType {
    Line {
        magnitude: f64,
    },
    Circle {
        // Set segments to 0 for automatic segmentation
        radius: i32,
        segments: i32,
    },
    Polygon {
        // A polygon must have (0, 0) as the first point
        points: Vec<(i32, i32)>,
    },
    Point,
    Rectangle {
        width: i32,
        height: i32,
        filled: bool,
    },
    Texture {
        texture: sdl2::render::Texture<'static>,
    },
    None {},
}

impl RenderType {
    fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, color: sdl2::pixels::Color, rotation: f64, filled: bool, coords: (f64, f64)) {
        // Calculate camera coordinates from world coords
        // Invert the y axis to match the physics

        // camera offset
        let camera_coords: (i32, i32);
        match self {
            RenderType::Line { magnitude } => {
                let mut start_point = (coords.0 as i32, coords.1 as i32);
                let end_x = (coords.0 + (rotation.to_radians().sin() * *magnitude as f64)) as i32;
                let end_y = (coords.1 + (rotation.to_radians().cos() * *magnitude as f64)) as i32;
                let mut end_point = (end_x, end_y);

                start_point.1 = invert_canvas_y(start_point.1, canvas);
                end_point.1 = invert_canvas_y(end_point.1, canvas);

                canvas.set_draw_color(color);
                canvas.draw_line(start_point, end_point).unwrap();
            },
            RenderType::Circle { radius, segments } => {

            },
            RenderType::Polygon { points } => {

            },
            RenderType::Point => {

            },
            RenderType::Rectangle { width, height, filled } => {

            },
            RenderType::Texture { texture } => {
                // render texture

            },
            RenderType::None {} => {
    
            },
        }
    }
}

fn invert_canvas_y(y: i32, canvas: &Canvas<sdl2::video::Window>) -> i32 {
    canvas.output_size().unwrap().1 as i32 - y
}