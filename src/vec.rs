use std::ops;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    d: [f32; 3],
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { d: [x, y, z] }
    }

    pub fn x(self) -> f32 {
        self.d[0]
    }

    pub fn y(self) -> f32 {
        self.d[1]
    }

    pub fn z(self) -> f32 {
        self.d[2]
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            d: [
                self.d[0] + rhs.d[0],
                self.d[1] + rhs.d[1],
                self.d[2] + rhs.d[2],
            ],
        }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 {
            d: [self.d[0] * rhs, self.d[1] * rhs, self.d[2] * rhs],
        }
    }
}

#[cfg(test)]
mod tests {
    pub use super::*;
    mod add {
        use super::Vec3;
        #[test]
        fn test_identity() {
            let v1 = Vec3::new(1.0, 2.0, 3.0);
            let identity = Vec3::new(0.0, 0.0, 0.0);

            assert_eq!(v1 + identity, v1);
            assert_eq!(identity + v1, v1);
            assert_eq!(identity + identity, identity)
        }

        #[test]
        fn test_double() {
            let v1 = Vec3::new(1.0, 2.0, 3.0);
            let v2 = Vec3::new(2.0, 4.0, 6.0);

            assert_eq!(v1 + v1, v2)
        }
    }

    mod multiply {
        use super::Vec3;
        #[test]
        fn test_zero() {
            let v1 = Vec3::new(1.0, 2.0, 3.0);
            let zero = Vec3::new(0.0, 0.0, 0.0);

            assert_eq!(v1 * 0.0, zero);
        }

        #[test]
        fn test_identity() {
            let v1 = Vec3::new(1.0, 2.0, 3.0);
            let identity = 1.0;

            assert_eq!(v1 * identity, v1);
        }

        #[test]
        fn test_scale() {
            let v1 = Vec3::new(1.0, 2.0, 3.0);
            let v2 = Vec3::new(2.0, 4.0, 6.0);

            assert_eq!(v1 * 2.0, v2);
        }

        #[test]
        fn test_multiply_add() {
            let v1 = Vec3::new(1.0, 2.0, 3.0);
            let zero = Vec3::new(0.0, 0.0, 0.0);

            assert_eq!(v1 + v1 * -1.0, zero);
            assert_eq!(v1 + v1 * -2.0, v1 * -1.0);
        }
    }
}
