mod network;
mod utils;

use std::io::{self};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let server_address = "88.178.231.193:17172";

    println!("Enter your username: ");
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("Failed to read username");
    let username = username.trim().to_string();

    let stream = match TcpStream::connect(server_address) {
        Ok(stream) => {
            println!("Connected to the server.");
            Arc::new(Mutex::new(stream))
        }
        Err(e) => {
            println!("Connection error: {}", e);
            return;
        }
    };

    let receive_stream = Arc::clone(&stream);
    let username_clone = username.clone();

    let receive_thread = thread::spawn(move || {
        network::receive_messages(receive_stream, username_clone);
    });

    network::send_messages(stream, username);

    receive_thread.join().expect("Failed to join receive thread");
}
