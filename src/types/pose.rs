use polars::prelude::*;


#[derive(Debug, Clone, Copy, PartialEq)]
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
            "Position: [{:.4},{:.4},{:.4}], Orientation: {:?}",
            self.position.x, self.position.y, self.position.z, self.orientation
        )
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
    // Get DataFrame compatbile key value pairs for the pose
    pub fn get_df(&self,lable:String) -> DataFrame
    {
        let mut df = df!(
            format!("{}.position.x", lable).as_str() => &[self.position.x],
            format!("{}.position.y", lable).as_str() => &[self.position.y],
            format!("{}.position.z", lable).as_str() => &[self.position.z],
            format!("{}.orientation.w", lable).as_str() => &[self.orientation.quaternion().w],
            format!("{}.orientation.i", lable).as_str() => &[self.orientation.quaternion().i],
            format!("{}.orientation.j", lable).as_str() => &[self.orientation.quaternion().j],
            format!("{}.orientation.k", lable).as_str() => &[self.orientation.quaternion().k],
            //roll, pitch, yaw
            format!("{}.orientation.r", lable).as_str() => &[self.orientation.euler_angles().0],
            format!("{}.orientation.p", lable).as_str() => &[self.orientation.euler_angles().1],
            format!("{}.orientation.y", lable).as_str() => &[self.orientation.euler_angles().2],

        ).unwrap();

        df
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pose_new() {
        let position = nalgebra::Vector3::new(1.0 as f32, 2.0, 3.0);
        let orientation = nalgebra::UnitQuaternion::from_euler_angles(0.0 as f32, 0.0, 0.0);
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

    #[test]
    fn test_pose_get_df() {
        let position = nalgebra::Vector3::new(1.0, 2.0, 3.0);
        let orientation = nalgebra::UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0);
        let pose = Pose::new(position, orientation);
        let result = pose.get_df("test".to_string());
        let expected = df!(
            "test.position.x" => &[1.0],
            "test.position.y" => &[2.0],
            "test.position.z" => &[3.0],
            "test.orientation.w" => &[1.0],
            "test.orientation.i" => &[0.0],
            "test.orientation.j" => &[0.0],
            "test.orientation.k" => &[0.0],
            "test.orientation.r" => &[0.0],
            "test.orientation.p" => &[0.0],
            "test.orientation.y" => &[0.0],
        ).unwrap();
        assert_eq!(result, expected);
    }
}
