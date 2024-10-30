use serde::{Deserialize, Serialize};

use super::vector3::Vector3;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuadAttitude {
    pub rpy_radians: Vector3,
}

impl QuadAttitude {
    pub fn zero() -> Self {
        Self {
            rpy_radians: Vector3::zero(),
        }
    }

    pub fn new_xyz(x: f64, y: f64, z: f64) -> Self {
        Self {
            rpy_radians: Vector3::new(x, y, z),
            ..Self::zero()
        }
    }

    pub fn new_f32(x: f32, y: f32, z: f32) -> Self {
        Self {
            rpy_radians: Vector3::new_f32(x, y, z),
        }
    }

    pub fn new_rpy_radians(rpy_radians: Vector3) -> Self {
        Self { rpy_radians }
    }
}

impl Default for QuadAttitude {
    fn default() -> Self {
        Self::zero()
    }
}
