use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(tag = "_type")]
pub struct Pose {
    pub position: nalgebra::Vector3<f32>,
    pub orientation: nalgebra::UnitQuaternion<f32>,
}

// Add custom debug implementation
impl std::fmt::Display for Pose {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //Formatted: Position: [1, 2, 3], Orientation: Quaternion { w: 1, i: 0, j: 0, k: 0 }
        write!(
            f,
            "Position: [{:.4},{:.4},{:.4}], Orientation: [{:.1}, {:.1}, {:.1}, {:.1}]",
            self.position.x,
            self.position.y,
            self.position.z,
            self.orientation.i,
            self.orientation.j,
            self.orientation.k,
            self.orientation.w
        )
    }
}

impl Default for Pose {
    fn default() -> Self {
        Pose {
            position: nalgebra::Vector3::zeros(),
            orientation: nalgebra::UnitQuaternion::identity(),
        }
    }
}
impl Pose {
    pub fn new(
        position: nalgebra::Vector3<f32>,
        orientation: nalgebra::UnitQuaternion<f32>,
    ) -> Pose {
        Pose {
            position,
            orientation,
        }
    }
    //Generate Rust Docs

    /// Returns a Pose with position and orientation set to zero.
    pub fn zero() -> Pose {
        Pose {
            position: nalgebra::Vector3::zeros(),
            orientation: nalgebra::UnitQuaternion::identity(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pose_new() {
        let position = nalgebra::Vector3::new(1.0_f32, 2.0, 3.0);
        let orientation = nalgebra::UnitQuaternion::from_euler_angles(0.0_f32, 0.0, 0.0);
        let pose = Pose::new(position, orientation);
        assert_eq!(pose.position, position);
        assert_eq!(pose.orientation, orientation);
    }

    #[test]
    fn test_pose_zero() {
        let pose = Pose::zero();
        assert_eq!(pose.position, nalgebra::Vector3::zeros());
        assert_eq!(pose.orientation, nalgebra::UnitQuaternion::identity());
    }

    #[test]
    fn test_pose_display() {
        let position = nalgebra::Vector3::new(1.0, 2.0, 3.0);
        let orientation = nalgebra::UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0);
        let pose = Pose::new(position, orientation);
        let result = format!("{}", pose);
        assert_eq!(
            result,
            "Position: [1.0000,2.0000,3.0000], Orientation: [0.0, 0.0, 0.0, 1.0]"
        );
    }
}
