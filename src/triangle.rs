use crate::{
    util::{Hit, Ray},
    vector::Vec3,
};

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

    pub fn hit(&self, ray: &Ray) -> Hit {
        // #ifndef USE_EIGEN
        let edge1 = self.p1 - self.p0;
        let edge2 = self.p2 - self.p0;
        // let normal = edge1.cross(&edge2).normalized();
        // #endif

        let h = ray.dir.cross(&edge2);
        let a = edge1.dot(&h);
        if a > -f32::EPSILON && a < f32::EPSILON {
            return Hit::new(false); // Ray is parallel to this triangle.
        }
        let f = 1.0 / a;
        let s = ray.origin - self.p0;
        let u = f * s.dot(&h);
        if u < 0.0 || u > 1.0 {
            return Hit::new(false);
        }
        let q = s.cross(&edge1);
        let v = f * ray.dir.dot(&q);
        if v < 0.0 || u + v > 1.0 {
            return Hit::new(false);
        }
        let t = f * edge2.dot(&q);
        if t < f32::EPSILON {
            // There is a line intersection but not a ray intersection
            return Hit::new(false);
        }

        // 	Hit hit(true);
        // hit.dist = (ray.dir * t).length();
        // hit.point = ray.origin * hit.dist;
        // hit.normal = correct_normal(normal, ray);
        // hit.color = color;
        return Hit::new(true);
    }
}
