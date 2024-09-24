use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use crate::utils::get_current_time;

pub fn receive_messages(stream: Arc<Mutex<TcpStream>>, username: String) {
    let mut buffer = vec![0; 1024];
    let mut reader = stream.lock().unwrap().try_clone().expect("Failed to clone stream");

    loop {
        match reader.read(&mut buffer) {
            Ok(0) => {
                println!("\nDisconnected from server.");
                break;
            }
            Ok(n) => {
                let message = String::from_utf8_lossy(&buffer[..n]);
                print!("\r{}\n{}: ", message, username);
                io::stdout().flush().unwrap();
            }
            Err(e) => {
                println!("Error receiving message from server: {}", e);
                break;
            }
        }
    }
}

pub fn send_messages(stream: Arc<Mutex<TcpStream>>, username: String) {
    let stdin = io::stdin();

    loop {
        print!("{}: ", username);
        io::stdout().flush().unwrap();

        let mut message = String::new();
        stdin.read_line(&mut message).expect("Failed to read from stdin");

        let current_time = get_current_time();
        let full_message = format!("[{}] [{}]: {}", current_time, username, message.trim());

        let mut stream = stream.lock().unwrap();
        if let Err(e) = stream.write_all(full_message.as_bytes()) {
            println!("Error sending message: {}", e);
            break;
        }

        if let Err(e) = stream.flush() {
            println!("Error flushing stream: {}", e);
            break;
        }
    }
}
