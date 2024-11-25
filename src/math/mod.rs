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

#[derive(Debug, Clone)]
pub struct Tri {
    pub v0: Vec3,
    pub v1: Vec3,
    pub v2: Vec3,
    pub color: Vec3,
    pub v_min: Vec3,
    pub v_max: Vec3,
}

pub trait Rotation {
    fn rotation_x(&self, a: f32) -> Self;
    fn rotation_y(&self, a: f32) -> Self;
    fn rotation_z(&self, a: f32) -> Self;
    fn rotate(&self, a: Vec3) -> Self
    where
        Self: Sized,
    {
        self.rotation_z(a.z).rotation_x(a.x).rotation_y(a.y)
    }
    fn rev_rotate(&self, a: Vec3) -> Self
    where
        Self: Sized,
    {
        self.rotation_y(-a.y).rotation_x(-a.x).rotation_z(-a.z)
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
        let v_min = Vec3::new(
            v0.x.min(v1.x).min(v2.x),
            v0.y.min(v1.y).min(v2.y),
            v0.z.min(v1.z).min(v2.z),
        );
        let v_max = Vec3::new(
            v0.x.max(v1.x).max(v2.x),
            v0.y.max(v1.y).max(v2.y),
            v0.z.max(v1.z).max(v2.z),
        );
        Self {
            v0,
            v1,
            v2,
            color,
            v_min,
            v_max,
        }
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

#[derive(Clone)]
pub struct Mesh {
    pub tris: Arc<[Tri]>,
}

impl Mesh {
    pub fn new(tris: Vec<Tri>) -> Self {
        Self { tris: tris.into() }
    }
}

#[derive(Debug)]
pub struct Octree {
    top_left_front: Vec3,
    bottom_right_back: Vec3,
    middle: Vec3,
    max_nodes: usize,
    inserted: usize,
    node: OctreeNode,
}

#[derive(Debug)]
pub enum OctreeNode {
    None,
    Leaf {
        tri: Vec<Arc<Tri>>,
    },
    Node {
        tri: Vec<Arc<Tri>>,
        children: [Box<Octree>; 8],
    },
}

impl Octree {
    pub fn new(top_left_front: Vec3, bottom_right_back: Vec3) -> Self {
        Self {
            top_left_front,
            bottom_right_back,
            middle: (top_left_front + bottom_right_back) / 2.,
            max_nodes: 1,
            inserted: 0,
            node: OctreeNode::None,
        }
    }

    pub fn with_max_nodes(self, max_nodes: usize) -> Self {
        Self {
            top_left_front: self.top_left_front,
            bottom_right_back: self.bottom_right_back,
            middle: (self.top_left_front + self.bottom_right_back) / 2.,
            inserted: self.inserted,
            max_nodes,
            node: self.node,
        }
    }

    pub fn insert(&mut self, tri: Arc<Tri>) {
        let insert = self.should_insert_tri(tri.clone());
        self.inserted += 1;
        match self.node {
            OctreeNode::None => {
                let mut tris = Vec::with_capacity(5);
                tris.push(tri);
                self.node = OctreeNode::Leaf { tri: tris };
            }
            OctreeNode::Node {
                tri: _,
                ref mut children,
            } => {
                for i in 0..8 {
                    if (insert >> i) & 1 == 1 {
                        children[i].insert(tri.clone());
                    }
                }
            }
            OctreeNode::Leaf {
                tri: ref mut innertri,
            } => {
                if innertri.len() < self.max_nodes {
                    innertri.push(tri.clone());
                    return;
                }
                let children = [
                    // bottom_right back
                    Box::new(
                        Octree::new(self.middle, self.bottom_right_back)
                            .with_max_nodes(self.max_nodes * 3),
                    ),
                    // bottom_right_front
                    Box::new(
                        Octree::new(
                            Vec3::new(self.middle.x, self.middle.y, self.top_left_front.z),
                            Vec3::new(
                                self.bottom_right_back.x,
                                self.bottom_right_back.y,
                                self.middle.z,
                            ),
                        )
                        .with_max_nodes(self.max_nodes * 3),
                    ),
                    // top_right_back
                    Box::new(
                        Octree::new(
                            Vec3::new(self.middle.x, self.top_left_front.y, self.middle.z),
                            Vec3::new(
                                self.bottom_right_back.x,
                                self.middle.y,
                                self.bottom_right_back.z,
                            ),
                        )
                        .with_max_nodes(self.max_nodes * 3),
                    ),
                    // top_right_front
                    Box::new(
                        Octree::new(
                            Vec3::new(self.middle.x, self.top_left_front.y, self.top_left_front.z),
                            Vec3::new(self.bottom_right_back.x, self.middle.y, self.middle.z),
                        )
                        .with_max_nodes(self.max_nodes * 3),
                    ),
                    // bottom_left back
                    Box::new(
                        Octree::new(
                            Vec3::new(self.top_left_front.x, self.middle.y, self.middle.z),
                            Vec3::new(
                                self.middle.x,
                                self.bottom_right_back.y,
                                self.bottom_right_back.z,
                            ),
                        )
                        .with_max_nodes(self.max_nodes * 3),
                    ),
                    // bottom_left_front
                    Box::new(
                        Octree::new(
                            Vec3::new(self.top_left_front.x, self.middle.y, self.top_left_front.z),
                            Vec3::new(self.middle.x, self.bottom_right_back.y, self.middle.z),
                        )
                        .with_max_nodes(self.max_nodes * 3),
                    ),
                    // top_left_back
                    Box::new(
                        Octree::new(
                            Vec3::new(self.top_left_front.x, self.top_left_front.y, self.middle.z),
                            Vec3::new(self.middle.x, self.middle.y, self.bottom_right_back.z),
                        )
                        .with_max_nodes(self.max_nodes * 3),
                    ),
                    // top_left_front
                    Box::new(
                        Octree::new(self.top_left_front, self.middle)
                            .with_max_nodes(self.max_nodes * 3),
                    ),
                ];

                self.node = OctreeNode::Node {
                    tri: innertri.clone(),
                    children,
                };
                self.insert(tri);
            }
        }
    }

    fn intersects(&self, ray_o: Vec3, ray_dir: Vec3) -> bool {
        let mut aabb_check_list = [0f32; 8];
        aabb_check_list[0] = (self.top_left_front.x - ray_o.x) / ray_dir.x;
        aabb_check_list[1] = (self.bottom_right_back.x - ray_o.x) / ray_dir.x;
        aabb_check_list[2] = (self.top_left_front.y - ray_o.y) / ray_dir.y;
        aabb_check_list[3] = (self.bottom_right_back.y - ray_o.y) / ray_dir.y;
        aabb_check_list[4] = (self.top_left_front.z - ray_o.z) / ray_dir.z;
        aabb_check_list[5] = (self.bottom_right_back.z - ray_o.z) / ray_dir.z;
        aabb_check_list[6] = aabb_check_list[0]
            .min(aabb_check_list[1])
            .max(aabb_check_list[2].min(aabb_check_list[3]))
            .max(aabb_check_list[4].min(aabb_check_list[5]));
        aabb_check_list[7] = aabb_check_list[0]
            .max(aabb_check_list[1])
            .min(aabb_check_list[2].max(aabb_check_list[3]))
            .min(aabb_check_list[4].max(aabb_check_list[5]));
        !(aabb_check_list[7] < 0. || aabb_check_list[6] > aabb_check_list[7])
    }

    pub fn ray_search_tree(&self, ro: Vec3, rd: Vec3) -> Vec<Arc<Tri>> {
        let mut result = Vec::with_capacity(self.inserted);

        match self.node {
            OctreeNode::None => (),
            OctreeNode::Leaf { ref tri } => result.append(&mut tri.clone()),
            OctreeNode::Node {
                ref tri,
                ref children,
            } => {
                result.append(&mut tri.clone());
                children
                    .iter()
                    .filter(|node| node.intersects(ro, rd))
                    .for_each(|node| result.append(&mut node.ray_search_tree(ro, rd)));
            }
        }

        result
    }

    fn should_insert_tri(&self, tri: Arc<Tri>) -> u8 {
        let mut should_insert = 0u8;
        for v in [tri.v0, tri.v1, tri.v2] {
            if v.x > self.middle.x {
                if v.y > self.middle.y {
                    if v.z > self.middle.z {
                        should_insert |= 0b00000001;
                    } else {
                        should_insert |= 0b00000010;
                    }
                } else {
                    if v.z > self.middle.z {
                        should_insert |= 0b00000100;
                    } else {
                        should_insert |= 0b00001000;
                    }
                }
            } else {
                if v.y > self.middle.y {
                    if v.z > self.middle.z {
                        should_insert |= 0b00010000;
                    } else {
                        should_insert |= 0b00100000;
                    }
                } else {
                    if v.z > self.middle.z {
                        should_insert |= 0b01000000;
                    } else {
                        should_insert |= 0b10000000;
                    }
                }
            }
        }
        should_insert
    }
}
