mod pid_controller;
mod height_control;
mod rate_control;
mod angle_control;
mod pos_control;

pub use height_control::HeightControl;
use log::info;
pub use pid_controller::PIDController;
pub use rate_control::RateControl;
pub use angle_control::AngleControl;
pub use pos_control::PositionControl;
use crate::uav::state::UAVState;

pub struct UAVController {
    pub height_control: HeightControl,
    pub rate_control: RateControl,
    pub angle_control: AngleControl,
    pub pos_control: PositionControl,
   
}

impl UAVController {
    pub fn new() -> Self {
        UAVController {
            height_control: HeightControl::new(6.0, 0.4),
            rate_control: RateControl::new(),
            angle_control: AngleControl::new(),
            pos_control: PositionControl::new(),
        }
    }

    pub fn update(&mut self, state: &UAVState, dt: f64) -> [f32; 4] {
        let height = self.height_control.update(state, dt);
        self.pos_control.target_pos = vec![0.0, 3.0];
        let pos_out = self.pos_control.update(state, dt);
        
        self.angle_control.target_angles = vec![-pos_out[1] * 0.1,0.0,  -pos_out[0] * 0.1];

        let [angle_x, angle_y, angle_z] = self.angle_control.update(state, dt);

        info!("angle_x: {}", angle_x);
        self.rate_control.target_rates = vec![angle_x * 400.0, angle_y* 400., angle_z * 400. ];
        let rate = self.rate_control.update(state, dt);
        let mut motors  = [0.0; 4];
        motors[0] =  height as f32 + (rate[2] as f32 );
        motors[1] =  height as f32 + (rate[0] as f32 );
        motors[2] =  height as f32 - (rate[2] as f32 );
        motors[3] = height as f32  - (rate[0] as f32 );
        motors
    }
    
}

