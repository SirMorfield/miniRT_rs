use crate::num;
use crate::octree::AABB;
use crate::util::{correct_normal, Hit, Ray, Shape};
use crate::vector::Vec3;

pub struct Triangle {
    pub p0: Vec3<f32>,
    pub p1: Vec3<f32>,
    pub p2: Vec3<f32>,
    pub color: Vec3<u8>,
}

impl Triangle {
    // can this be done better?
    pub fn new(p0: Vec3<f32>, p1: Vec3<f32>, p2: Vec3<f32>, color: Vec3<u8>) -> Self {
        Self { p0, p1, p2, color }
    }

    fn edges(&self) -> (Vec3<f32>, Vec3<f32>) {
        let edge1 = self.p1 - self.p0;
        let edge2 = self.p2 - self.p0;
        return (edge1, edge2);
    }

    fn hit_0(&self, ray: &Ray) -> Option<Hit> {
        // #ifndef USE_EIGEN
        let (edge1, edge2) = self.edges();
        // let normal = edge1.cross(&edge2).normalized();
        // #endif

        let h = ray.dir.cross(&edge2);
        let a = edge1.dot(&h);
        if a > -f32::EPSILON && a < f32::EPSILON {
            return None; // Ray is parallel to this triangle.
        }
        let f = 1.0 / a;
        let s = ray.origin - self.p0;
        let u = f * s.dot(&h);
        if u < 0.0 || u > 1.0 {
            return None;
        }
        let q = s.cross(&edge1);
        let v = f * ray.dir.dot(&q);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }
        let t = f * edge2.dot(&q);
        if t < f32::EPSILON {
            // There is a line intersection but not a ray intersection
            return None;
        }

        let normal = correct_normal(edge1.cross(&edge2).to_normalized(), &ray.dir);
        return Some(Hit::new(t, ray.origin, ray.origin * t, normal, self.color));
    }
}

impl Shape for Triangle {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        return self.hit_0(ray);
    }

    fn is_inside_aabb(&self, aabb: &AABB) -> bool {
        return aabb.is_inside_all(&[self.p0, self.p1, self.p2]);
    }

    fn aabb(&self) -> AABB {
        let min = Vec3::new(
            num::minn(&[self.p0.x, self.p1.x, self.p2.x]),
            num::minn(&[self.p0.y, self.p1.y, self.p2.y]),
            num::minn(&[self.p0.z, self.p1.z, self.p2.z]),
        );
        let max = Vec3::new(
            num::maxn(&[self.p0.x, self.p1.x, self.p2.x]),
            num::maxn(&[self.p0.y, self.p1.y, self.p2.y]),
            num::maxn(&[self.p0.z, self.p1.z, self.p2.z]),
        );
        return AABB::new(min, max);
    }
}
