extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use std::f64::consts::PI;
use std::thread;
use std::time::Duration;

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 480;
const VOXEL_SIZE: u32 = 16;
const SPHERE_RADIUS: i32 = 16;
const LIGHT_ANGLE: f64 = PI / 4.0;

struct Camera {
    x: f64,
    y: f64,
    z: f64,
    angle: f64,
}

impl Camera {
    fn new() -> Camera {
        Camera {
            x: 8.0,
            y: 8.0,
            z: SPHERE_RADIUS as f64,
            angle: 0.0,
        }
    }

    fn rotate(&mut self, angle: f64) {
        self.angle += angle;
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Voxel Sphere", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();
    let mut renderer = window.into_canvas().build().unwrap();

    let mut camera = Camera::new();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();

        // render voxels
        for x in -SPHERE_RADIUS..=SPHERE_RADIUS {
            for y in -SPHERE_RADIUS..=SPHERE_RADIUS {
                for z in -SPHERE_RADIUS..=SPHERE_RADIUS {
                    let distance = ((x * x) + (y * y) + (z * z)) as f64;
                    if distance <= (SPHERE_RADIUS as f64) * (SPHERE_RADIUS as f64) {
                        let projection = project(x as f64, y as f64, z as f64, &camera);
                        let (screen_x, screen_y) = (
                            projection.x as i32 + (WINDOW_WIDTH as i32 / 2),
                            projection.y as i32 + (WINDOW_HEIGHT as i32 / 2),
                        );
                        let voxel_rect = Rect::new(screen_x, screen_y, VOXEL_SIZE, VOXEL_SIZE);
                        let voxel_color = calculate_lighting(x as f64, y as f64, z as f64, &camera);
                        renderer.set_draw_color(voxel_color);
                        renderer.fill_rect(voxel_rect).unwrap();
                    }
                }
            }
        }
        renderer.present();

        camera.rotate(0.01);

        thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn project(x: f64, y: f64, z: f64, camera: &Camera) -> Point {
    let mut x = x - camera.x;
    let mut y = y - camera.y;
    let sin_angle = camera.angle.sin();
    let cos_angle = camera.angle.cos();

    let new_x = x * cos_angle - y * sin_angle;
    y = x * sin_angle + y * cos_angle;
    x = new_x;

    Point {
        x: (x / z) * WINDOW_WIDTH as f64 / 2.0,
        y: (y / z) * WINDOW_HEIGHT as f64 / 2.0,
    }
}

fn calculate_lighting(x: f64, y: f64, z: f64, camera: &Camera) -> Color {
    let light_direction = LIGHT_ANGLE.sin() * x + LIGHT_ANGLE.cos() * y;
    let brightness = (light_direction + 1.0) / 2.0;
    Color::RGB(
        (brightness * 255.0) as u8,
        (brightness * 255.0) as u8,
        (brightness * 255.0) as u8,
    )
}

struct Point {
    x: f64,
    y: f64,
}
