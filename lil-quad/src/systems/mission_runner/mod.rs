use std::collections::BTreeSet;

use lil_link::common::{
    identifiers::{IDENT_BASE_STATUS, IDENT_STATUS_EKF, IDENT_STATUS_HEALTH},
    types::{
        health_status::QuadHealthStatus, mode::QuadMode, request_arm::QuadSetModeRequest,
        request_takeoff::QuadTakeoffRequest,
    },
};
use log::{info, warn};
use task::{TaskType, Tasks};
use victory_commander::system::System;
use victory_data_store::{database::DataView, primitives::Primitives, topics::TopicKey};
use victory_wtf::{Timepoint, Timespan};

use crate::systems::timed_arm::ArmMessage;

pub mod task;

pub struct MissionRunner {
    pub tasks: Vec<TaskType>,
    current_idx: usize,
    subbed_conditions: BTreeSet<TopicKey>,
    last_executed: Option<victory_wtf::Timepoint>,
    current_time: victory_wtf::Timepoint,
}

impl MissionRunner {
    pub fn new(tasks: Vec<TaskType>) -> Self {
        Self {
            tasks,
            current_idx: 0,
            subbed_conditions: BTreeSet::new(),
            last_executed: None,
            current_time: Timepoint::now(),
        }
    }

    fn run_task(&mut self, name: String, task: Tasks, out: &mut DataView) {
        match task {
            Tasks::Takeoff(altitude) => {
                self.send_takeoff(altitude, out);
            }
            Tasks::Arm => {
                self.send_arm(out);
            }
            Tasks::SetMode(mode) => {
                self.send_set_mode(mode, out);
            }
            _ => {
                warn!("Unknown task type: {:?}", task);
            }
        }
        self.current_idx += 1;
        self.last_executed = Some(self.current_time.clone());
        out.add_latest(&TopicKey::from_str("status/mission/current/name"), name)
            .expect("Failed to add current task");
    }
    fn send_takeoff(&mut self, altitude: f32, out: &mut DataView) {
        info!("Sending takeoff command with altitude {}", altitude);
        let takeoff_msg = QuadTakeoffRequest {
            height: altitude,
            ack: false,
        };

        out.add_latest(&TopicKey::from_str("cmd/takeoff"), takeoff_msg)
            .expect("Failed to add takeoff message");
    }

    fn send_arm(&mut self, out: &mut DataView) {
        info!("Sending arm command");
        let arm_msg = ArmMessage {
            arm: true,
            ack: false,
        };
        out.add_latest(&TopicKey::from_str("cmd/arm"), arm_msg)
            .expect("Failed to add arm message");
    }

    fn send_set_mode(&mut self, mode: QuadMode, out: &mut DataView) {
        info!("Sending set mode command with mode {}", mode);
        let mode_msg = QuadSetModeRequest {
            mode: mode.clone(),
            ack: false,
        };
        out.add_latest(&TopicKey::from_str("cmd/mode"), mode_msg)
            .expect("Failed to add mode message");
    }
}

impl System for MissionRunner {
    fn init(&mut self) {
        for task in &self.tasks {
            if let TaskType::Condition(task) = task.clone() {
                self.subbed_conditions.insert(task.topic.clone());
            }
        }
    }

    fn get_subscribed_topics(
        &self,
    ) -> std::collections::BTreeSet<victory_data_store::topics::TopicKey> {
        self.subbed_conditions.clone()
    }

    fn execute<'a>(
        &mut self,
        inputs: &'a victory_data_store::database::DataView,
        dt: victory_wtf::Timespan,
    ) -> victory_data_store::database::DataView {
        let mut out = DataView::new();
        self.current_time = self.current_time.clone() + dt;
        // Write current task to status/mission/current_task

        if self.current_idx >= self.tasks.len() {
            return out;
        }

        let current_task = &self.tasks[self.current_idx];

        match current_task {
            TaskType::Timed(task) => {
                let time_since = self.current_time.clone()
                    - self.last_executed.clone().unwrap_or(Timepoint::zero());
                if time_since.secs() >= task.duration.secs() {
                    info!("Running timed task {}", task.name);
                    self.run_task(task.name.clone(), task.task.clone(), &mut out);
                }
            }
            TaskType::Condition(task) => {
                let read_value = inputs.get_latest_map(&task.topic).unwrap();
                let mut passed = false;

                if let Some(value) = task.value.clone() {
                    match read_value.get(&task.topic) {
                        Some(v) => {
                            passed = *v == value;
                        }
                        None => {
                            passed = false;
                        }
                    }
                } else {
                    passed = inputs.get_latest::<_, Primitives>(&task.topic).is_ok();
                }
                if passed {
                    self.run_task(task.name.clone(), task.task.clone(), &mut out);
                }
            }
        }

        out
    }

    fn cleanup(&mut self) {}
}
