use lil_link::common::types::pose_ned::QuadPoseNED;
use log::{info, warn};
use nalgebra::UnitQuaternion;
use rerun::{Asset3D, Boxes3D, Scalar, TextDocument, Vec3D};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, path::Path};
use victory_broker::{broker::time::BrokerTime, task::{
    config::{BrokerCommanderFlags, BrokerTaskConfig},
    subscription::BrokerTaskSubscription,
    trigger::BrokerTaskTrigger,
    BrokerTask,
}};
use victory_data_store::{
    database::view::DataView, datapoints::Datapoint, primitives::Primitives, topics::TopicKey,
};
use victory_wtf::{Timepoint, Timespan};

use crate::LilRerun;

pub struct RerunSystem {
    pub lil_rerun: LilRerun,
    time: Timepoint,
}

impl RerunSystem {
    pub fn new(name: String) -> RerunSystem {
        let run_id = Timepoint::now().ms().to_string();
        RerunSystem {
            lil_rerun: LilRerun::new(name, "lil-hopps".to_string(), run_id),
            time: Timepoint::zero(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AttitudeTemp {
    roll: f64,
    pitch: f64,
    yaw: f64,
}

impl BrokerTask for RerunSystem {
    fn init(&mut self) -> Result<(), anyhow::Error> {
        self.lil_rerun.create_rerun();
        let rerun = &mut self.lil_rerun.rerun.as_mut().unwrap();
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("assets/lil-hoppa.glb");
        rerun.log_static("attitude", &Asset3D::from_file(path).unwrap());
        rerun
            .log_static("world", &rerun::ViewCoordinates::RIGHT_HAND_Z_UP)
            .unwrap();
        Ok(())
    }

    fn get_config(&self) -> BrokerTaskConfig {
        BrokerTaskConfig::new("rerun-system")
            .with_trigger(BrokerTaskTrigger::Always)
            .with_subscription(BrokerTaskSubscription::new_updates_only(&TopicKey::empty()))
            .with_subscription(BrokerTaskSubscription::new_latest(&TopicKey::from_str("pose")))
            .with_flag(BrokerCommanderFlags::NonBlocking)
    }

    fn on_execute(&mut self, state: &DataView, timing: &BrokerTime) -> Result<DataView, anyhow::Error> {
       
        let rerun = &mut self.lil_rerun.rerun;
        
        let rerun = if let Some(rerun) = rerun {
            rerun
        } else {
            warn!("Rerun not found");
            return Ok(DataView::new());
        };
        rerun
            .log_static(
                "floor",
                &Boxes3D::from_sizes(vec![Vec3D::new(10.0, 10.0, 0.01)]),
            )
            .expect("Failed to log floor");

        let data_map = state.get_latest_map(&TopicKey::empty()).unwrap();
        rerun.set_time_seconds("broker-time", timing.time_monotonic.secs());
        for (key, datapoint) in data_map.iter() {
           
            match &datapoint.value {
                Primitives::Unset => {}
                Primitives::Instant(_timepoint) => {}
                Primitives::Duration(_timespan) => {}
                Primitives::Integer(val) => {
                    rerun
                        .log(key.display_name(), &Scalar::new(*val as f64))
                        .expect("Failed to log integer");
                }
                Primitives::Float(val) => {
                    rerun
                        .log(key.display_name(), &Scalar::new(*val))
                        .expect("Failed to log float");
                }
                Primitives::Text(val) => {
                    rerun
                        .log(key.display_name(), &TextDocument::new(val.clone()))
                        .expect("Failed to log text");
                }
                Primitives::Blob(vic_blob) => {
                    info!("Rerun logging blob: {:?}", vic_blob);
                }
                Primitives::Boolean(bool_val) => {
                    // Log text vesion
                    rerun
                        .log(
                            key.display_name(),
                            &TextDocument::new(format!(
                                "{}: {}",
                                key.display_name(),
                                if *bool_val { "true" } else { "false" }
                            )),
                        )
                        .expect("Failed to log boolean");
                    rerun
                        .log(key.display_name(), &Scalar::new(*bool_val as i64 as f64))
                        .expect("Failed to log boolean");
                }
                Primitives::List(_vec) => {}
                Primitives::Reference(_) => {}
                Primitives::StructType(_) => {
                    // info!("Rerun logging struct: {:?}", value);
                }
            }
        }
        let roll = state.get_datapoint(&TopicKey::from_str("pose/attitude/rpy_radians/x"));
        let marker_time = roll.map(|dp| dp.time.clone());
        let roll = match roll {
            Some(dp) => &dp.value,
            None => &Primitives::Float(0.0),
        };

        let pitch = state
            .get_datapoint(&TopicKey::from_str("pose/attitude/rpy_radians/y"))
            .map(|dp| &dp.value)
            .unwrap_or(&Primitives::Float(0.0));
        let yaw = state
            .get_datapoint(&TopicKey::from_str("pose/attitude/rpy_radians/z"))
            .map(|dp| &dp.value)
            .unwrap_or(&Primitives::Float(0.0));


        let position: Result<QuadPoseNED, _> = state.get_latest(&TopicKey::from_str("pose/ned"));
        let position_time = state
            .get_latest_map(&TopicKey::from_str("pose/ned"))
            .map(|map| map.iter().map(|(_, dp)| dp.time.clone()).max());
        if let Ok(position) = position {
            if let Ok((Some(position_time))) = position_time {    
                rerun.set_time_seconds("data-time", position_time.secs());
            }
            if let (Primitives::Float(roll), Primitives::Float(pitch), Primitives::Float(yaw)) =
                (roll, pitch, yaw)
            {
                let quat = UnitQuaternion::from_euler_angles(*roll, *pitch, *yaw);
                rerun
                    .log(
                        "attitude",
                        &Boxes3D::from_sizes(vec![Vec3D::new(0.5, 0.5, 0.1)])
                            .with_quaternions(vec![[
                                quat.coords[0] as f32,
                                quat.coords[1] as f32,
                                quat.coords[2] as f32,
                                quat.coords[3] as f32,
                            ]])
                            .with_centers(vec![Vec3D::new(
                                position.position.x as f32,
                                position.position.y as f32,
                                -position.position.z as f32,
                            )]),
                    )
                    .expect("Failed to log attitude");
                // use crate root
            }
        }
        self.time = self.time.clone() + Timespan::new_secs(1.0); // Assuming a 1-second timestep.  Adjust as needed.
        Ok(DataView::new())
    }
}
