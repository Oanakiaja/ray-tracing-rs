use std::ops::{Add, Div, Index, Mul, Sub};

#[derive(Debug, Clone, PartialEq)]
pub struct Vec3 {
    pub value: Vec<f32>,
}

pub type Color = Vec3;
pub type Point3 = Vec3;

impl Default for Vec3 {
    fn default() -> Self {
        Vec3 {
            value: vec![0.0, 0.0, 0.0],
        }
    }
}

impl Vec3 {
    pub fn new() -> Vec3 {
        Default::default()
    }

    pub fn from(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 {
            value: vec![x, y, z],
        }
    }

    pub fn x(&self) -> f32 {
        *self.value.index(0)
    }

    pub fn y(&self) -> f32 {
        *self.value.index(1)
    }

    pub fn z(&self) -> f32 {
        *self.value.index(2)
    }

    pub fn len(&self) -> f32 {
        return f32::sqrt(self.x() * self.x() + self.y() * self.y() + self.z() * self.z());
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        dot(self, other)
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        cross(self, other)
    }

    pub fn normalize(&self) -> Vec3 {
        let len = self.len();
        self.clone() / len
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            value: vec![
                self.x() + other.x(),
                self.y() + other.y(),
                self.z() + other.z(),
            ],
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            value: vec![
                self.x() - other.x(),
                self.y() - other.y(),
                self.z() - other.z(),
            ],
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, t: f32) -> Vec3 {
        Vec3 {
            value: vec![self.x() * t, self.y() * t, self.z() * t],
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, t: Vec3) -> Vec3 {
        Vec3 {
            value: vec![t.x() * self, t.y() * self, t.z() * self],
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, t: f32) -> Vec3 {
        self * (1.0 / t)
    }
}

pub fn dot(a: &Vec3, b: &Vec3) -> f32 {
    a.x() * b.x() + a.y() * b.y() + a.z() * b.z()
}

pub fn cross(a: &Vec3, b: &Vec3) -> Vec3 {
    Vec3 {
        value: vec![
            a.y() * b.z() - a.z() * b.y(),
            a.z() * b.x() - a.x() * b.z(),
            a.x() * b.y() - a.y() * b.x(),
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let default: Vec3 = Default::default();
        assert_eq!(
            default,
            Vec3 {
                value: vec![0.0, 0.0, 0.0]
            }
        );
    }
    #[test]
    fn x() {
        let vec: Vec3 = Vec3 {
            value: vec![1.0, 0.0, 0.0],
        };
        assert_eq!(vec.x(), 1.0);
    }
    #[test]
    fn y() {
        let vec: Vec3 = Vec3 {
            value: vec![0.0, 1.0, 0.0],
        };
        assert_eq!(vec.y(), 1.0);
    }
    #[test]
    fn z() {
        let vec: Vec3 = Vec3 {
            value: vec![0.0, 0.0, 1.0],
        };
        assert_eq!(vec.z(), 1.0);
    }

    #[test]
    fn add() {
        let vec1: Vec3 = Vec3 {
            value: vec![0.0, 0.0, 1.0],
        };
        let vec2: Vec3 = Vec3 {
            value: vec![0.0, 1.0, 0.0],
        };
        let vec3: Vec3 = Vec3 {
            value: vec![0.0, 1.0, 1.0],
        };
        assert_eq!(vec1 + vec2, vec3);
    }

    #[test]
    fn sub() {
        let vec1: Vec3 = Vec3 {
            value: vec![0.0, 1.0, 1.0],
        };
        let vec2: Vec3 = Vec3 {
            value: vec![0.0, 1.0, 0.0],
        };
        let vec3: Vec3 = Vec3 {
            value: vec![0.0, 0.0, 1.0],
        };
        assert_eq!(vec1 - vec2, vec3);
    }

    #[test]
    fn div() {
        let vec1: Vec3 = Vec3 {
            value: vec![0.0, 3.0, 3.0],
        };
        let vec2: Vec3 = Vec3 {
            value: vec![0.0, 1.0, 1.0],
        };
        assert_eq!(vec1 / 3.0, vec2);
    }

    #[test]
    fn mul() {
        let vec1: Vec3 = Vec3 {
            value: vec![0.0, 3.0, 3.0],
        };
        let vec2: Vec3 = Vec3 {
            value: vec![0.0, 1.0, 1.0],
        };

        assert_eq!(vec2.clone() * 3.0, vec1);
        assert_eq!(3.0 * vec2.clone(), vec1);
    }

    #[test]
    fn normal() {
        let vec: Vec3 = Vec3 {
            value: vec![0.6, 0.8, 0.0],
        };

        assert_eq!(vec.clone(), vec.normalize());
    }

    #[test]
    fn dot() {
        let vec1: Vec3 = Vec3 {
            value: vec![2.0, 2.0, 2.0],
        };
        let vec2: Vec3 = Vec3 {
            value: vec![3.0, 3.0, 3.0],
        };

        assert_eq!(vec1.dot(&vec2), 18.0);
    }

    #[test]
    fn cross() {
        let vec1: Vec3 = Vec3 {
            value: vec![1.0, 0.0, 0.0],
        };
        let vec2: Vec3 = Vec3 {
            value: vec![0.0, 1.0, 0.0],
        };
        let vec3: Vec3 = Vec3 {
            value: vec![0.0, 0.0, 1.0],
        };
        assert_eq!(vec1.cross(&vec2), vec3);
    }
}
