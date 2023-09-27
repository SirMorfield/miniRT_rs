use crate::num::f32::{max, min};
use crate::util::{Hit, Ray, Shape};
use crate::vector::Vec3;
use std::vec::Vec;

pub struct AABB {
    min: Vec3<f32>,
    max: Vec3<f32>,
}

impl AABB {
    pub fn new(min: Vec3<f32>, max: Vec3<f32>) -> Self {
        Self { min, max }
    }

    // https://gamedev.stackexchange.com/questions/18436
    pub fn hit(&self, r: &Ray) -> bool {
        let dirfrac = Vec3::new(1.0 / r.dir.x, 1.0 / r.dir.y, 1.0 / r.dir.z);
        let t1 = (self.min.x - r.origin.x) * dirfrac.x;
        let t2 = (self.max.x - r.origin.x) * dirfrac.x;
        let t3 = (self.min.y - r.origin.y) * dirfrac.y;

        let t4 = (self.max.y - r.origin.y) * dirfrac.y;
        let t5 = (self.min.z - r.origin.z) * dirfrac.z;
        let t6 = (self.max.z - r.origin.z) * dirfrac.z;

        let tmin = max(max(min(t1, t2), min(t3, t4)), min(t5, t6));
        let tmax = min(min(max(t1, t2), max(t3, t4)), max(t5, t6));

        // let mut t = 0.0;
        if tmax < 0.0 {
            // t = tmax;
            return false;
        }

        // if tmin > tmax, ray doesn't intersect AABB
        if tmin > tmax {
            // t = tmax;
            return false;
        }

        // t = tmin;
        return true;
    }

    pub fn is_inside(&self, point: &Vec3<f32>) -> bool {
        return point.x >= self.min.x
            && point.x <= self.max.x
            && point.y >= self.min.y
            && point.y <= self.max.y
            && point.z >= self.min.z
            && point.z <= self.max.z;
    }
    pub fn is_inside_all(&self, points: &[Vec3<f32>]) -> bool {
        points.iter().all(|point| self.is_inside(point))
    }

    /// subdivide space into  8 children
    pub fn subdivide(&self) -> Vec<AABB> {
        let center = (self.min + self.max) / 2.0;
        let mut children = Vec::with_capacity(8);
        children.push(AABB::new(self.min, center));
        children.push(AABB::new(
            Vec3::new(center.x, self.min.y, self.min.z),
            Vec3::new(self.max.x, center.y, center.z),
        ));
        children.push(AABB::new(
            Vec3::new(center.x, self.min.y, center.z),
            Vec3::new(self.max.x, center.y, self.max.z),
        ));
        children.push(AABB::new(
            Vec3::new(self.min.x, self.min.y, center.z),
            Vec3::new(center.x, center.y, self.max.z),
        ));
        children.push(AABB::new(
            Vec3::new(self.min.x, center.y, self.min.z),
            Vec3::new(center.x, self.max.y, center.z),
        ));
        children.push(AABB::new(
            Vec3::new(center.x, center.y, self.min.z),
            Vec3::new(self.max.x, self.max.y, center.z),
        ));
        children.push(AABB::new(center, self.max));
        children.push(AABB::new(
            Vec3::new(self.min.x, center.y, center.z),
            Vec3::new(center.x, self.max.y, self.max.z),
        ));
        return children;
    }
}

const MAX_SHAPES_PER_OCTREE: usize = 10;

/// Usage:
/// let mut octree = Octree::new();
/// for shape in shapes {
///    octree.push(shape);
/// }
/// octree.shrink_to_fit();
/// octree.subdivide();
///

pub struct Octree<T> {
    aabb: AABB,
    children: Vec<Octree<T>>,
    shapes: Vec<T>,
}

impl<T> Octree<T>
where
    T: Shape,
{
    pub fn new() -> Self {
        Self {
            aabb: AABB::new(
                Vec3::homogeneous(-f32::INFINITY),
                Vec3::homogeneous(f32::INFINITY),
            ),
            children: Vec::new(),
            shapes: Vec::new(),
        }
    }
    pub fn push(&mut self, shape: T) {
        self.shapes.push(shape);
    }

    pub fn shrink_to_fit(&mut self) {
        let mut min = self.aabb.min;
        let mut max = self.aabb.max;

        for shape in &self.shapes {
            let shape_aabb = shape.aabb();

            min.x = min.x.max(shape_aabb.min.x);
            min.y = min.y.max(shape_aabb.min.y);
            min.z = min.z.max(shape_aabb.min.z);

            max.x = max.x.min(shape_aabb.max.x);
            max.y = max.y.min(shape_aabb.max.y);
            max.z = max.z.min(shape_aabb.max.z);
        }

        self.aabb = AABB::new(max, min);
    }

    pub fn subdivide(&mut self) {
        if self.children.len() == 0 {
            self.children = self
                .aabb
                .subdivide()
                .into_iter()
                .map(|aabb| Octree::new_sized(aabb))
                .collect();
        }

        let mut shapes = Vec::new();
        std::mem::swap(&mut self.shapes, &mut shapes);
        for shape in shapes {
            self.insert(shape).unwrap();
        }
    }

    pub fn hit(&self, ray: &Ray) -> Option<Hit> {
        if !self.aabb.hit(ray) {
            return None;
        }
        let mut closest: Option<Hit> = None;

        for shape in &self.shapes {
            if !shape.hit(ray) {
                continue;
            }
            let hit = shape.hit_info(ray);

            closest = match closest {
                None => Some(hit),
                Some(mut closest) => {
                    closest.replace_if_closer(hit);
                    Some(closest)
                }
            }
        }

        for child in &self.children {
            let hit = child.hit(ray);
            if hit.is_none() {
                continue;
            }
            closest = match closest {
                None => hit,
                Some(mut closest) => {
                    closest.replace_if_closer(hit.unwrap());
                    Some(closest)
                }
            }
        }

        return closest;
    }

    fn new_sized(aabb: AABB) -> Self {
        Self {
            aabb,
            children: Vec::new(),
            shapes: Vec::new(),
        }
    }

    fn can_insert(&self, shape: &T) -> bool {
        return shape.is_inside_aabb(&self.aabb);
    }

    fn insert_unsafe(&mut self, shape: T) {
        if self.shapes.len() + 1 < MAX_SHAPES_PER_OCTREE {
            self.shapes.push(shape);
            return;
        }

        if self.children.len() == 0 {
            self.subdivide();
        }
        for child in &mut self.children {
            if child.can_insert(&shape) {
                child.insert_unsafe(shape);
                return;
            }
        }
        self.shapes.push(shape);
    }

    fn insert(&mut self, shape: T) -> Result<(), &'static str> {
        // if !self.can_insert(&shape) { // TODO enable this
        //     return Err("Shape is not inside AABB");
        // }
        self.insert_unsafe(shape);
        return Ok(());
    }
}
