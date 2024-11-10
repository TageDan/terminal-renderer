use std::{
    path::{Path, PathBuf},
    str::Chars,
};

use crate::math::{Mesh, Tri};
use vec3_rs::Vector3;

#[derive(Clone, Copy)]
pub enum MeshError {
    InvalidMeshError,
    FileNotFoundError,
    UTF8Error,
}

#[derive(Eq, PartialEq)]
enum LoadingState {
    VertexCollection,
    TriCollection,
}

fn add_face(tris: &mut Vec<Tri>, verts: &[Vector3<f64>], chars: Chars) -> Result<(), MeshError> {
    let mut line = chars;
    line.next();
    let line = line.collect::<String>();
    let mut vert_idx = Vec::new();
    for id in line.split(' ') {
        vert_idx.push(
            id.parse::<usize>()
                .map_err(|_| MeshError::InvalidMeshError)?
                - 1,
        );
    }

    println!("{:?}\n", vert_idx);

    tris.push(Tri::new(
        verts[*vert_idx.get(0).ok_or(MeshError::InvalidMeshError)?],
        verts[*vert_idx.get(2).ok_or(MeshError::InvalidMeshError)?],
        verts[*vert_idx.get(1).ok_or(MeshError::InvalidMeshError)?],
        Vector3::new(255., 255., 255.),
    ));

    Ok(())
}

fn add_vertex(vertices: &mut Vec<Vector3<f64>>, line: Chars) -> Result<(), MeshError> {
    let coords = line
        .collect::<String>()
        .trim()
        .split(' ')
        .map(|s| {
            s.trim()
                .parse::<f64>()
                .map_err(|_| MeshError::InvalidMeshError)
        })
        .collect::<Vec<_>>();
    let c1 = &coords[0].as_ref().map_err(|err| *err)?;
    let c2 = &coords[1].as_ref().map_err(|err| *err)?;
    let c3 = &coords[2].as_ref().map_err(|err| *err)?;
    vertices.push(Vector3::new(**c1 * (-1.), **c2 * (-1.), **c3 * (-1.)));
    Ok(())
}

pub fn load_obj_to_mesh<P: AsRef<Path>>(path: P) -> Result<Mesh, MeshError> {
    let mut verts = Vec::new();
    let mut tris = Vec::new();

    let file = std::fs::read(path).map_err(|err| MeshError::FileNotFoundError)?;
    let content = String::from_utf8(file).map_err(|err| MeshError::UTF8Error)?;

    let mut state = LoadingState::VertexCollection;

    for line in content.lines() {
        let mut chars = line.chars();
        match chars.next() {
            Some(c) => match c {
                'v' => {
                    if state == LoadingState::TriCollection {
                        state = LoadingState::VertexCollection;
                        // verts = Vec::new();
                    }
                    add_vertex(&mut verts, chars)?;
                }
                'f' => {
                    state = LoadingState::TriCollection;
                    add_face(&mut tris, &verts, chars)?;
                }
                _ => (),
            },
            None => (),
        };
    }

    println!("{:?}\n", verts);

    Ok(Mesh::new(tris))
}
