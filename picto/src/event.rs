use color_eyre::Result;
use ratatui::crossterm::event::{self, Event as CrosstermEvent, KeyEvent, MouseEvent};
use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::{Duration, Instant},
};

/// Terminal events.
#[derive(Debug, Copy, Clone)]
pub enum Event {
    /// Terminal tick.
    Tick,
    /// Key press.
    Key(KeyEvent),
    /// Mouse click/scroll.
    Mouse(MouseEvent),
    /// Terminal resize.
    Resize(u16, u16),
}

/*
 * Using MPSC (Multiple Producer Single Consumer) Channel for sending and receiving data
 * between threads safely without using locks or mutexes. Multiple threads can send data to this channel
 * but only can retrieve and process this data
 */
/// Terminal event handler.
#[derive(Debug)]
pub struct EventHandler {
    #[allow(dead_code)]
    sender: mpsc::Sender<Event>,
    #[allow(dead_code)]
    receiver: mpsc::Receiver<Event>,
    #[allow(dead_code)]
    handler: thread::JoinHandle<()>,
}

impl EventHandler {
    /// Constructs a new instance of [`EventHandler`]
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver): (Sender<Event>, Receiver<Event>) = mpsc::channel();
        let handler = {
            let sender = sender.clone();
            thread::spawn(move || {
                let mut last_tick = Instant::now();
                loop {
                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or(tick_rate);

                    if event::poll(timeout).expect("Unable to poll an event.") {
                        match event::read().expect("Unable to read an event") {
                            CrosstermEvent::Key(e) => {
                                if e.kind == event::KeyEventKind::Press {
                                    sender.send(Event::Key(e))
                                } else {
                                    Ok(())
                                }
                            }
                            CrosstermEvent::Mouse(e) => sender.send(Event::Mouse(e)),
                            CrosstermEvent::Resize(x, y) => sender.send(Event::Resize(x, y)),
                            _ => unimplemented!(),
                        }
                        .expect("Unable to send to terminal event")
                    }

                    if last_tick.elapsed() >= tick_rate {
                        sender.send(Event::Tick).expect("Failed to send tick event");
                        last_tick = Instant::now();
                    }
                }
            })
        };

        Self {
            sender,
            receiver,
            handler,
        }
    }

    /// Receive the next event from the handler thread.
    ///
    /// This function will always block the current thread if
    /// there is no data available and it's possible for more data to be sent.
    pub fn next(&self) -> Result<Event> {
        Ok(self.receiver.recv()?)
    }
}
