use std::ops::{Add, AddAssign, Mul, MulAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T> From<(T, T)> for Vector2<T> {
    fn from(pair: (T, T)) -> Self {
        Self {
            x: pair.0,
            y: pair.1,
        }
    }
}

impl Vector2<f64> {
    pub fn magnitude(&self) -> f64 {
        ((self.x * self.x) + (self.y * self.y)).sqrt() 
    }
}

impl<T: Add<Output=T>> Add for Vector2<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Add<Output=T> + Copy> AddAssign for Vector2<T> {
    fn add_assign(&mut self, other: Self) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
    }
}

impl<T: Mul<Output=T> + Copy> Mul<T> for Vector2<T> {
    type Output = Self;

    fn mul(self, scalar: T) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl<T: Mul<Output=T> + Copy> MulAssign<T> for Vector2<T> {
    fn mul_assign(&mut self, other: T) {
        self.x = self.x * other;
        self.y = self.y * other;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ERROR: f64 = 0.0001;

    #[test]
    fn add_i32() {
        let v1 = Vector2 { x: 2, y: 2};
        let v2 = Vector2 { x: 4, y: 4};

        assert_eq!(v1 + v2, Vector2 { x: 6, y: 6 });

        let mut v3 = Vector2 { x: 1, y: 2};
        v3 += v1;

        assert_eq!(v3, Vector2 { x: 3, y: 4 });
    }

    #[test]
    fn add_f64() {
        let v1 = Vector2 { x: 2.0, y: 1.0};
        let v2 = Vector2 { x: 3.0, y: 0.0};

        assert_eq!(v1 + v2, Vector2 { x: 5.0, y: 1.0 });

        let mut v3 = Vector2 { x: 2.0, y: 3.0};
        v3 += v1;

        assert_eq!(v3, Vector2 { x: 4.0, y: 4.0 });
    }

    #[test]
    fn multiply_i32() {
        let mut v1 = Vector2 { x: 2, y: 1};
        assert_eq!(v1 * 5, Vector2 { x: 10, y: 5 });

        v1 *= 3;
        assert_eq!(v1, Vector2 { x: 6, y: 3});
    }

    #[test]
    fn multiply_f64() {
        let mut v1 = Vector2 { x: 3.0, y: 4.0};
        assert_eq!(v1 * 2.0, Vector2 { x: 6.0, y: 8.0 });

        v1 *= 4.0;
        assert_eq!(v1, Vector2 { x: 12.0, y: 16.0});
    }

    #[test]
    fn magnitude() {
        let v1 = Vector2 { x: 1.0, y: 1.0 };
        assert!(v1.magnitude() - 2.0f64.sqrt() < ERROR);
    }
}
