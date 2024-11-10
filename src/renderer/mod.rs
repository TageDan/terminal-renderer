use std::f64::consts::PI;

use crate::math;
use crossterm;
use math::Rotation;
use rayon::iter::{
    IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator,
    IntoParallelRefMutIterator, ParallelIterator,
};
use vec3_rs::Vector3;

pub struct Camera {
    pub pos: Vector3<f64>,
    pub rotation: Vector3<f64>,
}

impl Camera {
    pub fn new(pos: Vector3<f64>, rotation: Vector3<f64>) -> Self {
        Self { pos, rotation }
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

        crossterm::terminal::enable_raw_mode();

        screen.update_size();

        println!("\x1b[?25l");
        println!("\x1b[2J");

        return screen;
    }

    pub fn update_size(&mut self) {
        if let Ok(s) = crossterm::terminal::size() {
            self.w = s.0 as usize;
            self.h = s.1 as usize;
        }
    }

    pub fn render(&self, camera: &Camera, mesh: &math::Mesh) {
        let buffer = vec![Vector3::new(0., 0., 0.); self.w * self.h + 10];
        let buffer: Vec<_> = buffer
            .into_par_iter()
            .enumerate()
            .map(|(idx, color)| {
                let row = (idx as usize) / self.w;
                let col = (idx as usize) % self.w;
                let ray_o = camera.pos; // Ray Origin
                let row = (row as f64 / self.h as f64) * 2. - 1.; // Scale from -1 to +1
                let col = (col as f64 / self.w as f64) * 2. - 1.; // --||--
                let ray_dir = Vector3::new(col, row, self.focus_dist);
                let ray_dir = ray_dir.rotate(camera.rotation);

                // Ray
                let ray = math::Ray::new(ray_o, ray_dir);

                // Get hit triangle and distance to hit
                let (hit_tri, distance) = {
                    let hit: Vec<(f64, &math::Tri)> = mesh
                        .tris
                        .iter()
                        .filter_map(|tri| {
                            if let Some(d) = tri.hit(&ray) {
                                return Some((d, tri));
                            } else {
                                return None;
                            };
                        })
                        .collect();

                    let mut dist = f64::MAX;
                    let mut hit_tri = None;

                    for (d, tri) in hit {
                        if d > 0. && d < dist {
                            dist = d;
                            hit_tri = Some(tri)
                        }
                    }
                    (hit_tri, dist)
                };

                if let Some(t) = hit_tri {
                    let normal = t.normal();
                    let inv_dir = ray.dir * -1.;
                    let a = normal.angle(&ray.dir).min(normal.angle(&inv_dir));
                    let f = 1.0 - a.abs() / PI;
                    const RENDER_DIST: f64 = 7.;
                    let color = t.color * f * ((RENDER_DIST - distance) / RENDER_DIST).max(0.);
                    return color;
                } else {
                    return Vector3::new(0., 0., 0.);
                }
            })
            .collect();
        self.flush(&buffer);
    }

    pub fn flush(&self, buffer: &[Vector3<f64>]) {
        print!("\x1b[H"); // Move curor Home
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
