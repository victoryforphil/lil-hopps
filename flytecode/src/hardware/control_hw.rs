pub trait ControlHardware{
    fn set_motor_values(&mut self, motor_values: [f32; 4]) -> Result<(), String>;
}