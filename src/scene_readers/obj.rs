use super::{default_ambient, look_at, FileType, Scene};
use crate::helpers::contains_duplicates;
use crate::light::Light;
use crate::num::Float0to1;
use crate::octree::Octree;
use crate::triangle;
use crate::triangle::Triangle;
use crate::vector::Point;
use image::{DynamicImage, GenericImageView, Pixel};
use std::path::{Path, PathBuf};
use tobj;

pub fn read_obj(path: &Path) -> Result<Scene, String> {
    let now = std::time::Instant::now();
    if path.extension().unwrap() != "obj" {
        return Err("File must end with .obj".into());
    }
    let mut opt = tobj::GPU_LOAD_OPTIONS;
    opt.single_index = false;

    let obj = tobj::load_obj(path, &opt);
    let (models, _) = obj.expect("Failed to load OBJ file");
    let texture = match get_texture(path) {
        Some((path, texture)) => {
            println!("Texture found: {}", path.display());
            Some(texture)
        }
        None => None,
    };
    let triangles = parse_triangle(models, &texture)?;
    if triangles.len() == 0 {
        return Err("No triangles found".into());
    }

    let camera = look_at(&triangles);
    let mut lights: Vec<Light> = vec![];

    lights.push(Light::new(
        camera.pos,
        Float0to1::new(0.5).unwrap(),
        Point::new(255, 255, 255),
    ));
    let triangles = Octree::new(triangles);
    let parse_duration = now.elapsed();
    return Ok(Scene::new(
        camera,
        triangles,
        lights,
        default_ambient(),
        parse_duration,
        FileType::Obj,
    ));
}

fn get_texture(obj_path: &Path) -> Option<(PathBuf, DynamicImage)> {
    let mtl_path = obj_path.with_extension("mtl");
    let mtl = tobj::load_mtl(mtl_path);
    if let Err(_) = mtl {
        return None;
    }
    let mut texture_path: Option<PathBuf> = None;

    for mat in mtl.unwrap().0.iter() {
        if let Some(ref path) = mat.diffuse_texture {
            texture_path = Some(obj_path.with_file_name(path));
        }
    }

    match texture_path {
        Some(path) => {
            let texture = image::open(&path).expect("Failed to open texture image");
            return Some((path, texture));
        }
        None => None,
    }
}

struct MeshInfo {
    pub has_vertex_normals: bool,
    pub has_texture_coords: bool,
}

fn validate_mesh(mesh: &tobj::Mesh, texture: &Option<DynamicImage>) -> Result<MeshInfo, String> {
    let vertices = &mesh.positions;
    let vertices_i = &mesh.indices;
    let normals = &mesh.normals;
    let normals_i = &mesh.normal_indices;
    let texture_coords = &mesh.texcoords;
    let texture_coords_i = &mesh.texcoord_indices;
    let mut info = MeshInfo {
        has_vertex_normals: false,
        has_texture_coords: false,
    };
    if vertices_i.len() % 3 != 0 {
        return Err("Indices must be a multiple of 3".into());
    }
    if vertices.iter().find(|n| !n.is_finite()).is_some() {
        return Err("Vertex is not finite".into());
    }
    if mesh.normals.len() != 0 && mesh.normal_indices.len() != 0 {
        info.has_vertex_normals = true;
        if vertices_i.len() != normals_i.len() {
            return Err("Indices and normals must be the same length".into());
        }
        if normals.iter().find(|n| !n.is_finite()).is_some() {
            return Err("Normal is not finite".into());
        }
    }
    if texture_coords_i.len() != 0 {
        info.has_texture_coords = true;
    }
    if texture_coords_i.len() != 0 && texture.is_none() {
        println!("WARN: Texture coordinates found but no texture");
        info.has_texture_coords = false;
    }
    if texture_coords_i.len() != 0 && texture_coords_i.len() != vertices_i.len() {
        return Err("Indices and texture coordinates must be the same length".into());
    }
    if texture_coords.iter().find(|n| !n.is_finite()).is_some() {
        return Err("Texture coordinate is not finite".into());
    }
    Ok(info)
}

// either the points of a triangle or the vertex normals of one
fn load_tri_vector(points: &Vec<f32>, indices: &Vec<u32>, i: usize) -> (Point<f32>, Point<f32>, Point<f32>) {
    let p0 = indices[i + 0] as usize * 3;
    let p1 = indices[i + 1] as usize * 3;
    let p2 = indices[i + 2] as usize * 3;
    let p0 = Point::new(points[p0], points[p0 + 1], points[p0 + 2]);
    let p1 = Point::new(points[p1], points[p1 + 1], points[p1 + 2]);
    let p2 = Point::new(points[p2], points[p2 + 1], points[p2 + 2]);
    (p0, p1, p2)
}

fn get_texture_coordinate(model: &tobj::Model, vertex_i: usize, texture_w: u32, texture_h: u32) -> Option<(u32, u32)> {
    let texture_coords = &model.mesh.texcoords;
    let texture_coords_i = &model.mesh.texcoord_indices;
    let i = texture_coords_i[vertex_i] as usize;
    if i * 2 + 1 >= texture_coords.len() {
        return None;
    }
    let u = texture_coords[i * 2];
    let v = texture_coords[i * 2 + 1];
    let x = (u * texture_w as f32) as u32;
    let y = ((1.0 - v) * texture_h as f32) as u32;
    Some((x, y))
}

fn parse_triangle(models: Vec<tobj::Model>, texture: &Option<DynamicImage>) -> Result<Vec<Triangle>, String> {
    let mut triangles: Vec<Triangle> = Vec::new();

    for (_, m) in models.iter().enumerate() {
        let vertices = &m.mesh.positions;
        let vertices_i = &m.mesh.indices;
        let normals = &m.mesh.normals;
        let normals_i = &m.mesh.normal_indices;
        let info = validate_mesh(&m.mesh, texture)?;
        let mut failed: usize = 0;

        if !info.has_vertex_normals {
            println!("No vertex normals found. Using geometric normals instead.");
        }

        for i in (0..vertices_i.len()).step_by(3) {
            let (p0, p1, p2) = load_tri_vector(vertices, vertices_i, i);
            if contains_duplicates(&[p0, p1, p2]) {
                failed += 1;
                continue;
            }
            // TODO, average the texture coordinates, not just use the first one
            let color = match info.has_texture_coords {
                true => {
                    let (w, h) = texture.as_ref().unwrap().dimensions();
                    let (x, y) = get_texture_coordinate(&m, i, w, h).unwrap();
                    let color = texture.as_ref().unwrap().get_pixel(x, y).to_rgb();
                    Point::new(color[0], color[1], color[2])
                }
                false => Point::new(255, 0, 0),
            };
            let triangle = match info.has_vertex_normals {
                true => {
                    let (n0, n1, n2) = load_tri_vector(normals, normals_i, i);
                    triangle::Triangle::with_vertex_normals(p0, p1, p2, n0, n1, n2, color)
                }
                false => triangle::Triangle::new(p0, p1, p2, color),
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
