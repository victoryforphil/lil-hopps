use wingman::types::pose::Pose;

/// NavigationHardware is a trait that defines the interface for hardware that can provide pose information.
/// 
/// # Functions
/// 
/// * get_pose - Returns the current pose of the hardware.
pub trait NavigationHardware{
    fn get_pose(&self) -> Result<Pose, String>;
}

#[cfg(test)]
mod tests{
    use super::*;
    use nalgebra::Vector3;
    use nalgebra::UnitQuaternion;

    struct MockNavigationHardware{
        pose: Pose
    }

    impl MockNavigationHardware{
        fn new(pose: Pose) -> MockNavigationHardware{
            MockNavigationHardware{
                pose
            }
        }
    }

    impl NavigationHardware for MockNavigationHardware{
        fn get_pose(&self) -> Result<Pose, String>{
            Ok(self.pose.clone())
        }
    }

    #[test]
    fn test_mock_navigation_hardware(){
        let position = Vector3::new(1.0, 2.0, 3.0);
        let orientation = UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0);
        let pose = Pose::new(position, orientation);
        let mock_navigation_hardware = MockNavigationHardware::new(pose);
        let result = mock_navigation_hardware.get_pose();
        assert!(result.is_ok());
        let result_pose = result.unwrap();
        assert_eq!(result_pose.position, position);
        assert_eq!(result_pose.orientation, orientation);
    }
}