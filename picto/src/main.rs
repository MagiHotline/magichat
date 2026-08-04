use std::thread;

use client::client::Client;

fn main() {
    let client = Client::new().expect("Failed to create a new Client");

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

    client
        .connect(&endpoint)
        .expect("Failed to connect to the address");

    println!(
        "Connected successfully to {}! Start sending and receiving messages",
        endpoint
    );

    let mut input = String::new();
    loop {
        /*  Listening Thread
        thread::spawn(|| {
            let mut buf = [0; 10];
            match client.recv(&mut buf) {
                Ok(received) => println!("Received {received} bytes {:?}", &buf[..received]),
                Err(e) => println!("recv function failed: {e:?}"),
            }
        });
        */

        println!("{} >> ", name);
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to get input");

        client
            .send(input.as_bytes())
            .expect("Failed to transform string to bytes");
    }
}
