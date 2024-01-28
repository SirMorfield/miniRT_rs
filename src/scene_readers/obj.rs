use super::{default_ambient, look_at, Scene};
use crate::helpers::contains_duplicates;
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

fn vertex_normals_loaded(mesh: &tobj::Mesh) -> bool {
    mesh.normals.len() != 0 && mesh.normal_indices.len() != 0
}

fn validate_mesh(mesh: &tobj::Mesh) -> Result<(), String> {
    let vertices = &mesh.positions;
    let vertices_i = &mesh.indices;
    let normals = &mesh.normals;
    let normals_i = &mesh.normal_indices;

    if vertices_i.len() % 3 != 0 {
        return Err("Indices must be a multiple of 3".into());
    }
    if vertices.iter().find(|n| !n.is_finite()).is_some() {
        return Err("Vertex is not finite".into());
    }
    if vertex_normals_loaded(mesh) {
        if vertices_i.len() != normals_i.len() {
            return Err("Indices and normals must be the same length".into());
        }
        if normals.iter().find(|n| !n.is_finite()).is_some() {
            return Err("Normal is not finite".into());
        }
    }
    Ok(())
}

///  either the points of a triangle or the vertex normals of one
fn load_tri_vector(
    points: &Vec<f32>,
    idx: &Vec<u32>,
    i: usize,
) -> (Vec3<f32>, Vec3<f32>, Vec3<f32>) {
    let p0 = idx[i + 0] as usize * 3;
    let p1 = idx[i + 1] as usize * 3;
    let p2 = idx[i + 2] as usize * 3;
    let p0 = Vec3::new(points[p0], points[p0 + 1], points[p0 + 2]);
    let p1 = Vec3::new(points[p1], points[p1 + 1], points[p1 + 2]);
    let p2 = Vec3::new(points[p2], points[p2 + 1], points[p2 + 2]);
    (p0, p1, p2)
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

        if !vertex_normals_loaded(&m.mesh) {
            println!("No vertex normals found. Using geometric normals instead.");
        }

        for i in (0..vertices_i.len()).step_by(3) {
            let (p0, p1, p2) = load_tri_vector(vertices, vertices_i, i);
            if contains_duplicates(&[p0, p1, p2]) {
                failed += 1;
                continue;
            }
            let color = Vec3::homogeneous(255);
            let triangle = match vertex_normals_loaded(&m.mesh) {
                true => {
                    let (n0, n1, n2) = load_tri_vector(normals, normals_i, i);
                    triangle::Triangle::with_vertex_normals(p0, p1, p2, n0, n1, n2, color)
                }
                false => triangle::Triangle::new(p0, p1, p2, Vec3::homogeneous(255)),
            };
            triangles.push(triangle);
        }
        if failed > 0 {
            println!(
                "Failed to parse {} of {} triangles due to duplicate vertices",
                failed,
                triangles.len()
            );
        }
    }
    Ok(triangles)
}
