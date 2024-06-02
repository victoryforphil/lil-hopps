use std::sync::{Arc, Mutex};

use lil_broker::Database;
use rerun::RecordingStream;

pub struct RerunDataView {
    rec: RecordingStream,
    db: Arc<Mutex<Database>>,
}

impl RerunDataView {
    pub fn new(name: &str, database_arc: Arc<Mutex<Database>>) -> Self {
        let rec = rerun::RecordingStreamBuilder::new(name).spawn().unwrap();
        Self {
            db: database_arc,
            rec,
        }
    }
}
