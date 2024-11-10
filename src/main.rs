use std::{error::Error, f64::consts::PI, ops::Add, time::Duration};

use crossterm::event::{self, Event, KeyCode, KeyEvent, ModifierKeyCode};
use device_query::{DeviceQuery, DeviceState, Keycode};
use terminal_renderer::math::Rotation;
use vec3_rs::Vector3;

fn main() -> Result<(), Box<dyn Error>> {
    let screen = terminal_renderer::renderer::Screen::new(1.5);
    let mut camera = terminal_renderer::renderer::Camera::new(
        Vector3::new(0., 2., -5.),
        Vector3::new(0., 0., 0.),
    );
    let mut mesh = terminal_renderer::math::Mesh::new(vec![]);
    let m = terminal_renderer::loader::load_obj_to_mesh("./objects/trees.obj");
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

    // for tri in mesh.tris.iter() {
    //     println!("{:?}", tri);
    // }

    loop {
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
                        camera.rotation = camera.rotation.add(Vector3::new(0., -0.1, 0.));
                    }
                    KeyCode::Right => {
                        camera.rotation = camera.rotation.add(Vector3::new(0., 0.1, 0.));
                    }
                    KeyCode::Up => {
                        if camera.rotation.get_x() < PI / 2. {
                            camera.rotation = camera.rotation.add(Vector3::new(0.1, 0., 0.));
                        }
                    }
                    KeyCode::Down => {
                        if camera.rotation.get_x() > -PI / 2. {
                            camera.rotation = camera.rotation.add(Vector3::new(-0.1, 0., 0.));
                        }
                    }
                    KeyCode::Char('w') => {
                        camera.pos = camera
                            .pos
                            .add(Vector3::new(0., 0., 0.1).rotate(camera.rotation));
                    }
                    KeyCode::Char('a') => {
                        camera.pos = camera
                            .pos
                            .add(Vector3::new(-0.1, 0., 0.).rotate(camera.rotation));
                    }
                    KeyCode::Char('s') => {
                        camera.pos = camera
                            .pos
                            .add(Vector3::new(0., 0., -0.1).rotate(camera.rotation));
                    }
                    KeyCode::Char('d') => {
                        camera.pos = camera
                            .pos
                            .add(Vector3::new(0.1, 0., 0.).rotate(camera.rotation));
                    }
                    KeyCode::Char(' ') => {
                        camera.pos = camera.pos.add(Vector3::new(0., -0.1, 0.));
                    }
                    KeyCode::Enter => {
                        camera.pos = camera.pos.add(Vector3::new(0., 0.1, 0.));
                    }
                    KeyCode::Char('e') => {
                        crossterm::terminal::disable_raw_mode();
                        panic!("exit");
                    }
                    _ => (),
                },
            },
            _ => (),
        }
        screen.render(&camera, &mesh);
    }
}
