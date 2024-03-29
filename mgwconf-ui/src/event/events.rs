use std::{thread, time::Duration};

use crate::event::Key;
use crossterm::event;
use std::sync::mpsc;

#[derive(Debug, Clone, Copy)]
/// Configuration for event handling.
pub struct EventConfig {
    /// The key that is used to exit the application.
    pub exit_key: Key,
    /// The tick rate at which the application will sent an tick event.
    pub tick_rate: Duration,
}

impl Default for EventConfig {
    fn default() -> EventConfig {
        EventConfig {
            exit_key: Key::Ctrl('c'),
            tick_rate: Duration::from_millis(250),
        }
    }
}

#[derive(Debug)]
/// An occurred event.
pub enum Event<I> {
    /// An input event occurred.
    Input(I),
    /// An tick event occurred.
    Tick,
}

/// A small event handler that wrap crossterm input and tick event. Each event
/// type is handled in its own thread and returned to a common `Receiver`
pub struct Events {
    rx: mpsc::Receiver<Event<Key>>,
    // Need to be kept around to prevent disposing the sender side.
    _tx: mpsc::Sender<Event<Key>>,
}

impl Events {
    /// Constructs an new instance of `Events` with the default config.
    pub fn new(tick_rate: u64) -> Events {
        Events::with_config(EventConfig {
            tick_rate: Duration::from_millis(tick_rate),
            ..Default::default()
        })
    }

    /// Constructs an new instance of `Events` from given config.
    pub fn with_config(config: EventConfig) -> Events {
        let (tx, rx) = mpsc::channel();

        let event_tx = tx.clone();
        thread::spawn(move || {
            loop {
                // poll for tick rate duration, if no event, sent tick event.
                if event::poll(config.tick_rate).unwrap() {
                    if let event::Event::Key(key) = event::read().unwrap() {
                        let key = Key::from(key);

                        event_tx.send(Event::Input(key)).unwrap();
                    }
                }
                event_tx.send(Event::Tick).unwrap();
            }
        });

        Events { rx, _tx: tx }
    }
}

impl Iterator for Events {
    type Item = Result<Event<Key>, mpsc::RecvError>;

    fn next(&mut self) -> Option<Result<Event<Key>, mpsc::RecvError>> {
        Some(self.rx.recv())
    }
}
