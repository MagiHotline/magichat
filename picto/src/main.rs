use color_eyre::eyre::Result;
use ratatui::{Terminal, backend::CrosstermBackend};

use crate::{
    app::App,
    event::{Event, EventHandler},
    tui::Tui,
    update::update,
};

pub mod app;
pub mod event;
pub mod tui;
pub mod ui;
pub mod update;

fn main() -> Result<()> {
    color_eyre::install()?; // install the panic hooks of eyre

    let mut app = App::new();
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250); // a tick is every 250 ms
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    while !app.should_quit {
        tui.draw(&mut app)?;

        match tui.events.next()? {
            Event::Tick => {}
            Event::Key(key_event) => update(&mut app, key_event),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        };
    }

    /*
    // Bind to a socket
    println!("First, bind your socket to an address (X.X.X.X:YYYY): ");
    let mut bind_address = String::new();
    std::io::stdin()
        .read_line(&mut bind_address)
        .expect("Failed to get input");

    // Get the name of the client
    println!("What is your name?");
    let mut name = String::new();
    std::io::stdin()
        .read_line(&mut name)
        .expect("Failed to get input");

    let client = Client::new(name.trim().to_string(), bind_address.trim().to_string())
        .expect("Failed to create a new Client");

    println!("Insert the address where you want to connect (X.X.X.X:YYYY): ");
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
            let mut buf = [0; 8192];
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
        let prompt = format!("{}> ", client.name());
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
    */

    tui.exit()?;
    Ok(())
}
