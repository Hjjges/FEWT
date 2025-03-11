use std::process::ChildStdin;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct ShellContext {
    pub stdin: Arc<Mutex<ChildStdin>>,
    pub stdout: Arc<Mutex<Vec<String>>>,
}