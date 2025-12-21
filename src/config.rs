use std::io;


#[derive(Debug)]
pub enum ConfigError {
    IoError(io::Error),
}
