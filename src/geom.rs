use super::ray;
use super::vec::Vec3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct HitRecord {
    pub t: f32,
    pub intersection: Vec3,
    pub normal: Vec3,
}

pub trait Hittable {
    fn hit(&self, r: &ray::Ray, t_max: f32) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, r: &ray::Ray, t_max: f32) -> Option<HitRecord> {
        // The point P given by (x,y,z) is on a sphere with radius R
        // and center C given by (c_x, c_y, c_z)
        //   (x - c_x)^2 + (y - c_y)^2 + (z - c_z)^2 = R^2
        // In vector form:
        //   (p - c) * (p-c) = R^2
        // Our ray is described by p(t)
        // We can substitue in p(t):
        //  (p(t) - c) * (p(t) - c) = R^2
        //  => (a + tb - c) * (a + tb - c) - R^2 = 0
        //  => [b*b]t^2 + [2b*(a-c)]t +[(a-c)*(a-c)-R^2] = 0
        // The discrimant is [2b*(a-c)]-4[(b*b)((a-c)*(a-c)-R^2)]
        // If the discrimant is less than 0, there are no solutions. The ray
        // does not intersect the sphere.
        // If the discriminant is greater 0, there are 2 solutions
        // If the discriminant is zero, there is one solution

        let oc = r.origin - self.center;
        let a = r.direction.dot(r.direction);
        let b = 2.0 * r.direction.dot(oc);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discrimant = b * b - 4.0 * a * c;

        if discrimant <= 0.0 {
            None
        } else {
            let t = (-b - discrimant.sqrt()) / (2.0 * a);
            if t > 0.0 && t < t_max {
                let intersection = r.point_at_parameter(t);
                let normal = (intersection - self.center).unit();
                Some(HitRecord {
                    t,
                    intersection,
                    normal,
                })
            } else {
                None
            }
        }
    }
}
