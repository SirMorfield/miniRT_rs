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
    let mut opt = tobj::GPU_LOAD_OPTIONS;
    opt.single_index = false;
    let obj = tobj::load_obj(path, &opt);
    let (models, _materials) = obj.expect("Failed to load OBJ file");
    let triangles = parse_triangle(models)?;
    if triangles.len() == 0 {
        return Err("No triangles found".into());
    }
    let camera = look_at(&triangles);
    let mut lights: Vec<Light> = vec![];

    lights.push(Light::new(
        camera.pos + 10.0,
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

fn validate_triangle(t: &Triangle) -> Result<(), String> {
    let err = Err("Triangle has two identical points".into());
    if t.p0 == t.p1 {
        return err;
    }
    if t.p0 == t.p2 {
        return err;
    }
    if t.p1 == t.p2 {
        return err;
    }
    Ok(())
}

fn validate_mesh(mesh: &tobj::Mesh) -> Result<(), String> {
    let vertices = &mesh.positions;
    let vertices_i = &mesh.indices;
    let normals = &mesh.normals;
    let normals_i = &mesh.normal_indices;

    if vertices_i.len() % 3 != 0 {
        return Err("Indices must be a multiple of 3".into());
    }
    if vertices_i.len() != normals_i.len() {
        return Err("Indices and normals must be the same length".into());
    }
    if normals.iter().find(|n| !n.is_finite()).is_some() {
        return Err("Normal is not finite".into());
    }
    if vertices.iter().find(|n| !n.is_finite()).is_some() {
        return Err("Vertex is not finite".into());
    }
    Ok(())
}

fn parse_triangle(models: Vec<tobj::Model>) -> Result<Vec<Triangle>, String> {
    let mut triangles: Vec<Triangle> = Vec::new();

    for (_, m) in models.iter().enumerate() {
        let vertices = &m.mesh.positions;
        let vertices_i = &m.mesh.indices;
        let normals = &m.mesh.normals;
        let normals_i = &m.mesh.normal_indices;
        let mut failed: usize = 0;

        validate_mesh(&m.mesh)?;

        for i in (0..vertices_i.len()).step_by(3) {
            let p0 = vertices_i[i + 0] as usize * 3;
            let p1 = vertices_i[i + 1] as usize * 3;
            let p2 = vertices_i[i + 2] as usize * 3;
            let n0 = normals_i[i + 0] as usize * 3;
            let n1 = normals_i[i + 1] as usize * 3;
            let n2 = normals_i[i + 2] as usize * 3;

            let triangle = triangle::Triangle::new_with_vertex_normals(
                Vec3::new(vertices[p0], vertices[p0 + 1], vertices[p0 + 2]),
                Vec3::new(vertices[p1], vertices[p1 + 1], vertices[p1 + 2]),
                Vec3::new(vertices[p2], vertices[p2 + 1], vertices[p2 + 2]),
                Vec3::new(normals[n0], normals[n0 + 1], normals[n0 + 2]),
                Vec3::new(normals[n1], normals[n1 + 1], normals[n1 + 2]),
                Vec3::new(normals[n2], normals[n2 + 1], normals[n2 + 2]),
                Vec3::homogeneous(255),
            );
            if validate_triangle(&triangle).is_err() {
                failed += 1;
                continue;
            }
            triangles.push(triangle);
        }
        if failed > 0 {
            println!(
                "Failed to parse {} of {} triangles",
                failed,
                triangles.len()
            );
        }
    }
    Ok(triangles)
}
