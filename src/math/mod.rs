use std::{rc::Rc, sync::Arc};

use vec3_rs::Vector3;

pub struct Ray {
    pub origin: Vector3<f64>,
    pub dir: Vector3<f64>,
}

impl Ray {
    pub fn new(origin: Vector3<f64>, dir: Vector3<f64>) -> Self {
        Self { origin, dir }
    }
}

#[derive(Debug)]
pub struct Tri {
    pub v0: Vector3<f64>,
    pub v1: Vector3<f64>,
    pub v2: Vector3<f64>,
    pub color: Vector3<f64>,
}

pub trait Rotation {
    fn rotation_x(&self, a: f64) -> Self;
    fn rotation_y(&self, a: f64) -> Self;
    fn rotation_z(&self, a: f64) -> Self;
    fn rotate(&self, a: Vector3<f64>) -> Self
    where
        Self: Sized,
    {
        self.rotation_x(a.get_x())
            .rotation_y(a.get_y())
            .rotation_z(a.get_z())
    }
}

impl Rotation for Vector3<f64> {
    fn rotation_z(&self, a: f64) -> Self {
        let x = self.get_x();
        let y = self.get_y();
        let z = self.get_z();
        let nx = a.cos() * x - a.sin() * y;
        let ny = a.sin() * x + a.cos() * y;
        Vector3::new(nx, ny, z)
    }
    fn rotation_y(&self, a: f64) -> Self {
        let x = self.get_x();
        let y = self.get_y();
        let z = self.get_z();
        let nx = a.cos() * x + a.sin() * z;
        let nz = -a.sin() * x + a.cos() * z;
        Vector3::new(nx, y, nz)
    }
    fn rotation_x(&self, a: f64) -> Self {
        let x = self.get_x();
        let y = self.get_y();
        let z = self.get_z();
        let ny = a.cos() * y - a.sin() * z;
        let nz = a.sin() * y + a.cos() * z;
        Vector3::new(x, ny, nz)
    }
}

impl Tri {
    pub fn new(v0: Vector3<f64>, v1: Vector3<f64>, v2: Vector3<f64>, color: Vector3<f64>) -> Self {
        Self { v0, v1, v2, color }
    }

    /// Normal vector for triangle
    pub fn normal(&self) -> Vector3<f64> {
        let e1 = self.v1 - self.v0; // edge 1
        let e2 = self.v2 - self.v0; // edge 2
        e1.cross(&e2) // Normal Vector
    }

    // Möller-Trumbore algo (https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-rendering-a-triangle/moller-trumbore-ray-triangle-intersection.html)
    pub fn hit(&self, ray: &Ray) -> Option<f64> {
        let e1 = self.v1 - self.v0;
        let e2 = self.v2 - self.v0;
        let p = ray.dir.cross(&e2);
        let det = e1.dot(&p);
        const EPSILON: f64 = 0.001;

        // If determinant is close to zero the ray and triangle are parallel
        if det.abs() < EPSILON {
            return None;
        }

        let inv_det = 1. / det;
        let t = ray.origin - self.v0;
        let u = t.dot(&p) * inv_det;
        if u < 0. || u > 1. {
            return None;
        };

        let q = t.cross(&e1);
        let v = ray.dir.dot(&q) * inv_det;
        if (v < 0. || u + v > 1.) {
            return None;
        }
        let t = e2.dot(&q) * inv_det;
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
