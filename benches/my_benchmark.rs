use std::sync::Arc;

use criterion::*;
use glam::Vec3;
use terminal_renderer::math::Octree;
use terminal_renderer::math::Tri;

fn octre_insert_tris(c: &mut Criterion) {
    let mut tris = Vec::with_capacity(5000);
    for _ in 0..5000 {
        tris.push(Tri::new(
            Vec3::new(
                rand::random::<f32>() * 1.5 - 0.75,
                rand::random::<f32>() * 1.5 - 0.75,
                rand::random::<f32>() * 1.5 - 0.75,
            ),
            Vec3::new(
                rand::random::<f32>() * 1.5 - 0.75,
                rand::random::<f32>() * 1.5 - 0.75,
                rand::random::<f32>() * 1.5 - 0.75,
            ),
            Vec3::new(
                rand::random::<f32>() * 1.5 - 0.75,
                rand::random::<f32>() * 1.5 - 0.75,
                rand::random::<f32>() * 1.5 - 0.75,
            ),
            Vec3::new(255., 255., 255.),
        ));
    }

    let mut octree = Octree::new(Vec3::splat(-1.0), Vec3::splat(1.0));
    c.bench_function("insert 5000", |b| {
        b.iter(|| {
            octree = Octree::new(Vec3::splat(-1.0), Vec3::splat(1.0));
            for tri in &tris {
                octree.insert(Arc::new((*tri).clone()))
            }
        })
    });
    c.bench_function("search 5000", |b| {
        b.iter(|| {
            for _ in 0..tris.len() {
                octree.ray_search_tree(
                    Vec3 {
                        x: 0.,
                        y: 0.,
                        z: -2.,
                    },
                    Vec3::new(
                        rand::random::<f32>() * 2. - 1.,
                        rand::random::<f32>() * 2. - 1.,
                        1.0,
                    ),
                );
            }
        })
    });
}

criterion_group!(benches, octre_insert_tris);
criterion_main!(benches);
