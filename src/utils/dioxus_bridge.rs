use dioxus::logger::tracing;
use dioxus::prelude::*;
use std::io::{BufRead, BufReader, Write};
//use std::fmt::Write as plat;
use std::process::*;
use std::thread;
use std::path::PathBuf;
use std::fs;
use std::time::Duration;
use std::sync::{Arc, Mutex};

use crate::utils::ShellContext;
use crate::CURRENT_DIR;

pub fn initialize_dioxus_bridge() -> Task {
    let shell = use_context::<ShellContext>();
    let dioxus_bridge = fs::read_to_string(PathBuf::from("js-src/dioxus_bridge.js")).expect("JS file is not valid");
    document::eval(&dioxus_bridge);

    spawn(async move {
        let process_queue = fs::read_to_string(PathBuf::from("js-src/process_queue.js")).expect("JS file is not valid");
        let mut eval = document::eval(&process_queue);
        while let Ok(message) = eval.recv::<String>().await {
            tracing::info!("Received from JS: {}", message);
            send_command(&shell, &message);
        }
    })
}

pub fn send_command(shell: &ShellContext, command: &str) {
    // im thinking wrap this into a use_resource hook, return the value and then display terminal output.
    // although the issue here is that you write to stdin and stdout is populated, so its not a 'taskable' resource
    shell.stdout.lock().unwrap().clear();

    tracing::info!("{}", "Sending command to bash instance");
    let mut stdin = shell.stdin.lock().unwrap();
    writeln!(stdin, "{}", command).expect("Failed to write to shell");
    
   
    thread::sleep(Duration::from_millis(100));
    let stdout_clone: Arc<Mutex<Vec<String>>> = shell.stdout.clone();
    let raw_output = stdout_clone.lock().unwrap().join("\n");

    // Escape `"` and `\n` for JavaScript
    let escaped_output = raw_output
        .replace('\\', "\\\\") // Escape backslashes
        .replace('"', "\\\"") // Escape double quotes
        .replace('\n', "\\n\\r"); // Convert newlines to `\n`

    let output = format!(r#"window.displayTerminalOutput("{}");"#, escaped_output);
    document::eval(&output);

    // Update the current directory the one just used?
    if command.starts_with("cd") {
        writeln!(stdin, "{}", "pwd").expect("Failed to write to shell");
        thread::sleep(Duration::from_millis(100));
        let stdout_clone_b: Arc<Mutex<Vec<String>>> = shell.stdout.clone();
        let raw_output_b = stdout_clone_b.lock().unwrap().join("");

        // slight issue: it has the whole history when trying to change directory.
        *CURRENT_DIR.write() = raw_output_b;
    }
}

/*
  r#"
            if (window.dioxusBridge) {{
                const response = {};
                console.log("Received from Rust:", response);
                window.displayTerminalOutput(response);
                
                // If there are callbacks waiting, resolve the first one
                if (window.dioxusBridge.waitCallbacks.length > 0) {{
                    const callback = window.dioxusBridge.waitCallbacks.shift();
                    callback(response);
                }} else {{
                    // Otherwise, add to the queue
                    window.dioxusBridge.fromRustQueue.push(response);
                }}
            }}
            "#, 
            */

// pub fn check_directory_from_bash(shell: &ShellContext) {
//     let mut stdin = shell.stdin.lock().unwrap();
//     writeln!(stdin, "{}", "pwd").expect("Failed to write to shell");
//     let stdout = shell.stdout.clone();
//     let output = stdout.lock().unwrap();

//     for line in output.iter() {
//         if CURRENT_DIR() != line.clone() {
//             *CURRENT_DIR.write() = line.clone();

//         }
//     }
// }


pub fn initialize_bash() {

    let mut child = Command::new("sh")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start shell");

    let stdin = Arc::new(Mutex::new(child.stdin.take().expect("Failed to open stdin")));
    let stdout = Arc::new(Mutex::new(vec![]));
    let stdout_clone = stdout.clone();
    let reader = BufReader::new(child.stdout.take().expect("Failed to open stdout"));
    thread::spawn(move || {
        for line in reader.lines() {
            if let Ok(output) = line {
                stdout_clone.lock().unwrap().push(output);
            }
        }
    });

    use_context_provider(|| ShellContext {
        stdin,
        stdout
    });

    tracing::info!("{}", "Successfully created bash instance");
}