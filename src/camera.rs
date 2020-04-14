use super::ray;
use super::vec;

pub trait Camera {
    fn get_ray(&self, u: f32, v: f32) -> ray::Ray;
}

pub struct Simple {
    origin: vec::Vec3,
    lower_left_corner: vec::Vec3,
    horizontal: vec::Vec3,
    vertical: vec::Vec3,
}

impl Simple {
    pub fn new(origin: vec::Vec3, width: f32, height: f32, depth: f32) -> Simple {
        let horizontal = vec::Vec3::new(width, 0.0, 0.0);
        let vertical = vec::Vec3::new(0.0, height, 0.0);
        let back = vec::Vec3::new(0.0, 0.0, depth);
        let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) + back;
        return Simple {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        };
    }
}

impl Camera for Simple {
    fn get_ray(&self, u: f32, v: f32) -> ray::Ray {
        return ray::Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v,
        );
    }
}
