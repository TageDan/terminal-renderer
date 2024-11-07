use std::ops::Add;

use device_query::{DeviceQuery, DeviceState, Keycode};
use vec3_rs::Vector3;

fn main() {
    let screen = terminal_renderer::renderer::Screen::new(1.5);
    let mut camera = terminal_renderer::renderer::Camera::new(
        Vector3::new(0., 2., -5.),
        Vector3::new(0., 0., 0.),
    );
    let mut mesh = terminal_renderer::math::Mesh::new(vec![]);
    let m = terminal_renderer::loader::load_obj_to_mesh("./objects/cube.obj");
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

    for tri in mesh.tris.iter() {
        println!("{:?}", tri);
    }

    let state = DeviceState::new();

    loop {
        let keys = state.get_keys();
        if keys.contains(&Keycode::W) {
            camera.pos = camera.pos.add(Vector3::new(0., 0., 0.1));
        }
        if keys.contains(&Keycode::A) {
            camera.pos = camera.pos.add(Vector3::new(-0.1, 0., 0.));
        }
        if keys.contains(&Keycode::S) {
            camera.pos = camera.pos.add(Vector3::new(0., 0., -0.1));
        }
        if keys.contains(&Keycode::D) {
            camera.pos = camera.pos.add(Vector3::new(-0.1, 0., 0.));
        }
        if keys.contains(&Keycode::Up) {
            camera.rotation = camera.rotation.add(Vector3::new(0., -0.1, 0.));
        }
        if keys.contains(&Keycode::Left) {
            camera.pos = camera.pos.add(Vector3::new(0., 0., 0.1));
        }
        if keys.contains(&Keycode::Down) {
            camera.pos = camera.pos.add(Vector3::new(0., 0.1, 0.));
        }
        if keys.contains(&Keycode::Right) {
            camera.pos = camera.pos.add(Vector3::new(0., 0., -0.1));
        }

        screen.render(&camera, &mesh);
    }
}
