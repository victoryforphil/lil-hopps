use lil_link::common::types::pose_ned::QuadPoseNED;
use log::{info, warn};
use nalgebra::UnitQuaternion;
use rerun::{
    Boxes3D, Scalar, TextDocument, Vec3D,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use victory_commander::system::System;
use victory_data_store::{database::DataView, primitives::Primitives, topics::TopicKey};
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

impl System for RerunSystem {
    fn init(&mut self) {
        self.lil_rerun.create_rerun();
    }

    fn execute(&mut self, state: &DataView, _dt: Timespan) -> DataView {
        let rerun = &mut self.lil_rerun.rerun;

        let rerun = if let Some(rerun) = rerun {
            rerun
        } else {
            warn!("Rerun not found");
            return DataView::new();
        };
        rerun.log_static(
            "floor",
            &Boxes3D::from_sizes(vec![Vec3D::new(100.0, 100.0, 0.1)]),
        ).expect("Failed to log floor");
        rerun.set_time_seconds("system-time", self.time.secs());

        let data_map = state.get_latest_map(&TopicKey::empty()).unwrap();
        for (key, value) in data_map.iter() {
            match value {
                Primitives::Unset => {}
                Primitives::Instant(_timepoint) => {}
                Primitives::Duration(_timespan) => {}
                Primitives::Integer(val) => {
                    rerun.log(key.display_name(), &Scalar::new(*val as f64)).expect("Failed to log integer");   
                }
                Primitives::Float(val) => {
                    rerun.log(key.display_name(), &Scalar::new(*val)).expect("Failed to log float");
                }
                Primitives::Text(val) => {
                    rerun.log(key.display_name(), &TextDocument::new(val.clone()))
                        .expect("Failed to log text");
                }
                Primitives::Blob(vic_blob) => {
                    info!("Rerun logging blob: {:?}", vic_blob);
                }
                Primitives::Boolean(bool) => {
                    // Log text vesion
                    rerun.log(
                        key.display_name(),
                        &TextDocument::new(if *bool { "true" } else { "false" }.to_string()),
                    )
                    .expect("Failed to log boolean");
                    rerun
                        .log(key.display_name(), &Scalar::new(*bool as i64 as f64))
                        .expect("Failed to log boolean");
                }
                Primitives::List(_vec) => {}
                Primitives::Reference(_) => {}
                Primitives::StructType(_) => {
                    // info!("Rerun logging struct: {:?}", value);
                }
            }
        }
        let roll = data_map
            .get(&TopicKey::from_str("attitude/roll"))
            .unwrap_or(&Primitives::Float(0.0));
        let pitch = data_map
            .get(&TopicKey::from_str("attitude/pitch"))
            .unwrap_or(&Primitives::Float(0.0));
        let yaw = data_map
            .get(&TopicKey::from_str("attitude/yaw"))
            .unwrap_or(&Primitives::Float(0.0));

        let position = state
            .get_latest(&TopicKey::from_str("position/ned"))
            .unwrap_or(QuadPoseNED::new_xyz(0.0, 0.0, 0.0));

        if let (Primitives::Float(roll), Primitives::Float(pitch), Primitives::Float(yaw)) = (roll, pitch, yaw) {
            let quat = UnitQuaternion::from_euler_angles(*roll, *pitch, *yaw);
            let quat = quat.quaternion();
            rerun.log(
                "attitude",
                &Boxes3D::from_sizes(vec![Vec3D::new(1.0, 1.0, 0.1)])
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
        }

        self.time = self.time.clone() + _dt;
        DataView::new()
    }

    fn cleanup(&mut self) {}

    fn get_subscribed_topics(&self) -> std::collections::BTreeSet<TopicKey> {
        let mut topics = BTreeSet::new();
        topics.insert(TopicKey::from_str("attitude"));
        topics.insert(TopicKey::from_str("status"));
        topics.insert(TopicKey::from_str("params"));
        topics.insert(TopicKey::from_str("position"));
        topics
    }
}
