pub struct PIDController {
    kp: f64,
    ki: f64,
    kd: f64,
    integral: f64,
    prev_error: f64,
    max: f64,
    min: f64,
}

impl PIDController {
    pub fn new(kp: f64, ki: f64, kd: f64) -> Self {
        PIDController {
            kp,
            ki,
            kd,
            integral: 0.0,
            prev_error: 0.0,
            max: f64::MAX,
            min: f64::MIN,
        }
    }

    pub fn new_clamped(kp: f64, ki: f64, kd: f64, min: f64, max: f64) -> Self {
        PIDController {
            kp,
            ki,
            kd,
            integral: 0.0,
            prev_error: 0.0,
            max,
            min,
        }
    }

    pub fn update(&mut self, setpoint: f64, process_variable: f64, dt: f64) -> f64 {
        let error = setpoint - process_variable;
        self.integral += error * dt;
        let derivative = (error - self.prev_error) / dt;
        let output = self.kp * error + self.ki * self.integral + self.kd * derivative;
        self.prev_error = error;
        
        //Clamp output
        if output > self.max {
            self.max
        } else if output < self.min {
            self.min
        } else {
            output
        }
    }

    
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pid_controller_update() {
        let mut pid_controller = PIDController::new(1.0, 0.5, 0.2);
        let setpoint = 10.0;
        let process_variable = 5.0;
        let dt = 0.1;

        let output = pid_controller.update(setpoint, process_variable, dt);

        // Add assertions here to verify the correctness of the output
        assert_eq!(output, 7.5);
    }

    #[test]
    fn test_pid_controller_update_clamped() {
        let mut pid_controller = PIDController::new_clamped(1.0, 0.5, 0.2, 0.0, 10.0);
        let setpoint = 10.0;
        let process_variable = 5.0;
        let dt = 0.1;

        let output = pid_controller.update(setpoint, process_variable, dt);

        // Add assertions here to verify the correctness of the output
        assert_eq!(output, 7.5);
    }
}
