use lil_link::common::types::{mode::QuadMode, pose_ned::QuadPoseNED};
use victory_data_store::{primitives::Primitives, topics::TopicKey};
#[derive(Clone, Debug)]

pub enum TaskType {
    Timed(TimedTask),
    Condition(ConditionTask),
}
#[derive(Clone, Debug)]
pub enum Tasks {
    Arm,
    SetMode(QuadMode),
    Takeoff(f32),
    Land,
    Waypoint(QuadPoseNED),
}
#[derive(Clone, Debug)]
pub struct TimedTask {
    pub name: String,
    pub duration: victory_wtf::Timespan,
    pub task: Tasks,
}
#[derive(Clone, Debug)]
pub struct ConditionTask {
    pub name: String,
    pub topic: TopicKey,
    pub value: Option<Primitives>,
    pub task: Tasks,
}

impl TimedTask {
    pub fn new(name: String, duration: victory_wtf::Timespan, task: Tasks) -> Self {
        Self {
            name,
            duration,
            task,
        }
    }
}

impl ConditionTask {
    pub fn new(name: String, topic: TopicKey, value: Option<Primitives>, task: Tasks) -> Self {
        Self {
            name,
            topic,
            value,
            task,
        }
    }
}
