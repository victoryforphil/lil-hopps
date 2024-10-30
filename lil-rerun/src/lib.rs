use std::{
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use rerun::RecordingStreamBuilder;
pub mod system;
pub mod types;
pub type LilRerunHandle = Arc<Mutex<LilRerun>>;
#[derive(Debug, Clone, PartialEq)]
pub enum RerunMode {
    Save,
    Live,
    Spawn,
}
#[derive(Debug, Clone)]
pub struct LilRerun {
    pub name: String,
    pub group: String,
    pub run_id: String,
    pub rerun: Option<rerun::RecordingStream>,
    rerun_override: Option<RerunMode>,
}

impl LilRerun {
    pub fn new(name: String, group: String, run_id: String) -> Self {
        LilRerun {
            rerun: None,
            name,
            group,
            run_id,
            rerun_override: None,
        }
    }

    pub fn create_rerun(&mut self) {
        let app_id = format!("{}/{}", self.group, self.name);
        let rerun_mode = self.get_rerun_mode();
        let path = self.get_rerun_save_path(&self.group);
        let rec = match rerun_mode {
            RerunMode::Save => RecordingStreamBuilder::new(app_id)
                .recording_id(self.run_id.clone())
                .save(path.join(format!("{}.rrd", self.name)))
                .unwrap(),
            RerunMode::Spawn => RecordingStreamBuilder::new(app_id)
                .recording_id(self.run_id.clone())
                .spawn()
                .unwrap(),
            _ => RecordingStreamBuilder::new(app_id).connect().unwrap(),
        };

        if rerun_mode != RerunMode::Save {
            // Or log via a logging handler:
            rerun::Logger::new(rec.clone()) // recording streams are ref-counted
                .with_path_prefix("logs/handler")
                // You can also use the standard `RUST_LOG` environment variable!
                .with_filter(rerun::default_log_filter())
                .init()
                .unwrap_err();

            log::info!("This INFO log got added through the standard logging interface");
        }

        self.rerun = Some(rec);
    }

    pub fn rerun(&self) -> Option<rerun::RecordingStream> {
        self.rerun.clone()
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
        match self.rerun_override {
            Some(RerunMode::Live) => RerunMode::Live,
            Some(RerunMode::Save) => RerunMode::Save,
            _ => Self::get_rerun_env(),
        }
    }

    pub fn get_rerun_env() -> RerunMode {
        let rerun_mode: String = std::env::var("RERUN_MODE").unwrap_or("SAVE".to_string());
        match rerun_mode.as_str().to_uppercase().as_str() {
            "LIVE" => RerunMode::Live,
            "SAVE" => RerunMode::Save,
            "SPAWN" => RerunMode::Spawn,
            _ => RerunMode::Save,
        }
    }
    /// Get the path to save the rerun data
    ///
    /// This will be the root workspace path, and will create a folder
    /// .lil/logs
    ///
    pub fn get_rerun_save_path(&self, group: &str) -> PathBuf {
        let path = PathBuf::new()
            // Root workspace path
            .join(Path::new(env!("CARGO_MANIFEST_DIR")))
            .join(Path::new(".lil/logs"))
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
}
