use std::{fmt,
          path::{Path, PathBuf}};

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub log_path: Option<PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_error_path: Option<PathBuf>,
    #[serde(default)]
    pub rotation: RotationKind,
    pub level: Option<LevelInner>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExporterEndpoint {
    pub port: u16,
    pub host: String,
}

impl ExporterEndpoint {
    pub fn get_host(&self) -> String { format!("{}:{}", self.host, self.port) }
}

impl Config {
    pub fn log_path(&self) -> Option<LogPath> { self.log_path.as_ref().map(Self::build_path) }

    pub fn log_error_path(&self) -> Option<LogPath> {
        self.log_error_path.as_ref().map(Self::build_path)
    }

    fn build_path(path: &PathBuf) -> LogPath {
        let directory = path.parent().unwrap_or_else(|| Path::new(""));
        let file_name = path.file_name().unwrap_or(path.as_os_str());
        LogPath {
            directory: directory.display().to_string(),
            filename: file_name.to_string_lossy().to_string(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            log_error_path: None,
            log_path: None,
            rotation: RotationKind::default(),
            level: None,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LogPath {
    pub directory: String,
    pub filename: String,
}

/// Defines a fixed period for rolling of a log file.
#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize, Hash, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum RotationKind {
    /// Provides a rotation that never rotates.
    #[default]
    Never,
    /// Provides an minutely rotation.
    Minutely,
    /// Provides an hourly rotation.
    Hourly,
    /// Provides a daily rotation.
    Daily,
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize, Hash, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum LevelInner {
    /// The "trace" level.
    ///
    /// Designates very low priority, often extremely verbose, information.
    Trace,
    /// The "debug" level.
    ///
    /// Designates lower priority information.
    Debug,
    /// The "info" level.
    ///
    /// Designates useful information.
    #[default]
    Info,
    /// The "warn" level.
    ///
    /// Designates hazardous situations.
    Warn,
    /// The "error" level.
    ///
    /// Designates very serious errors.
    Error,
}

impl fmt::Display for LevelInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{:?}", &self) }
}
