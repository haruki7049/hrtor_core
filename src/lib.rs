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

pub trait HrtorProcessor where Self: Sized {
    /// Handle the strings from inputs by main.rs on Hrtor implementation
    fn handle_command(&mut self, command: ReadResult) -> anyhow::Result<CommandStatus>;

    /// Evaluates the command
    fn eval(&mut self, str: String) -> anyhow::Result<CommandStatus>;
}

pub trait Hrtor {
    /// Creates Hrtor instance by HrtorProcessor
    fn new<P: HrtorProcessor>(processor: P) -> Self;
}
