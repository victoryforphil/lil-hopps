use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
    time::Duration,
};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lil_broker::Timestamp;
use lil_helper::types::Pose;
use lil_quad::{
    runner::{FixtureRunner, UAVRunnerConfig},
    uav::HoverTask,
};
use serde_json::json;

fn task_hover(duration: f32) {
    let mut pose = Pose::default();
    pose.position.x = 1.0;
    pose.position.y = 1.0;
    pose.position.z = 1.0;

    let mut init_state = BTreeMap::new();
    init_state.insert("sense/pose".to_string(), json!(pose));

    let task = Arc::new(Mutex::new(HoverTask::new()));
    let config = UAVRunnerConfig::default()
        .set_max_t(Timestamp::from_seconds(duration))
        .set_dt(Timestamp::from_hz(1000.0));

    let mut runner =
        FixtureRunner::new(config.clone(), task, init_state).expect("Failed to create runner");
    runner.start().expect("Failed to start runner");
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("task_hover", |b| b.iter(|| task_hover(black_box(5.0))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
