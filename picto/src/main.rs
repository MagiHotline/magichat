use std::{
    io::Write,
    sync::{Arc, Mutex},
    thread,
};

use client::client::Client;

fn main() {
    // Bind to a socket
    println!("First, bind your socket to an address: ");
    let mut bind_address = String::new();
    std::io::stdin()
        .read_line(&mut bind_address)
        .expect("Failed to get input");

    let client =
        Client::new(bind_address.trim().to_string()).expect("Failed to create a new Client");

    // Needs mutable access
    let sender_client = Arc::new(Mutex::new(client));
    let receiver_client = Arc::clone(&sender_client);

    // Get the name of the client
    println!("What is your name?");
    let mut name = String::new();
    std::io::stdin()
        .read_line(&mut name)
        .expect("Failed to get input");

    println!("Insert the address where you want to connect: ");
    let mut endpoint = String::new();
    std::io::stdin()
        .read_line(&mut endpoint)
        .expect("Failed to get input");

    println!("ENDPOINT IN CONNECTION: {endpoint}");

    sender_client
        .lock()
        .unwrap()
        .connect(&endpoint.trim().to_string())
        .expect("Failed to connect to the address");

    println!(
        "Connected successfully to {}! Start sending and receiving messages",
        endpoint
    );

    let mut input = String::new();

    // Spawn the "listening thread"
    thread::spawn(move || {
        loop {
            let mut buf = [0; 10];
            if let Ok(_) = receiver_client.lock().unwrap().recv(&mut buf) {
                let output = str::from_utf8(&buf).expect("Failed to convert from bytes to string");
                println!("{output}");
            } else {
                println!("recv function failed");
                break;
            }
        }
    });

    // sending thread
    loop {
        let id = name.trim().to_string() + "> ";
        print!("{id}");
        std::io::stdout().flush().unwrap();

        input.clear();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to get input");

        sender_client
            .lock()
            .unwrap()
            .send((id + &input).as_bytes())
            .expect("Failed to transform string to bytes");
    }
}
