use std::sync::mpsc::{channel, Receiver, Sender};

use std::thread;
use std::time::Duration;

use crossterm::event::{poll, read, Event};

pub struct Events {
    rx: Receiver<Event>,
    _tx: Sender<Event>,
}

impl Events {
    pub fn new(tick_rate: u64) -> Events {
        let (tx, rx) = channel();
        let event_tx = tx.clone();

        thread::spawn(move || {
            loop {
                // `poll()` waits for an `Event` for a given time period
                if poll(Duration::from_millis(tick_rate)).unwrap() {
                    event_tx.send(read().unwrap()).expect("Something happened from event_tx.send()");
                } else {
                    panic!()
                }
            }
        });

        Events { rx, _tx: tx }
    }

    pub fn next(&self) -> Result<Event, std::sync::mpsc::RecvError> {
        self.rx.recv()
    }
}
