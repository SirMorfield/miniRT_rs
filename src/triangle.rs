use crate::num::{self, f32};
use crate::octree::AABB;
use crate::util::{correct_normal, Hit, Ray, Shape};
use crate::vector::Point;

#[derive(Debug)]
pub struct Triangle {
    pub p0: Point<f32>,
    pub p1: Point<f32>,
    pub p2: Point<f32>,

    pub vertex_normals: bool,
    pub n0: Point<f32>,
    pub n1: Point<f32>,
    pub n2: Point<f32>,

    pub color: Point<u8>,
}

impl Triangle {
    // can this be done better?
    pub fn new(p0: Point<f32>, p1: Point<f32>, p2: Point<f32>, color: Point<u8>) -> Self {
        let (p0, p1, p2) = make_points_unique(&mut p0.clone(), &mut p1.clone(), &mut p2.clone());
        Self {
            p0,
            p1,
            p2,
            color,
            vertex_normals: false,
            n0: Point::homogeneous(0.0),
            n1: Point::homogeneous(0.0),
            n2: Point::homogeneous(0.0),
        }
    }

    pub fn with_vertex_normals(
        p0: Point<f32>,
        p1: Point<f32>,
        p2: Point<f32>,
        n0: Point<f32>,
        n1: Point<f32>,
        n2: Point<f32>,
        color: Point<u8>,
    ) -> Self {
        let (p0, p1, p2) = make_points_unique(&mut p0.clone(), &mut p1.clone(), &mut p2.clone());
        Self {
            p0,
            p1,
            p2,
            color,
            vertex_normals: true,
            n0,
            n1,
            n2,
        }
    }

    #[allow(dead_code)]
    fn edges(&self) -> (Point<f32>, Point<f32>) {
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

        let normal = match self.vertex_normals {
            true => self.vertex_normal(ray.origin + ray.dir * t, 1.0),
            false => self.normal(),
        };

        let normal = correct_normal(normal, &ray.dir);
        return Some(Hit::new(t, ray.origin, ray.origin * t, normal, self.color));
    }

    fn barycentric_coordinates(&self, point: Point<f32>) -> (f32, f32) {
        // Calculate vectors from p0 to p1 and p0 to p2
        let v0 = self.p1 - self.p0;
        let v1 = self.p2 - self.p0;

        // Calculate vectors from p0 to the given point
        let v2 = point - self.p0;

        // Calculate dot products and denominator
        let dot00 = v0.dot(&v0);
        let dot01 = v0.dot(&v1);
        let dot02 = v0.dot(&v2);
        let dot11 = v1.dot(&v1);
        let dot12 = v1.dot(&v2);

        // Calculate barycentric coordinates
        let inv_denom = 1.0 / (dot00 * dot11 - dot01 * dot01);
        let u = (dot11 * dot02 - dot01 * dot12) * inv_denom;
        let v = (dot00 * dot12 - dot01 * dot02) * inv_denom;

        (u, v)
    }

    #[allow(dead_code)]
    pub fn compare(&self) {
        let normal = self.normal();
        let vertex_normal = self.vertex_normal(self.p0, 1.0);
        println!("points:        {:?}", (self.p0, self.p1, self.p2));
        println!("normal:        {:?}", normal);
        println!("vertex_normal: {:?}", vertex_normal);
        println!();
    }

    fn normal(&self) -> Point<f32> {
        let (edge1, edge2) = self.edges();
        edge1.cross(&edge2).to_normalized()
    }

    fn vertex_normal(&self, point: Point<f32>, intensity: f32) -> Point<f32> {
        let (u, v) = self.barycentric_coordinates(point);

        // Interpolate normals at vertices
        let n0 = if self.vertex_normals {
            self.n0
        } else {
            self.p0
        };
        let n1 = if self.vertex_normals {
            self.n1
        } else {
            self.p1
        };
        let n2 = if self.vertex_normals {
            self.n2
        } else {
            self.p2
        };

        // Barycentric interpolation of normals
        let normal = n0 * (1.0 - u - v) + n1 * u + n2 * v;
        let normal = normal * intensity;
        let normal = normal.to_normalized();
        normal
    }
}

/// Make sure that the points are unique.
/// This is needed for calculating the normal.
/// If two points are the same, the normal calculation will return NaN.
fn make_points_unique(
    p0: &mut Point<f32>,
    p1: &mut Point<f32>,
    p2: &mut Point<f32>,
) -> (Point<f32>, Point<f32>, Point<f32>) {
    if p0 == p1 {
        *p0 += f32::EPSILON * 10.0;
    }

    if p1 == p2 {
        *p1 -= f32::EPSILON * 10.0;
    }

    if p2 == p0 {
        *p2 += f32::EPSILON * 10.0;
    }
    return (p0.clone(), p1.clone(), p2.clone());
}

impl Shape for Triangle {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        return self.hit_0(ray);
    }

    fn is_inside_aabb(&self, aabb: &AABB) -> bool {
        return aabb.is_inside_all(&[self.p0, self.p1, self.p2]);
    }

    fn aabb(&self) -> AABB {
        let min = Point::new(
            num::minn(&[self.p0.x, self.p1.x, self.p2.x]),
            num::minn(&[self.p0.y, self.p1.y, self.p2.y]),
            num::minn(&[self.p0.z, self.p1.z, self.p2.z]),
        );
        let max = Point::new(
            num::maxn(&[self.p0.x, self.p1.x, self.p2.x]),
            num::maxn(&[self.p0.y, self.p1.y, self.p2.y]),
            num::maxn(&[self.p0.z, self.p1.z, self.p2.z]),
        );
        return AABB::new(min, max);
    }
}

#[cfg(test)]
mod tests {
    use crate::triangle::Triangle;
    use crate::vector::Point;

    #[test]
    fn test_normal() {
        let t = Triangle::new(
            Point::new(-8.350787, 547.8047, -204.87953),
            Point::new(19.802517, 662.56616, -351.3024),
            Point::new(19.802517, 662.56616, -351.3024),
            Point::new(0, 0, 0),
        );
        let normal = t.normal();
        assert!(normal.x.is_finite());
        assert!(normal.y.is_finite());
        assert!(normal.z.is_finite());
    }

    #[test]
    fn test_make_unique() {
        let mut p0 = Point::new(0.1, 0.0, 0.0);
        let mut p1 = Point::new(0.0, 0.0, 0.0);
        let mut p2 = Point::new(0.0, 0.0, 0.0);

        let (p0, p1, p2) = super::make_points_unique(&mut p0, &mut p1, &mut p2);
        assert!(p0 != p1);
        assert!(p0 != p2);
        assert!(p1 != p2);
    }
}
