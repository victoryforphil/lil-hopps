#[derive(Debug, Clone, Copy)]
pub struct UAVConfig{
    pub motor_force_n: f32,
    pub arm_length_m: f32,
    pub weight_g: f32,
}

impl UAVConfig{
    pub fn new_250mm() -> UAVConfig{
        UAVConfig{
            motor_force_n: 1000.0,
            arm_length_m: 125.0,
            weight_g: 500.0,
        }
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
    }
}

