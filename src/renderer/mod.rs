use core::panic;
use std::{f32::consts::PI, rc::Rc, sync::Arc};

use crate::math;
use crossterm;
use math::Rotation;
use rayon::iter::{
    IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator,
    IntoParallelRefMutIterator, ParallelIterator,
};

use glam::Vec3;

pub struct Camera {
    pub pos: Vec3,
    pub rotation: Vec3,
}

impl Camera {
    pub fn new(pos: Vec3, rotation: Vec3) -> Self {
        Self { pos, rotation }
    }
}

pub struct Screen {
    w: usize,
    h: usize,
    focus_dist: f32,
}

impl Drop for Screen {
    fn drop(&mut self) {
        crossterm::terminal::disable_raw_mode();
        println!("\x1b[?25h");
    }
}

impl Screen {
    pub fn new(focus_dist: f32) -> Self {
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

    pub fn render(&self, camera: &Camera, mesh: &math::Mesh, char_buffer: &[char]) {
        let buffer = vec![Vec3::new(0., 0., 0.); self.w * self.h];
        let forward = Vec3::new(0., 0., 1.).rotate(camera.rotation);
        let tris = mesh
            .tris
            .par_iter()
            .filter(|tri| {
                let v0 = tri.v0 - camera.pos;
                let v1 = tri.v1 - camera.pos;
                let v2 = tri.v2 - camera.pos;
                forward.dot(v0) > 0. || forward.dot(v1) > 0. || forward.dot(v2) > 0.
            })
            .collect::<Vec<_>>();
        let scale = self.w.min(self.h * 2);
        let buffer: Vec<_> = buffer
            .into_par_iter()
            .enumerate()
            .map(|(idx, color)| {
                let row = (idx as usize) / self.w;
                let col = (idx as usize) % self.w;
                let ray_o = camera.pos; // Ray Origin
                let row = (row as f32 * 2 as f32 / scale as f32) * 2. - 1.; // Scale from -1 to +1
                let col = (col as f32 / scale as f32) * 2. - 1.; // --||--
                let ray_dir = Vec3::new(col, row, self.focus_dist);
                let ray_dir = ray_dir.rotate(camera.rotation);

                // Ray
                let ray = math::Ray::new(ray_o, ray_dir);

                let mut aabb_check_list = [0f32; 9];

                // Get hit triangle and distance to hit
                let hit = tris
                    .iter()
                    .filter(|tri| {
                        aabb_check_list[0] = (tri.v_min.x - ray_o.x) / ray_dir.x;
                        aabb_check_list[1] = (tri.v_max.x - ray_o.x) / ray_dir.x;
                        aabb_check_list[2] = (tri.v_min.y - ray_o.y) / ray_dir.y;
                        aabb_check_list[3] = (tri.v_max.y - ray_o.y) / ray_dir.y;
                        aabb_check_list[4] = (tri.v_min.z - ray_o.z) / ray_dir.z;
                        aabb_check_list[5] = (tri.v_max.z - ray_o.z) / ray_dir.z;
                        aabb_check_list[6] = aabb_check_list[0]
                            .min(aabb_check_list[1])
                            .max(aabb_check_list[2].min(aabb_check_list[3]))
                            .max(aabb_check_list[4].min(aabb_check_list[5]));
                        aabb_check_list[7] = aabb_check_list[0]
                            .max(aabb_check_list[1])
                            .min(aabb_check_list[2].max(aabb_check_list[3]))
                            .min(aabb_check_list[4].max(aabb_check_list[5]));
                        !(aabb_check_list[7] < 0. || aabb_check_list[6] > aabb_check_list[7])
                    })
                    .fold(None, |acc, tri| {
                        if let Some(d) = tri.hit(&ray) {
                            if d < 0. {
                                return acc;
                            };
                            if let Some((d2, _)) = acc {
                                if d < d2 {
                                    return Some((d, *tri));
                                } else {
                                    return acc;
                                };
                            } else {
                                return Some((d, *tri));
                            }
                        } else {
                            return acc;
                        };
                    });

                if let Some((d, t)) = hit {
                    let normal = t.normal();
                    let inv_dir = ray.dir * -1.;
                    let a = normal.dot(ray.dir).max(normal.dot(inv_dir));
                    let f = a / (normal.length() * inv_dir.length());
                    // let f = f.sqrt();
                    const RENDER_DIST: f32 = 100_000.;
                    let color = t.color * f * ((RENDER_DIST - d) / RENDER_DIST).max(0.);
                    return color;
                } else {
                    return Vec3::new(0., 0., 0.);
                }
            })
            .collect();
        self.flush(&buffer, char_buffer);
    }

    pub fn flush(&self, buffer: &[Vec3], char_buffer: &[char]) {
        print!("\x1b[H"); // Move curor Home
        let mut last_color = Vec3::new(0., 0., 0.);
        print!(
            "\x1b[48;2;{r};{g};{b}m",
            r = last_color.x as u8,
            g = last_color.y as u8,
            b = last_color.z as u8
        );
        for row in 0..self.h {
            for col in 0..self.w {
                let color = buffer[row * self.w + col];
                if color != last_color {
                    print!(
                        "\x1b[{t};2;{r};{g};{b}m",
                        r = color.x as u8,
                        g = color.y as u8,
                        b = color.z as u8,
                        t = match char_buffer.len() {
                            0 => "48",
                            _ => "38",
                        },
                    );
                    last_color = color;
                }
                if char_buffer.len() != 0 {
                    let light = color.element_sum() / (255. * 3.);
                    let light = ((light * char_buffer.len() as f32) as usize)
                        .clamp(0, char_buffer.len() - 1);
                    print!("{}", char_buffer[light]);
                } else {
                    print!(" ");
                }
            }
            if row != self.h - 1 {
                println!("\r");
            }
        }
        print!("\x1b[48;2;0;0;0m\r");
    }

    pub fn render_octree(&self, camera: &Camera, mesh: &math::Mesh, char_buffer: &[char]) {
        let buffer = vec![Vec3::new(0., 0., 0.); self.w * self.h + 10];
        let forward = Vec3::new(0., 0., 1.).rotate(camera.rotation);
        let tris = mesh
            .tris
            .par_iter()
            .filter(|tri| {
                let v0 = tri.v0 - camera.pos;
                let v1 = tri.v1 - camera.pos;
                let v2 = tri.v2 - camera.pos;
                forward.dot(v0) > 0. || forward.dot(v1) > 0. || forward.dot(v2) > 0.
            })
            .collect::<Vec<_>>();
        let (mut min_v, mut max_v) = (Vec3::MAX, Vec3::MIN);
        for tri in &tris {
            for v in [tri.v_min, tri.v_max] {
                min_v.x = min_v.x.min(v.x);
                min_v.y = min_v.y.min(v.y);
                min_v.z = min_v.z.min(v.z);
                max_v.x = max_v.x.max(v.x);
                max_v.y = max_v.y.max(v.y);
                max_v.z = max_v.z.max(v.z);
            }
        }

        let mut octree = math::Octree::new(min_v, max_v);

        for tri in &tris {
            octree.insert(Arc::new((*tri).clone()));
        }

        let scale = self.w.min(self.h * 2);
        let buffer: Vec<_> = buffer
            .into_par_iter()
            .enumerate()
            .map(|(idx, color)| {
                let row = (idx as usize) / self.w;
                let col = (idx as usize) % self.w;
                let ray_o = camera.pos; // Ray Origin
                let row = (row as f32 * 2 as f32 / scale as f32) * 2. - 1.; // Scale from -1 to +1
                let col = (col as f32 / scale as f32) * 2. - 1.; // --||--
                let ray_dir = Vec3::new(col, row, self.focus_dist);
                let ray_dir = ray_dir.rotate(camera.rotation);

                // Ray
                let ray = math::Ray::new(ray_o, ray_dir);

                let mut aabb_check_list = [0f32; 9];

                // Get hit triangle and distance to hit
                let hit = octree
                    .ray_search_tree(ray_o, ray_dir)
                    .iter()
                    .fold(None, |acc, tri| {
                        if let Some(d) = tri.hit(&ray) {
                            if d < 0. {
                                return acc;
                            };
                            if let Some((d2, _)) = acc {
                                if d < d2 {
                                    return Some((d, tri.clone()));
                                } else {
                                    return acc;
                                };
                            } else {
                                return Some((d, tri.clone()));
                            }
                        } else {
                            return acc;
                        };
                    });

                if let Some((d, t)) = hit {
                    let normal = t.normal();
                    let inv_dir = ray.dir * -1.;
                    let a = normal.dot(ray.dir).max(normal.dot(inv_dir));
                    let f = a / (normal.length() * inv_dir.length());
                    // let f = f.sqrt();
                    const RENDER_DIST: f32 = 100_000.;
                    let color = t.color * f * ((RENDER_DIST - d) / RENDER_DIST).max(0.);
                    return color;
                } else {
                    return Vec3::new(0., 0., 0.);
                }
            })
            .collect();
        self.flush(&buffer, char_buffer);
    }
}
