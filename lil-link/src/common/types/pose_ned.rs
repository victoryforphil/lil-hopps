use serde::{Deserialize, Serialize};

use super::vector3::Vector3;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuadPoseNED {
    pub position: Vector3,
    pub velocity: Vector3,
}

impl QuadPoseNED {
    pub fn zero() -> Self {
        Self {
            position: Vector3::zero(),
            velocity: Vector3::zero(),
        }
    }

    pub fn new_xyz(x: f64, y: f64, z: f64) -> Self {
        Self {
            position: Vector3::new(x, y, z),
            ..Self::zero()
        }
    }

    pub fn new_position_and_velocity(position: Vector3, velocity: Vector3) -> Self {
        Self { position, velocity }
    }

    pub fn distance(&self, other: &Self) -> f64 {
        self.position.distance(&other.position)
    }
}

impl Default for QuadPoseNED {
    fn default() -> Self {
        Self::zero()
    }
}
