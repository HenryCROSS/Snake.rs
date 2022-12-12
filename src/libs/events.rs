use std::sync::mpsc::{channel, Receiver, Sender};

use std::thread::{self, JoinHandle};
use std::time::Duration;

use crossterm::event::{poll, read, Event, self};

pub enum Input_Events {
    Input(event::Event),
    Tick,
}

pub struct Events_listener {
    handle: Option<JoinHandle<()>>,
    rx: Receiver<Input_Events>,
    _tx: Sender<Input_Events>,
}

impl Events_listener {
    pub fn new(tick_rate: u64) -> Events_listener {
        let (tx, rx) = channel();
        let event_tx = tx.clone();

        let handle = Some(thread::spawn(move || {
                    loop {
                        // `poll()` waits for an `Event` for a given time period
                        if poll(Duration::from_millis(tick_rate)).unwrap() {
                            // HACK: What happend if nothing is getting inside
                            event_tx.send(Input_Events::Input(read().unwrap())).expect("Something happened from event_tx.send()");
                        } else {
                            event_tx.send(Input_Events::Tick).expect("Err happened from sent nothing");
                        }
                    }
                }));

        Events_listener { handle, rx, _tx: tx }
    }

    pub fn next(&self) -> Result<Input_Events, std::sync::mpsc::RecvError> {
        self.rx.recv()
    }
}

pub struct Event_ops {
    event: Option<Events_listener>,
    tick_rate: u64,
}

impl Event_ops {
    pub fn new(tick_rate: u64) -> Event_ops {
        Event_ops { event: None, tick_rate }
    }

    pub fn exec_event(&mut self){
        if let None = self.event {
            self.event = Some(Events_listener::new(self.tick_rate));
        }
    }

    pub fn get_event_handle(&mut self) -> Option<&mut Events_listener> {
        self.exec_event();

        if let Some(handle) = &mut self.event {
            Some(handle)
        } else {
            None
        }
    }
}
