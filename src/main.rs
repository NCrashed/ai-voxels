use sdl2::{
    event::Event,
    keyboard::Keycode,
    mouse::MouseButton,
    render::{Canvas, TextureCreator},
    video::{Window, WindowContext},
};

use cgmath::{EuclideanSpace, SquareMatrix};
use rand::prelude::*;

use cgmath::{Deg, Matrix4, PerspectiveFov, Point3, Rad, Vector3};

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 480;

const FOV: f32 = 60.0;
const NEAR_PLANE: f32 = 0.1;
const FAR_PLANE: f32 = 100.0;

struct Camera {
    position: Point3<f32>,
    rotation: Vector3<f32>,
}

impl Camera {
    fn projection_matrix(&self) -> Matrix4<f32> {
        let aspect_ratio = WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32;
        PerspectiveFov {
            fovy: Deg(FOV).into(),
            aspect: aspect_ratio,
            near: NEAR_PLANE,
            far: FAR_PLANE,
        }
        .to_perspective()
        .into()
    }

    fn view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at(
            self.position,
            Point3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
        ) * Matrix4::from_angle_x(Rad(self.rotation.x))
            * Matrix4::from_angle_y(Rad(self.rotation.y))
            * Matrix4::from_angle_z(Rad(self.rotation.z))
    }
}

struct Voxel {
    position: Point3<f32>,
    color: (u8, u8, u8),
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Voxel Grid", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())?;

    let texture_creator: TextureCreator<_> = canvas.texture_creator();

    let mut voxels: Vec<Voxel> = Vec::new();

    let radius = 5;
    for x in -radius..=radius {
        for y in -radius..=radius {
            for z in -radius..=radius {
                let position = Point3::new(x as f32, y as f32, z as f32);
                let distance =
                    (position.x.powi(2) + position.y.powi(2) + position.z.powi(2)).sqrt();
                if distance < radius as f32 {
                    let color = (
                        (position.x * 255.0 / radius as f32) as u8,
                        (position.y * 255.0 / radius as f32) as u8,
                        (position.z * 255.0 / radius as f32) as u8,
                    );
                    voxels.push(Voxel { position, color });
                }
            }
        }
    }

    let mut camera = Camera {
        position: Point3::new(0.0, 0.0, 10.0),
        rotation: Vector3::new(0.0, 0.0, 0.0),
    };

    let mut event_pump = sdl_context.event_pump()?;

    let mut running = true;
    while running {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    running = false;
                }
                Event::MouseMotion {
                    xrel,
                    yrel,
                    mousestate,
                    ..
                } => {
                    if mousestate.left() {
                        camera.rotation.x += yrel as f32 / 100.0;
                        camera.rotation.y -= xrel as f32 / 100.0;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    camera.position.z -= 0.1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    camera.position.z += 0.1;
                }
                _ => {}
            }
        }

        canvas.set_draw_color((0, 0, 0, 255));
        canvas.clear();

        let projection = camera.projection_matrix();
        let view = camera.view_matrix();

        for voxel in &voxels {
            let clip_vec = (projection * view) * voxel.position.to_homogeneous();

            let (x, y) = (
                ((clip_vec.x / clip_vec.w + 1.0) * WINDOW_WIDTH as f32 / 2.0) as i32,
                ((clip_vec.y / clip_vec.w + 1.0) * WINDOW_HEIGHT as f32 / 2.0) as i32,
            );

            let voxel_size = 16;
            let mut texture = texture_creator
                .create_texture_static(sdl2::pixels::PixelFormatEnum::RGB24, voxel_size, voxel_size)
                .map_err(|e| e.to_string())?;

            let pixel_data: Vec<u8> = [voxel.color.0, voxel.color.1, voxel.color.2]
                .into_iter()
                .cycle()
                .take((voxel_size * voxel_size) as usize)
                .collect();
            texture
                .update(None, &pixel_data, 3)
                .map_err(|e| e.to_string())?;

            let target = sdl2::rect::Rect::new(x, y, voxel_size, voxel_size);
            canvas.copy(&texture, None, Some(target))?;
        }
        canvas.present();
    }

    Ok(())
}
