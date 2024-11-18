use std::{
    env,
    error::Error,
    f32::consts::PI,
    ops::Add,
    sync::Arc,
    thread::{self, JoinHandle},
    time::Duration,
};

use clap::Parser;
use crossterm::event::{self, Event, KeyCode, KeyEvent, ModifierKeyCode};
use glam::Vec3;
use terminal_renderer::math::Rotation;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    /// Path to the .obj file
    #[arg(short, long)]
    path: String,

    /// Option to list the number of triangles instead of rendering
    #[arg(short)]
    count_tris: bool,

    /// Characters to use for different light levels [low..high]
    #[arg(long, num_args=0..)]
    chars: Vec<char>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let m = terminal_renderer::loader::load_obj(args.path);
    let mut mesh = terminal_renderer::math::Mesh::new(vec![]);
    if let Ok(x) = m {
        mesh = x;
    } else {
        match m {
            Err(terminal_renderer::loader::MeshError::InvalidMeshError) => panic!("Invalid mesh"),
            Err(terminal_renderer::loader::MeshError::FileNotFoundError) => {
                panic!("File Not Found")
            }
            Err(terminal_renderer::loader::MeshError::UTF8Error) => panic!("Invalid UTF8"),
            _ => (),
        }
    }

    if args.count_tris {
        println!("{}", mesh.tris.len());
        return Ok(());
    }
    let screen = terminal_renderer::renderer::Screen::new(1.5);
    let (mut max_x, mut min_x, mut max_y, mut min_y, mut max_z, mut min_z) =
        (f32::MIN, f32::MAX, f32::MIN, f32::MAX, f32::MIN, f32::MAX);

    let count = (mesh.tris.len() * 3) as f32;
    for tri in mesh.tris.iter() {
        for v in [tri.v0, tri.v1, tri.v2] {
            max_x = max_x.max(v.x);
            min_x = min_x.min(v.x);
            max_y = max_y.max(v.y);
            min_y = min_y.min(v.y);
            max_z = max_z.max(v.z);
            min_z = min_z.min(v.z);
        }
    }

    let sum_point = Vec3::new(
        (max_x + min_x) / 2.,
        (max_y + min_y) / 2.,
        (max_z + min_z) / 2.,
    );

    let largest = mesh.tris.iter().fold(0.0f32, |acc, tri| {
        acc.max((tri.v0 - sum_point).length())
            .max((tri.v1 - sum_point).length())
            .max((tri.v2 - sum_point).length())
    });

    let mut camera = terminal_renderer::renderer::Camera::new(
        sum_point + Vec3::new(0., 0., largest),
        Vec3::new(0., PI, 0.),
    );
    loop {
        screen.render(&camera, &mesh, &args.chars);
        while let Ok(true) = event::poll(Duration::from_millis(0)) {
            let _ = event::read();
        }

        match event::read()? {
            Event::Key(e) => match e {
                KeyEvent {
                    code,
                    modifiers: _,
                    kind: _,
                    state: _,
                } => match code {
                    KeyCode::Left => {
                        camera.rotation = camera.rotation.add(Vec3::new(0., -0.1, 0.));
                        camera.pos = {
                            let l = -(camera.pos - sum_point).length();
                            let dir = Vec3::new(0., 0., 1.).rotate(camera.rotation);
                            sum_point + dir * l
                        }
                    }
                    KeyCode::Right => {
                        camera.rotation = camera.rotation.add(Vec3::new(0., 0.1, 0.));
                        camera.pos = {
                            let l = -(camera.pos - sum_point).length();
                            let dir = Vec3::new(0., 0., 1.).rotate(camera.rotation);
                            sum_point + dir * l
                        }
                    }
                    KeyCode::Up => {
                        if camera.rotation.x < PI / 2. {
                            camera.rotation = camera.rotation.add(Vec3::new(0.1, 0., 0.));
                        }
                        camera.pos = {
                            let l = -(camera.pos - sum_point).length();
                            let dir = Vec3::new(0., 0., 1.).rotate(camera.rotation);
                            sum_point + dir * l
                        }
                    }
                    KeyCode::Down => {
                        if camera.rotation.x > -PI / 2. {
                            camera.rotation = camera.rotation.add(Vec3::new(-0.1, 0., 0.));
                        }
                        camera.pos = {
                            let l = -(camera.pos - sum_point).length();
                            let dir = Vec3::new(0., 0., 1.).rotate(camera.rotation);
                            sum_point + dir * l
                        }
                    }
                    KeyCode::Char('w') => {
                        camera.pos = camera.pos.add(
                            Vec3::new(0., 0., (camera.pos - sum_point).length() * 0.1)
                                .rotate(camera.rotation),
                        );
                    }
                    KeyCode::Char('s') => {
                        camera.pos = camera.pos.add(
                            Vec3::new(0., 0., -(camera.pos - sum_point).length() * 0.1)
                                .rotate(camera.rotation),
                        );
                    }
                    KeyCode::Char('e') => {
                        crossterm::terminal::disable_raw_mode();
                        panic!("exit");
                        println!("\x1b[?25h");
                    }
                    _ => (),
                },
            },
            _ => (),
        }
    }
}
