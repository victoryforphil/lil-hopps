use std::{path::{Path, PathBuf}, sync::{Arc, Mutex}};

use lil_broker::{Database, Primatives};
use tracing::{debug, info};
use rerun::{external::re_log::ResultExt, RecordingStream, RecordingStreamBuilder};
#[derive(Debug, Clone, PartialEq)]
pub enum RerunMode {
    Save,
    Live,
}
#[derive(Debug, Clone)]
pub struct RerunDataview {
    pub name: String,
    pub group: String,
    pub run_id: String,
    pub rerun_override: Option<RerunMode>,
    pub rerun: Option<RecordingStream>,
    pub db: Arc<Mutex<Database>>,
}

impl RerunDataview {
    pub fn new(name: String, group: String, run_id: String, db: Arc<Mutex<Database>>) -> Self {
        RerunDataview {
            name,
            group,
            run_id,
            rerun_override: None,
            rerun: None,
            db: db,
        }
    }
    pub fn set_rerun_live(&mut self) {
        self.rerun_override = Some(RerunMode::Live);
    }

    pub fn set_rerun_save(&mut self) {
        self.rerun_override = Some(RerunMode::Save);
    }

    /// Get the current rerun mode
    ///
    /// If self.rerun_override is Some, return that value
    ///
    /// else will read from the environment variable RERUN_MODE (LIVE, SAVE)
    pub fn get_rerun_mode(&self) -> RerunMode {
        let rerun_mode = std::env::var("RERUN_MODE").unwrap_or("SAVE".to_string());
        match self.rerun_override {
            Some(RerunMode::Live) => RerunMode::Live,
            Some(RerunMode::Save) => RerunMode::Save,
            _ => match rerun_mode.as_str().to_uppercase().as_str() {
                "LIVE" => RerunMode::Live,
                "SAVE" => RerunMode::Save,
                _ => RerunMode::Save,
            },
        }
    }
    /// Get the path to save the rerun data
    ///
    /// This will be the root workspace path, and will create a folder
    /// .firefly/logs
    ///
    pub fn get_rerun_save_path(&self, group: &str) -> PathBuf {
        let path = PathBuf::new()
            // Root workspace path
            .join(Path::new(env!("CARGO_MANIFEST_DIR")))
            .join("..")
            .join(Path::new("lil-data/logs"))
            .join(group);
        //.join(now.to_string());
        if !path.exists() {
            std::fs::create_dir_all(&path).unwrap();
        }
        // Empty folder
        for entry in std::fs::read_dir(&path).unwrap() {
            let entry = entry.unwrap();
            std::fs::remove_file(entry.path()).unwrap();
        }
        path
    }

    pub fn update(&mut self) -> Result<(), anyhow::Error>{
        let mut db = self.db.lock().unwrap();
        let mut rec = self.rerun.as_mut().unwrap();
        for (key, bucket) in &db.buckets {
           for (time, value) in &bucket.values {
                rec.set_time_seconds("time", time.seconds());
               match value.data{
                     Primatives::Number(float) => {
                        rec.log(
                            self.name.clone() + "/" + key.as_str() + "/float",
                            &rerun::Scalar::new(float as f64),
                        )?;
                     }
                     _ => {
                     }
               }
           }
        }
        
        Ok(())

    }
 

    pub fn logging_start(&mut self) {
        let app_id = format!("{}/{}", self.group, self.name);
        let rerun_mode = self.get_rerun_mode();
        let path = self.get_rerun_save_path(&self.group);
        let rec = match rerun_mode {
            RerunMode::Save => RecordingStreamBuilder::new(app_id).recording_id(self.run_id.clone())
                .save(path.join(format!("{}.rrd", self.name)))
                .unwrap(),
            RerunMode::Live => RecordingStreamBuilder::new(app_id).recording_id(self.run_id.clone()).spawn().unwrap(),
            _ => RecordingStreamBuilder::new(app_id)
                .save(path.join(format!("{}.rrd", self.name)))
                .unwrap(),
        };
        info!("Rerun mode: {:?}", rerun_mode);

        self.rerun = Some(rec);
    }
}
