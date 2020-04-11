use crate::vec;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    pub origin: vec::Vec3,
    pub direction: vec::Vec3,
}

impl Ray {
    pub fn new(origin: vec::Vec3, direction: vec::Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn point_at_parameter(&self, t: f32) -> vec::Vec3 {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod tests {
    use super::vec;
    use super::Ray;
    #[test]
    fn test_point_at_parameter0() {
        let v1 = vec::Vec3::new(0.0, 0.0, 0.0);
        let v2 = vec::Vec3::new(1.0, 2.0, 3.0);
        let r1 = Ray::new(v1, v2);

        assert_eq!(r1.point_at_parameter(0.0), v1);
        assert_eq!(r1.point_at_parameter(0.5), v2 * 0.5);
        assert_eq!(r1.point_at_parameter(1.0), v2);
    }
}
