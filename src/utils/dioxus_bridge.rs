use dioxus::prelude::*;
use std::io::{BufRead, BufReader, Write};
use std::fmt::Write as plat;
use std::process::*;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

use crate::utils::ShellContext;

use super::DirectoryContext;

pub fn initialize_dioxus_bridge() -> Task {
    let bridge_script = r#"
    window.dioxusBridge = {
        toRustQueue: [],
        fromRustQueue: [],
        waitCallbacks: [],

        sendToRust: function(message) {
            this.toRustQueue.push(message);
        },

        receiveFromRust: function() {
            return new Promise((resolve) => {
                if (this.fromRustQueue.length > 0) {
                    resolve(this.fromRustQueue.shift());
                } else {
                    this.waitCallbacks.push(resolve);
                }
            });
        }
    }

    console.log("Rust Macro - Dioxus Bridge Initialized");
    "#;

    document::eval(bridge_script);

    spawn(async move {
        let mut eval = document::eval(r#"
        async function processToRustQueue() {
            while (true) {
                if (window.dioxusBridge && window.dioxusBridge.toRustQueue.length > 0) {
                    const message = window.dioxusBridge.toRustQueue.shift();
                    console.log("Sending to Rust:", message);
                    dioxus.send(message); // utilising dioxus rust macro context
                }
                await new Promise(resolve => setTimeout(resolve, 100));
            }
        }
        processToRustQueue();
        "#);

        while let Ok(message) = eval.recv::<String>().await {
            println!("Received from JS: {}", message);
            let shell = use_context::<ShellContext>();
            send_command(&shell, &message);
        }
    })
}

pub fn send_command(shell: &ShellContext, command: &str) {


    shell.stdout.lock().unwrap().clear();

    println!("{}", "Sending command to bash instance");
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

pub fn check_directory_from_bash(shell: &ShellContext) {
    let mut stdin = shell.stdin.lock().unwrap();
    writeln!(stdin, "{}", "pwd").expect("Failed to write to shell");
    let stdout = shell.stdout.clone();
    let output = stdout.lock().unwrap();

    for line in output.iter() {
        let current_dir = use_context::<DirectoryContext>().current_directory;
        if current_dir.read().to_string() != line.clone() {
            consume_context::<DirectoryContext>().current_directory.set(line.clone());
        }
    }
}


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

    // let stdout_clone_b = stdout.clone();
    // thread::spawn(move || {
    //     loop {
    //         thread::sleep(Duration::from_secs(15));
    //         let output = stdout_clone_b.lock().unwrap();
    //         println!("Shell Output History:\n{}", output.join("\n"));
    //     }
    // });

    use_context_provider(|| ShellContext {
        stdin,
        stdout
    });

    println!("{}", "Successfully created bash instance");
}