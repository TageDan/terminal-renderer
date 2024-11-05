use std::f64::consts::PI;

use crate::math;
use term_size;
use vec3_rs::Vector3;

pub struct Camera {
    pub pos: Vector3<f64>,
}

impl Camera {
    pub fn new(pos: Vector3<f64>) -> Self {
        Self { pos }
    }
}

pub struct Screen {
    w: usize,
    h: usize,
    focus_dist: f64,
}

impl Screen {
    pub fn new(focus_dist: f64) -> Self {
        let mut screen = Self {
            w: 0,
            h: 0,
            focus_dist,
        };

        screen.update_size();

        println!("\x1b[?25l");
        println!("\x1b[2J");

        return screen;
    }

    pub fn update_size(&mut self) {
        if let Some(s) = term_size::dimensions() {
            self.w = s.0;
            self.h = s.1;
        }
    }

    pub fn render(&self, camera: &Camera, mesh: &math::Mesh) {
        let mut buffer = Vec::with_capacity(self.w * self.h + 10);
        for row in 0..self.h {
            for col in 0..self.w {
                let ray_o = camera.pos; // Ray Origin
                let row = (row as f64 / self.h as f64) * 2. - 1.; // Scale from -1 to +1
                let col = (col as f64 / self.w as f64) * 2. - 1.; // --||--

                // Ray
                let ray = math::Ray::new(ray_o, Vector3::new(col, row, self.focus_dist));

                // Get hit triangle and distance to hit
                let (hit_tri, distance) = {
                    let mut hit_tri = None;
                    let mut dist = f64::MAX;
                    for tri in mesh.tris.iter() {
                        if let Some(d) = tri.hit(&ray) {
                            if d < dist {
                                dist = d;
                                hit_tri = Some(tri);
                            };
                        };
                    }
                    (hit_tri, dist)
                };

                if let Some(t) = hit_tri {
                    let normal = t.normal();
                    let inv_dir = ray.dir * -1.;
                    let a = normal.angle(&ray.dir).min(normal.angle(&inv_dir));
                    let f = 1.0 - a.abs() / PI;
                    const RENDER_DIST: f64 = 75.;
                    let color = t.color * f * ((RENDER_DIST - distance) / RENDER_DIST).max(0.);
                    buffer.push(color);
                } else {
                    buffer.push(Vector3::new(0., 0., 0.));
                }
            }
        }
        self.flush(&buffer);
    }

    pub fn flush(&self, buffer: &[Vector3<f64>]) {
        println!("\x1b[H"); // Move curor Home
        for row in 0..self.h {
            for col in 0..self.w {
                let color = buffer[row * self.w + col];
                print!(
                    "\x1b[48;2;{r};{g};{b}m ",
                    r = color.get_x() as u8,
                    g = color.get_y() as u8,
                    b = color.get_z() as u8
                );
            }
            if row != self.h - 1 {
                println!("\r");
            }
        }
        print!("\x1b[48;2;0;0;0m\r");
    }
}
