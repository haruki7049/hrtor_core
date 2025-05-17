//! # hrtor_core

pub mod constants;
pub mod parser;

use constants::CommandStatus;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FileInfo {
    pub path: Option<PathBuf>,
    pub context: String,
}

#[derive(Clone)]
pub struct HrtorProcessor {
    pub editing_file: FileInfo,
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

impl HrtorProcessor {
    /// Creates Hrtorprocessor from a FileInfo which user want to edit
    fn from(file: FileInfo) -> Self {
        Self { editing_file: file }
    }
}

pub trait Processor {
    /// Handle the strings from inputs by main.rs on Hrtor implementation
    fn handle_command(&mut self, command: ReadResult) -> anyhow::Result<CommandStatus>;

    /// Evaluates the command
    fn eval(&mut self, str: String) -> anyhow::Result<CommandStatus>;
}

pub struct Hrtor {
    pub processor: HrtorProcessor,
}

impl Hrtor {
    /// Creates Hrtor instance by HrtorProcessor
    pub fn new(processor: HrtorProcessor) -> Self {
        Self { processor }
    }

    /// Creates Hrtor instance from the file user want to edit
    pub fn from(file: FileInfo) -> Self {
        Self {
            processor: HrtorProcessor::from(file),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{FileInfo, Hrtor, HrtorProcessor};

    #[test]
    fn test_hrtor_struct() {
        let hrtor_processor: HrtorProcessor = HrtorProcessor {
            editing_file: FileInfo {
                path: None,
                context: String::from("test"),
            },
        };
        let _hrtor = Hrtor::new(hrtor_processor);
    }
}
