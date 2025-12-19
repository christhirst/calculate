use std::{fs, io, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum ConfigError {
    IoError(io::Error),
}
