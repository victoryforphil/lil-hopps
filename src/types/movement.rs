#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Movement {
    pub lin_accel: nalgebra::Vector3<f32>,
    pub ang_accel: nalgebra::Vector3<f32>,
    pub lin_vel: nalgebra::Vector3<f32>,
    pub ang_vel: nalgebra::Vector3<f32>,
}

impl Movement {
    pub fn new(
        lin_accel: nalgebra::Vector3<f32>,
        ang_accel: nalgebra::Vector3<f32>,
        lin_vel: nalgebra::Vector3<f32>,
        ang_vel: nalgebra::Vector3<f32>,
    ) -> Movement {
        Movement {
            lin_accel,
            ang_accel,
            lin_vel,
            ang_vel,
        }
    }

    pub fn zero() -> Movement {
        Movement {
            lin_accel: nalgebra::Vector3::zeros(),
            ang_accel: nalgebra::Vector3::zeros(),
            lin_vel: nalgebra::Vector3::zeros(),
            ang_vel: nalgebra::Vector3::zeros(),
        }
    }
}

impl std::fmt::Display for Movement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Linear Acceleration: [{:.4},{:.4},{:.4}], Angular Acceleration: [{:.4},{:.4},{:.4}], Linear Velocity: [{:.4},{:.4},{:.4}], Angular Velocity: [{:.4},{:.4},{:.4}]", self.lin_accel.x, self.lin_accel.y, self.lin_accel.z, self.ang_accel.x, self.ang_accel.y, self.ang_accel.z, self.lin_vel.x, self.lin_vel.y, self.lin_vel.z, self.ang_vel.x, self.ang_vel.y, self.ang_vel.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_movement_new() {
        let lin_accel = nalgebra::Vector3::new(1.0, 2.0, 3.0);
        let ang_accel = nalgebra::Vector3::new(4.0, 5.0, 6.0);
        let lin_vel = nalgebra::Vector3::new(7.0, 8.0, 9.0);
        let ang_vel = nalgebra::Vector3::new(10.0, 11.0, 12.0);
        let movement = Movement::new(lin_accel, ang_accel, lin_vel, ang_vel);
        assert_eq!(movement.lin_accel, lin_accel);
        assert_eq!(movement.ang_accel, ang_accel);
        assert_eq!(movement.lin_vel, lin_vel);
        assert_eq!(movement.ang_vel, ang_vel);
    }

    #[test]
    fn test_movement_zero() {
        let movement = Movement::zero();
        assert_eq!(movement.lin_accel, nalgebra::Vector3::zeros());
        assert_eq!(movement.ang_accel, nalgebra::Vector3::zeros());
        assert_eq!(movement.lin_vel, nalgebra::Vector3::zeros());
        assert_eq!(movement.ang_vel, nalgebra::Vector3::zeros());
    }

    #[test]
    fn test_movement_display() {
        let lin_accel = nalgebra::Vector3::new(1.0, 2.0, 3.0);
        let ang_accel = nalgebra::Vector3::new(4.0, 5.0, 6.0);
        let lin_vel = nalgebra::Vector3::new(7.0, 8.0, 9.0);
        let ang_vel = nalgebra::Vector3::new(10.0, 11.0, 12.0);
        let movement = Movement::new(lin_accel, ang_accel, lin_vel, ang_vel);
        let result = format!("{}", movement);
        assert_eq!(result, "Linear Acceleration: [1.0000,2.0000,3.0000], Angular Acceleration: [4.0000,5.0000,6.0000], Linear Velocity: [7.0000,8.0000,9.0000], Angular Velocity: [10.0000,11.0000,12.0000]");
    }
}
