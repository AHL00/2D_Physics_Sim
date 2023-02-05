use sdl2::{self, sys::SDL_Point, render::Canvas};
use uuid::Uuid;

pub struct Graphics {
    sdl_context: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    canvas: Canvas<sdl2::video::Window>,
    pub render_objects: Vec<Box<RenderObject>>,
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

        Graphics {
            sdl_context: sdl_context,
            video_subsystem: video_subsystem,
            canvas: canvas,
            render_objects: Vec::new(),
        }
    }

    pub fn update(&mut self) {
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
        for render_object in self.render_objects.iter() {
            render_object.shape.render(&mut self.canvas, render_object.color, false, (render_object.x, render_object.y));
        }

        self.canvas.present();

    }
}

pub struct RenderObject {
    pub id: u128,
    pub x: i32,
    pub y: i32,
    pub color: sdl2::pixels::Color,
    pub shape: Shape,
}

impl RenderObject {
    pub fn new(x: i32, y: i32, color: sdl2::pixels::Color, shape: Shape) -> RenderObject {
        let id = Uuid::new_v4().as_u128();
        RenderObject {
            id: id,
            x: x,
            y: y,
            color: color,
            shape: shape,
        }
    }
}

pub enum Shape {
    Line {
        vector: (i32, i32),
    },
    Circle {
        radius: i32,
    },
    Polygon {
        points: Vec<(i32, i32)>,
    },
    Point
}

impl Shape {
    fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, color: sdl2::pixels::Color, filled: bool, coords: (i32, i32)) {
        match self {
            Shape::Line { vector} => {
                // calculate start point and end point
                canvas.set_draw_color(color);
                let start = (coords.0, coords.1);
                let end = (coords.0 + vector.0, coords.1 + vector.1);
                canvas.draw_line(start, end).unwrap();
            },
            Shape::Circle {radius } => {
                canvas.set_draw_color(color);
                
            },
            Shape::Polygon { points } => {
                canvas.set_draw_color(color);
                let mut sdl_points: Vec<SDL_Point> = Vec::new();

                for point in points.iter() {
                    // turn points into SDL_Point while offsetting by render coordinates
                    sdl_points.push(SDL_Point { x: point.0 + coords.0, y: point.1 + coords.1});
                }
                
                for i in 0..sdl_points.len() {
                    // draw line with current point and next point
                    let current_point = sdl_points[i];
                    let next_point = sdl_points[(i + 1) % sdl_points.len()];
                    canvas.draw_line((current_point.x, current_point.y), (next_point.x, next_point.y)).unwrap();
                }
                
                // draw line with last point and first point
                let last_point = sdl_points[sdl_points.len() - 1];
                let first_point = sdl_points[0];
                canvas.draw_line((last_point.x, last_point.y), (first_point.x, first_point.y)).unwrap();
            },
            Shape::Point { } => {
                canvas.set_draw_color(color);
                canvas.draw_point((coords.0, coords.1)).unwrap();
            }
        }
    }
}