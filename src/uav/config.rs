#[derive(Debug, Clone, Copy)]
pub struct UAVConfig{
    pub motor_force_n: f32,
    pub arm_length_m: f32,
    pub weight_g: f32,
    pub rotation_lock: (bool, bool, bool), // (x, y, z)
}

impl UAVConfig{
    pub fn new_250mm() -> UAVConfig{
        UAVConfig{
            motor_force_n: 1000.0,
            arm_length_m: 125.0,
            weight_g: 500.0,
            rotation_lock: (false, false, false),
        }
    }

    pub fn lock_pitch(&mut self){
        self.rotation_lock.0 = true;
    }

    pub fn lock_roll(&mut self){
        self.rotation_lock.1 = true;
    }

    pub fn lock_yaw(&mut self){
        self.rotation_lock.2 = true;
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_new_250mm(){
        let config = UAVConfig::new_250mm();
        assert_eq!(config.motor_force_n, 1000.0);
        assert_eq!(config.arm_length_m, 125.0);
        assert_eq!(config.weight_g, 500.0);

        println!("{:?}", config);
    }

    #[test]
    fn test_rotation_lock(){
        let mut config = UAVConfig{
            motor_force_n: 1000.0,
            arm_length_m: 200.0,
            weight_g: 400.0,
            rotation_lock: (false, false, false),
        };
        config.lock_pitch();
        assert_eq!(config.rotation_lock.0, true);
        assert_eq!(config.rotation_lock.1, false);
        assert_eq!(config.rotation_lock.2, false);

        config.lock_roll();
        assert_eq!(config.rotation_lock.0, true);
        assert_eq!(config.rotation_lock.1, true);
        assert_eq!(config.rotation_lock.2, false);

        config.lock_yaw();
        assert_eq!(config.rotation_lock.0, true);
        assert_eq!(config.rotation_lock.1, true);
        assert_eq!(config.rotation_lock.2, true);
    }
}

