use anyhow::Result;
use dioxus::{document::Eval, html::g::dangerous_inner_html, prelude::*};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::path::PathBuf;
use std::process::Command;
use wasmer::{Store, Module, Instance, Value, imports};
use wasmer_wasi::{WasiState, WasiEnv, WasiFunctionEnv};

use crate::utils::DirectoryContext;

pub struct TerminalState {
    pub current_dir: std::path::PathBuf,
    pub wasmer_store: Store,
    pub wasi_env: WasiFunctionEnv,
}

impl TerminalState {
    pub fn new() -> Result<Self> {
        // Initilaize the current directory
        let current_dir = std::env::current_dir()?;

        // Initialise Wasmer
        let mut store = Store::default();

        let wasi_env = WasiState::new("terminal-user")
            .preopen_dir(current_dir.clone())?
            .finalize(&mut store)?;

        Ok(Self {
            current_dir,
            wasmer_store: store,
            wasi_env,
        })
    }


    pub fn execute_command(&mut self, command: &str) -> String {
        let parts: Vec<&str> = command.trim().split_whitespace().collect();

        if parts.is_empty() {
            return String::new();
        }

        println!("{}", command);
        //let mut commanded = Command::new(command);

        let output = Command::new("sh")
            .arg("-c")
            .arg("echo hello")
            .output()
            .expect("failed to execute process");

        let hello = format!("{:?}", output.stdout);

        return hello;
    }
}
