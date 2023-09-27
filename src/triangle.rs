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
}

impl Shape for Triangle {
    fn t(&self, ray: &Ray) -> f32 {
        // #ifndef USE_EIGEN
        let (edge1, edge2) = self.edges();
        // let normal = edge1.cross(&edge2).normalized();
        // #endif

        let h = ray.dir.cross(&edge2);
        let a = edge1.dot(&h);
        if a > -f32::EPSILON && a < f32::EPSILON {
            return 0.0; // Ray is parallel to this triangle.
        }
        let f = 1.0 / a;
        let s = ray.origin - self.p0;
        let u = f * s.dot(&h);
        if u < 0.0 || u > 1.0 {
            return 0.0;
        }
        let q = s.cross(&edge1);
        let v = f * ray.dir.dot(&q);
        if v < 0.0 || u + v > 1.0 {
            return 0.0;
        }
        let t = f * edge2.dot(&q);
        // if t < f32::EPSILON {
        //     // There is a line intersection but not a ray intersection
        //     return 0.0;
        // }

        return t;
    }

    fn hit(&self, ray: &Ray) -> bool {
        self.t(ray) > f32::EPSILON
    }

    fn hit_info(&self, ray: &Ray) -> Hit {
        let t = self.t(ray);
        let dist = (ray.dir * t).length(); // TODO
        let (edge1, edge2) = self.edges();
        let normal = correct_normal(edge1.cross(&edge2).to_normalized(), &ray.dir);

        return Hit::new(dist, ray.origin, ray.origin * dist, normal, self.color);
    }

    fn is_inside_aabb(&self, aabb: &AABB) -> bool {
        return aabb.is_inside_all(&[self.p0, self.p1, self.p2]);
    }

    fn aabb(&self) -> AABB {
        let min = Vec3::new(
            self.p0.x.min(self.p1.x.min(self.p2.x)),
            self.p0.y.min(self.p1.y.min(self.p2.y)),
            self.p0.z.min(self.p1.z.min(self.p2.z)),
        );
        let max = Vec3::new(
            self.p0.x.max(self.p1.x.max(self.p2.x)),
            self.p0.y.max(self.p1.y.max(self.p2.y)),
            self.p0.z.max(self.p1.z.max(self.p2.z)),
        );
        return AABB::new(min, max);
    }
}
