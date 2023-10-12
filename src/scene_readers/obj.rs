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

        if pos.len() % 3 != 0 {
            return Err("Positions must be a multiple of 3".into());
        }

        for v in 0..pos.len() / 9 {
            let v = 9 * v;
            let triangle = triangle::Triangle::new(
                Vec3::new(pos[v + 0], pos[v + 1], pos[v + 2]),
                Vec3::new(pos[v + 3], pos[v + 4], pos[v + 5]),
                Vec3::new(pos[v + 6], pos[v + 7], pos[v + 8]),
                Vec3::homogeneous(255),
            );
            triangles.push(triangle);
        }
    }
    Ok(triangles)
}
