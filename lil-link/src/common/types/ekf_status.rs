use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct QuadEkfStatus {
    pub attitude: bool,       // EKF_ATTITUDE: Set if EKF's attitude estimate is good
    pub vel_horiz: bool, // EKF_VELOCITY_HORIZ: Set if EKF's horizontal velocity estimate is good
    pub vel_vert: bool,  // EKF_VELOCITY_VERT: Set if EKF's vertical velocity estimate is good
    pub pos_horiz_rel: bool, // EKF_POS_HORIZ_REL: Set if EKF's horizontal position (relative) estimate is good
    pub pos_horiz_abs: bool, // EKF_POS_HORIZ_ABS: Set if EKF's horizontal position (absolute) estimate is good
    pub pos_vert_abs: bool, // EKF_POS_VERT_ABS: Set if EKF's vertical position (absolute) estimate is good
    pub pos_vert_agl: bool, // EKF_POS_VERT_AGL: Set if EKF's vertical position (above ground) estimate is good
    pub const_pos_mode: bool, // EKF_CONST_POS_MODE: EKF is in constant position mode and does not know position
    pub pred_pos_horiz_rel: bool, // EKF_PRED_POS_HORIZ_REL: Set if EKF's predicted horizontal position (relative) estimate is good
    pub pred_pos_horiz_abs: bool, // EKF_PRED_POS_HORIZ_ABS: Set if EKF's predicted horizontal position (absolute) estimate is good
    pub uninitialized: bool,      // EKF_UNINITIALIZED: Set if EKF has never been healthy
}

impl QuadEkfStatus {
    pub fn new_null() -> Self {
        Self {
            attitude: false,
            vel_horiz: false,
            vel_vert: false,
            pos_horiz_rel: false,
            pos_horiz_abs: false,
            pos_vert_abs: false,
            pos_vert_agl: false,
            const_pos_mode: false,
            pred_pos_horiz_rel: false,
            pred_pos_horiz_abs: false,
            uninitialized: false,
        }
    }

    pub fn is_healthy(&self) -> Result<(), String> {
        if !self.attitude {
            return Err("Attitude is not healthy".to_string());
        }

        if !self.vel_horiz {
            return Err("Horizontal velocity is not healthy".to_string());
        }

        if !self.vel_vert {
            return Err("Vertical velocity is not healthy".to_string());
        }

        if !self.pos_horiz_rel {
            return Err("Horizontal position (relative) is not healthy".to_string());
        }

        if !self.pos_horiz_abs {
            return Err("Horizontal position (absolute) is not healthy".to_string());
        }

        if !self.pos_vert_abs {
            return Err("Vertical position (absolute) is not healthy".to_string());
        }

        if self.const_pos_mode {
            return Err("EKF is in constant position mode".to_string());
        }

        if self.uninitialized {
            return Err("EKF has not been initialized yet".to_string());
        }

        Ok(())
    }
}
