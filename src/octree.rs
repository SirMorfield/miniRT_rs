use crate::num::f32::{max, min};
use crate::util::{Hit, Ray, Shape};
use crate::vector::Vec3;
use std::vec::Vec;

#[derive(Debug)]
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

#[cfg(test)]
mod aabb_test {
    use crate::octree::AABB;
    use crate::vector::Vec3;

    #[test]
    fn real() {
        let aabb = AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));
        assert_eq!(aabb.is_inside(&Vec3::new(0.5, 0.5, 0.5)), true);
        assert_eq!(aabb.is_inside(&Vec3::new(0.0, 0.0, 0.0)), true);
        assert_eq!(aabb.is_inside(&Vec3::new(1.0, 1.0, 1.0)), true);
        assert_eq!(aabb.is_inside(&Vec3::new(0.0, 0.0, 1.0)), true);

        assert_eq!(aabb.is_inside(&Vec3::new(0.0, 0.0, 1.1)), false);
        assert_eq!(aabb.is_inside(&Vec3::new(0.0, 0.0, -0.1)), false);
    }
    #[test]
    fn infinity() {
        let aabb = AABB::new(
            Vec3::homogeneous(-f32::INFINITY),
            Vec3::homogeneous(f32::INFINITY),
        );
        assert_eq!(aabb.is_inside(&Vec3::new(0.0, 0.0, 0.0)), true);
        assert_eq!(aabb.is_inside(&Vec3::homogeneous(f32::MAX)), true);
        assert_eq!(aabb.is_inside(&Vec3::homogeneous(f32::MIN)), true);
        assert_eq!(aabb.is_inside(&Vec3::homogeneous(f32::INFINITY)), true);
        assert_eq!(aabb.is_inside(&Vec3::homogeneous(-f32::INFINITY)), true);
    }
}

// This number was chosen by a dice
const MAX_SHAPES_PER_OCTREE: usize = 10;

/// Usage:
/// ```
/// let shapes = vec![Triangle::new(
///    Vec3::new(0.0, 0.0, 0.0),
///    Vec3::new(1.0, 0.0, 0.0),
///    Vec3::new(0.0, 1.0, 0.0),
///    Vec3::new(255, 0, 0),
/// )];
/// let octree = Octree::new(shapes);
/// assert_eq!(octree.shapes_count(), 1);
///```

pub struct Octree<T> {
    aabb: AABB,
    children: Vec<Octree<T>>,
    shapes: Vec<T>,
    pub shapes_count: usize,
}

impl<T> Octree<T>
where
    T: Shape,
{
    pub fn new(shapes: Vec<T>) -> Self {
        let mut this = Self {
            aabb: AABB::new(Vec3::homogeneous(f32::MIN), Vec3::homogeneous(f32::MAX)),
            children: Vec::new(),
            shapes_count: shapes.len(),
            shapes,
        };
        if this.shapes.len() != 0 {
            this.shrink_to_fit();
            this.subdivide();
            this.shapes.shrink_to(MAX_SHAPES_PER_OCTREE);
        }
        for shape in &this.shapes {
            assert!(shape.is_inside_aabb(&this.aabb));
            // println!("shape: {:?}", shape.aabb());
        }
        return this;
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

    #[allow(dead_code)]
    pub fn shapes_count(&self) -> usize {
        return self.shapes_count;
    }

    fn new_sized(aabb: AABB) -> Self {
        Self {
            aabb,
            children: Vec::new(),
            shapes: Vec::new(),
            shapes_count: 0,
        }
    }

    fn shrink_to_fit(&mut self) {
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

    fn subdivide(&mut self) {
        if self.children.len() == 0 {
            self.children = self
                .aabb
                .subdivide()
                .into_iter()
                .map(|aabb| Octree::new_sized(aabb))
                .collect();
        }

        let shapes_count = self.shapes.len();
        let mut inserted = 0;

        let mut shapes = Vec::with_capacity(MAX_SHAPES_PER_OCTREE);
        std::mem::swap(&mut self.shapes, &mut shapes);

        for shape in shapes {
            if self.can_insert(&shape) {
                self.insert_unsafe(shape);
                inserted += 1;
            }
        }
        let percentage = (inserted as f32 / shapes_count as f32) * 100.0;
        if percentage < 90.0 {
            panic!("Inserted {inserted} out of {shapes_count} shapes ({percentage}");
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
}
