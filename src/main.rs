use vec3_rs::Vector3;

fn main() {
    let screen = terminal_renderer::renderer::Screen::new(1.);
    let camera = terminal_renderer::renderer::Camera::new(Vector3::new(0., 0., -6.));
    use terminal_renderer::math::Tri as T;
    let mut t: f64 = 0.0;
    loop {
        t += 0.01;
        let mesh = vec![T::new(
            Vector3::new(-t.cos() * 5., -5., -t.sin() * 5.),
            Vector3::new(0., 5., 0.),
            Vector3::new(t.cos() * 5., -5., t.sin() * 5.),
            Vector3::new(0., 255., 0.),
        )];
        let mesh = terminal_renderer::math::Mesh::new(mesh);
        screen.render(&camera, &mesh);
    }
}
