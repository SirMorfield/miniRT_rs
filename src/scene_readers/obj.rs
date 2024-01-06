use super::{default_ambient, look_at, Scene};
use crate::light::Light;
use crate::num::Float0to1;
use crate::octree::Octree;
use crate::triangle;
use crate::triangle::Triangle;
use crate::vector::Vec3;
use tobj;

pub fn read_obj(path: &std::path::Path) -> Result<Scene, String> {
    let path = path.display().to_string();
    if !path.ends_with(".obj") {
        return Err("File must end with .rt".into());
    }
    let obj = tobj::load_obj(path, &tobj::GPU_LOAD_OPTIONS);
    let (models, _materials) = obj.expect("Failed to load OBJ file");
    let triangles = parse_triangle(models)?;
    if triangles.len() == 0 {
        return Err("No triangles found".into());
    }
    let camera = look_at(&triangles);
    let mut lights: Vec<Light> = vec![];

    lights.push(Light::new(
        camera.pos + 20.0,
        Float0to1::new(0.5).unwrap(),
        Vec3::new(50, 255, 50),
    ));
    return Ok(Scene::new(
        camera,
        Octree::new(triangles),
        lights,
        default_ambient(),
    ));
}

fn parse_triangle(models: Vec<tobj::Model>) -> Result<Vec<Triangle>, String> {
    let mut triangles: Vec<Triangle> = Vec::new();

    for (_, m) in models.iter().enumerate() {
        let pos = &m.mesh.positions;
        let idx = &m.mesh.indices;

        if idx.len() % 3 != 0 {
            return Err("Indices must be a multiple of 3".into());
        }

        for i in (0..idx.len()).step_by(3) {
            let p0 = idx[i + 0] as usize * 3;
            let p1 = idx[i + 1] as usize * 3;
            let p2 = idx[i + 2] as usize * 3;

            let triangle = triangle::Triangle::new(
                Vec3::new(pos[p0], pos[p0 + 1], pos[p0 + 2]),
                Vec3::new(pos[p1], pos[p1 + 1], pos[p1 + 2]),
                Vec3::new(pos[p2], pos[p2 + 1], pos[p2 + 2]),
                Vec3::homogeneous(255),
            );
            triangles.push(triangle);
        }
    }
    Ok(triangles)
}
