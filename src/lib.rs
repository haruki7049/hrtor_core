//! # hrtor_core

pub mod constants;
pub mod parser;

use constants::CommandStatus;
use std::collections::HashMap;
use std::path::PathBuf;

pub trait Loader {
    /// Loads the buffer
    fn buffer(&self) -> FileInfo;

    /// Loads the configuration files
    fn config(&self) -> HashMap<String, FileInfo>;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FileInfo {
    pub path: Option<PathBuf>,
    pub context: String,
}

pub enum ReadResult {
    Input(String),
    Signal(Signal),
    Eof,
}

pub enum Signal {
    Break,
    Continue,
    Interrupt,
    Resize,
    Suspend,
    Quit,
}

pub trait Processor {
    /// Handle the strings from inputs by main.rs on Hrtor implementation
    fn handle_command(&mut self, command: ReadResult) -> anyhow::Result<CommandStatus>;

    /// Evaluates the command
    fn eval(&mut self, str: String) -> anyhow::Result<CommandStatus>;
}
