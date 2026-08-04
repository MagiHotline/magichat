use std::{io::Write, thread};

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

    // Get the name of the client
    println!("What is your name?");
    let mut name = String::new();
    std::io::stdin()
        .read_line(&mut name)
        .expect("Failed to get input");

    name = name.trim().to_string();

    println!("Insert the address where you want to connect: ");
    let mut endpoint = String::new();
    std::io::stdin()
        .read_line(&mut endpoint)
        .expect("Failed to get input");

    println!("ENDPOINT IN CONNECTION: {endpoint}");

    client
        .connect(&endpoint.trim().to_string())
        .expect("Failed to connect to the address");

    println!(
        "Connected successfully to {}! Start sending and receiving messages",
        endpoint.trim()
    );

    let mut input = String::new();
    let receiver_client = client.try_clone().expect("Failed to clone the cient");

    // Spawn the "listening thread"
    thread::spawn(move || {
        loop {
            let mut buf = [0; 1024];
            if let Ok(received) = receiver_client.recv(&mut buf) {
                if let Ok(output) = str::from_utf8(&buf[..received]) {
                    println!("{output}");
                }
            } else {
                println!("Recv function failed");
                break;
            }
        }
    });

    // sending thread
    loop {
        let prompt = format!("{name}> ");
        std::io::stdout().flush().unwrap();

        input.clear();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to get input");

        let message = format!("{}{}", prompt, input.trim());
        client
            .send(message.as_bytes())
            .expect("Failed to transform string to bytes");
    }
}
