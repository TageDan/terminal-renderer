use std::{rc::Rc, sync::Arc};

use glam::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, dir: Vec3) -> Self {
        Self { origin, dir }
    }
}

#[derive(Debug)]
pub struct Tri {
    pub v0: Vec3,
    pub v1: Vec3,
    pub v2: Vec3,
    pub color: Vec3,
}

pub trait Rotation {
    fn rotation_x(&self, a: f32) -> Self;
    fn rotation_y(&self, a: f32) -> Self;
    fn rotation_z(&self, a: f32) -> Self;
    fn rotate(&self, a: Vec3) -> Self
    where
        Self: Sized,
    {
        self.rotation_x(a.x).rotation_y(a.y).rotation_z(a.z)
    }
}

impl Rotation for Vec3 {
    fn rotation_z(&self, a: f32) -> Self {
        let x = self.x;
        let y = self.y;
        let z = self.z;
        let nx = a.cos() * x - a.sin() * y;
        let ny = a.sin() * x + a.cos() * y;
        Vec3::new(nx, ny, z)
    }
    fn rotation_y(&self, a: f32) -> Self {
        let x = self.x;
        let y = self.y;
        let z = self.z;
        let nx = a.cos() * x + a.sin() * z;
        let nz = -a.sin() * x + a.cos() * z;
        Vec3::new(nx, y, nz)
    }
    fn rotation_x(&self, a: f32) -> Self {
        let x = self.x;
        let y = self.y;
        let z = self.z;
        let ny = a.cos() * y - a.sin() * z;
        let nz = a.sin() * y + a.cos() * z;
        Vec3::new(x, ny, nz)
    }
}

impl Tri {
    pub fn new(v0: Vec3, v1: Vec3, v2: Vec3, color: Vec3) -> Self {
        Self { v0, v1, v2, color }
    }

    /// Normal vector for triangle
    pub fn normal(&self) -> Vec3 {
        let e1 = self.v1 - self.v0; // edge 1
        let e2 = self.v2 - self.v0; // edge 2
        e1.cross(e2) // Normal Vector
    }

    // Möller-Trumbore algo (https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-rendering-a-triangle/moller-trumbore-ray-triangle-intersection.html)
    pub fn hit(&self, ray: &Ray) -> Option<f32> {
        let e1 = self.v1 - self.v0;
        let e2 = self.v2 - self.v0;
        let p = ray.dir.cross(e2);
        let det = e1.dot(p);
        const EPSILON: f32 = 0.001;

        // If determinant is close to zero the ray and triangle are parallel
        if det.abs() < EPSILON {
            return None;
        }

        let inv_det = 1. / det;
        let t = ray.origin - self.v0;
        let u = t.dot(p) * inv_det;
        if u < 0. || u > 1. {
            return None;
        };

        let q = t.cross(e1);
        let v = ray.dir.dot(q) * inv_det;
        if (v < 0. || u + v > 1.) {
            return None;
        }
        let t = e2.dot(q) * inv_det;
        if t < 0. {
            return None;
        }
        Some(t)
    }
}

pub struct Mesh {
    pub tris: Arc<[Tri]>,
}

impl Mesh {
    pub fn new(tris: Vec<Tri>) -> Self {
        Self { tris: tris.into() }
    }
}
