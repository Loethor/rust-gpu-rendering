use std::ops::{Add, Sub, Mul, Div, Neg};


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// Useful aliases to make the ray tracing
pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    /// Creates a new 3D vector.
    #[inline]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Creates a zero vector (0, 0, 0).
    #[inline]
    pub const fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    /// Creates a one vector (1, 1, 1).
    #[inline]
    pub const fn one() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }

    /// Returns the squared length of the vector.
    #[inline]
    pub fn length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Returns the actual length (magnitude) of the vector.
    #[inline]
    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    /// Returns a normalized (unit) vector pointing in the same direction.
    #[inline]
    pub fn normalized(self) -> Self {
        self / self.length()
    }

    /// Dot product.
    #[inline]
    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Cross product.
    #[inline]
    pub fn cross(self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
    
    /// Checks if the vector is near zero (useful for avoiding division by zero).
    #[inline]
    pub fn near_zero(self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    /// Returns the reflection of this vector bouncing off a surface with the given normal.
    /// Assumes `normal` is normalized.
    #[inline]
    pub fn reflect(self, normal: Vec3) -> Vec3 {
        self - 2.0 * self.dot(normal) * normal
    }
}


impl Add for Vec3 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vec3 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Neg for Vec3 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}

// Vector * Vector (Component-wise multiplication, useful for Colors)
impl Mul for Vec3 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

// Vector * Scalar
impl Mul<f32> for Vec3 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f32) -> Self {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

// Scalar * Vector
impl Mul<Vec3> for f32 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: Vec3) -> Vec3 {
        rhs * self
    }
}

// Vector / Scalar
impl Div<f32> for Vec3 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: f32) -> Self {
        self * (1.0 / rhs)
    }
}


/// A ray defined by an origin point and a direction vector.
/// Equation: P(t) = origin + t * direction
#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    #[inline]
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    /// Calculates the position along the ray at parameter `t`.
    #[inline]
    pub fn at(&self, t: f32) -> Point3 {
        self.origin + t * self.direction
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_basics() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        
        assert_eq!(v1 + v2, Vec3::new(5.0, 7.0, 9.0));
        assert_eq!(v1 * 2.0, Vec3::new(2.0, 4.0, 6.0));
        assert_eq!(2.0 * v1, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_dot_and_cross() {
        let v1 = Vec3::new(1.0, 0.0, 0.0);
        let v2 = Vec3::new(0.0, 1.0, 0.0);
        
        assert_eq!(v1.dot(v2), 0.0);
        assert_eq!(v1.cross(v2), Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_ray_at() {
        let r = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(r.at(2.0), Point3::new(2.0, 4.0, 6.0));
    }

        #[test]
    fn test_reflect_straight_on() {
        // A ray going straight down (-Y) hitting a floor with normal (+Y)
        // should bounce straight back up (+Y).
        let v = Vec3::new(0.0, -1.0, 0.0);
        let n = Vec3::new(0.0, 1.0, 0.0);
        assert_eq!(v.reflect(n), Vec3::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn test_reflect_45_degrees() {
        // A ray coming down and right (1, -1) hitting a flat floor (0, 1)
        // should bounce up and right (1, 1).
        let v = Vec3::new(1.0, -1.0, 0.0);
        let n = Vec3::new(0.0, 1.0, 0.0);
        assert_eq!(v.reflect(n), Vec3::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn test_reflect_arbitrary_angle() {
        // Ray: (2, -1, 0). Normal: (0, 1, 0).
        // V·N = -1. R = (2,-1,0) - 2*(-1)*(0,1,0) = (2,-1,0) + (0,2,0) = (2,1,0).
        let v = Vec3::new(2.0, -1.0, 0.0);
        let n = Vec3::new(0.0, 1.0, 0.0);
        assert_eq!(v.reflect(n), Vec3::new(2.0, 1.0, 0.0));
    }
}