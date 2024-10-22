use std::collections::BTreeSet;

use log::{info, warn};
use nalgebra::{Quaternion, UnitQuaternion};
use rerun::{components::{PoseRotationAxisAngle, PoseRotationQuat}, Angle, Arrows3D, Boxes3D, RotationAxisAngle, Scalar, Text, TextDocument, Vec3D};
use serde::{Deserialize, Serialize};
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
            lil_rerun: LilRerun::new(name, "default-group".to_string(), run_id),
            time: Timepoint::zero(),
        }
    }
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AttitudeTemp{
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

        let mut rerun = if let Some(rerun) = rerun {
            rerun
        } else {
            warn!("Rerun not found");
            return DataView::new();
        };
    
        rerun.set_time_seconds("system-time", self.time.secs());

             
        let data_map = state.get_latest_map(&TopicKey::empty()).unwrap();
        for (key, value) in data_map.iter() {
         
            match value {
                Primitives::Unset => {}
                Primitives::Instant(timepoint) => {}
                Primitives::Duration(timespan) => {}
                Primitives::Integer(val) => {
                    rerun.log(key.display_name(), &Scalar::new(*val as f64));
                }
                Primitives::Float(val) => {
                    rerun.log(key.display_name(), &Scalar::new(*val));
                }
                Primitives::Text(val) => {
                    rerun.log(key.display_name(), &TextDocument::new(val.clone()));
                }
                Primitives::Blob(vic_blob) => {
                    info!("Rerun logging blob: {:?}", vic_blob);
                }
                Primitives::Boolean(_) => {}
                Primitives::List(vec) => {}
                Primitives::Reference(_) => {}
                Primitives::StructType(_) => {
                    info!("Rerun logging struct: {:?}", value);
                }
            }

            let roll = data_map.get(&TopicKey::from_str("attitude/roll")).unwrap();
            let pitch = data_map.get(&TopicKey::from_str("attitude/pitch")).unwrap();
            let yaw = data_map.get(&TopicKey::from_str("attitude/yaw")).unwrap();

          
            match (roll, pitch, yaw) {
                (Primitives::Float(roll), Primitives::Float(pitch), Primitives::Float(yaw)) => {
                    let quat = UnitQuaternion::from_euler_angles(*roll, *pitch, *yaw);
                    let quat = quat.quaternion();
                    rerun.log("attitude", &Boxes3D::from_sizes(vec![
                        Vec3D::new(1.0, 1.0, 0.1),
                     ]).with_quaternions(vec![[quat.coords[0] as f32, quat.coords[1] as f32, quat.coords[2] as f32, quat.coords[3] as f32]]));
                }
                _ => {}
            }
        }

        self.time = self.time.clone() + _dt;
        DataView::new()
    }

    fn cleanup(&mut self) {}
    
    fn get_subscribed_topics(&self) -> std::collections::BTreeSet<TopicKey> {
        let mut topics = BTreeSet::new();
        topics.insert(TopicKey::from_str("attitude"));
        topics
    }
}
