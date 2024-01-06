use crate::num;
use crate::octree::AABB;
use crate::util::{correct_normal, Hit, Ray, Shape};
use crate::vector::Vec3;

#[derive(Debug)]
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

    #[allow(dead_code)]
    fn edges(&self) -> (Vec3<f32>, Vec3<f32>) {
        let edge1 = self.p1 - self.p0;
        let edge2 = self.p2 - self.p0;
        return (edge1, edge2);
    }

    #[allow(dead_code)]
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

    #[allow(dead_code)]
    fn hit_1(&self, ray: &Ray) -> Option<Hit> {
        // compute the plane's normal
        let v0v1 = self.p1 - self.p0;
        let v0v2 = self.p2 - self.p0;
        // no need to normalize
        let normal = v0v1.cross(&v0v2); // N

        // Step 1: finding P
        // check if the ray and plane are parallel.
        let n_dot_ray_direction = normal.dot(&ray.dir);
        if n_dot_ray_direction.abs() < f32::EPSILON {
            return None; // they are parallel, so they don't intersect!
        }

        // compute d parameter using equation 2
        let d = -normal.dot(&self.p0);

        // compute t (equation 3)
        let t = -(normal.dot(&ray.origin) + d) / n_dot_ray_direction;

        // check if the triangle is behind the ray
        if t < 0.0 {
            return None;
        }

        // compute the intersection point using equation 1
        let p = ray.origin + (ray.dir * t);

        // edge 0
        let edge0 = self.p1 - self.p0;
        let vp0 = p - self.p0;
        let mut c = edge0.cross(&vp0); // vector perpendicular to triangle's plane
        if normal.dot(&c) < 0.0 {
            return None;
        } // P is on the right side

        // edge 1
        let edge1 = self.p2 - self.p1;
        let vp1 = p - self.p1;
        c = edge1.cross(&vp1);
        if normal.dot(&c) < 0.0 {
            return None;
        } // P is on the right side

        // edge 2
        let edge2 = self.p0 - self.p2;
        let vp2 = p - self.p2;
        c = edge2.cross(&vp2);
        if normal.dot(&c) < 0.0 {
            return None;
        } // P is on the right side;

        return Some(Hit::new(
            t,
            ray.origin,
            ray.origin * t,
            correct_normal(normal.to_normalized(), &ray.dir),
            self.color,
        ));
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
