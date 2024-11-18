use std::{
    path::{Path, PathBuf},
    str::Chars,
};

use crate::math::{Mesh, Tri};
use glam::Vec3;

#[derive(Clone, Copy)]
pub enum MeshError {
    InvalidMeshError,
    FileNotFoundError,
    UTF8Error,
}

pub fn load_obj<P: AsRef<Path>>(path: P) -> Result<Mesh, MeshError> {
    let mut verts = Vec::new();
    let mut tris = Vec::new();

    let file = std::fs::read(path).map_err(|err| MeshError::FileNotFoundError)?;
    let content = String::from_utf8(file).map_err(|err| MeshError::UTF8Error)?;

    for line in content.lines() {
        let mut parts = line.split(' ');
        match parts.next() {
            Some(t) => match t {
                "v" => {
                    add_vertex(&mut verts, parts)?;
                }
                "f" => {
                    add_face(&mut tris, &verts, parts)?;
                }
                _ => (),
            },
            None => (),
        };
    }

    Ok(Mesh::new(tris))
}

fn add_face(
    tris: &mut Vec<Tri>,
    verts: &[Vec3],
    mut parts: std::str::Split<char>,
) -> Result<(), MeshError> {
    let mut collected_tris = Vec::new();

    let v1 = parts.next().ok_or(MeshError::InvalidMeshError)?;
    let v1: usize = v1
        .parse::<usize>()
        .map_err(|_| MeshError::InvalidMeshError)?
        - 1;
    let mut parts_peek = parts.peekable();
    while let Some(idx) = parts_peek.next() {
        if let Some(idx_2) = parts_peek.peek() {
            let v2 = idx
                .parse::<usize>()
                .map_err(|_| MeshError::InvalidMeshError)?
                - 1;
            let v3 = idx_2
                .parse::<usize>()
                .map_err(|_| MeshError::InvalidMeshError)?
                - 1;
            collected_tris.push(Tri::new(
                *verts.get(v1).ok_or(MeshError::InvalidMeshError)?,
                *verts.get(v2).ok_or(MeshError::InvalidMeshError)?,
                *verts.get(v3).ok_or(MeshError::InvalidMeshError)?,
                Vec3::new(255., 255., 255.),
            ));
        }
    }

    if collected_tris.is_empty() {
        return Err(MeshError::InvalidMeshError);
    }

    tris.append(&mut collected_tris);

    Ok(())
}

fn add_vertex(verts: &mut Vec<Vec3>, mut parts: std::str::Split<char>) -> Result<(), MeshError> {
    // Coordinate 1
    let c1 = parts.next().ok_or(MeshError::InvalidMeshError)?;
    let c1: f32 = c1.parse().map_err(|_| MeshError::InvalidMeshError)?;

    // Coordinate 2
    let c2 = parts.next().ok_or(MeshError::InvalidMeshError)?;
    let c2: f32 = -c2.parse().map_err(|_| MeshError::InvalidMeshError)?;

    // Coordinate 3
    let c3 = parts.next().ok_or(MeshError::InvalidMeshError)?;
    let c3: f32 = c3.parse().map_err(|_| MeshError::InvalidMeshError)?;

    // Too many coordinates
    if parts.next() != None {
        return Err(MeshError::InvalidMeshError);
    }

    verts.push(Vec3::new(c1, c2, c3));

    Ok(())
}
