use std::time::Duration;

use crossterm::event::{Event as CrosstermEvent, EventStream, KeyEvent};
use futures::StreamExt;

#[derive(Debug)]
pub enum Event {
    Key(KeyEvent),
    Tick,
}

pub struct EventHandler {
    rx: tokio::sync::mpsc::UnboundedReceiver<Event>,
    _task: tokio::task::JoinHandle<()>,
}

impl EventHandler {
    pub fn new(tick_rate: Duration) -> Self {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

        let task = tokio::spawn(async move {
            let mut event_stream = EventStream::new();
            let mut tick_interval = tokio::time::interval(tick_rate);

            loop {
                tokio::select! {
                    _ = tick_interval.tick() => {
                        if tx.send(Event::Tick).is_err() {
                            break;
                        }
                    }
                    event = event_stream.next() => {
                        match event {
                            Some(Ok(CrosstermEvent::Key(key))) => {
                                if tx.send(Event::Key(key)).is_err() {
                                    break;
                                }
                            }
                            Some(Ok(_)) => {}
                            Some(Err(_)) => break,
                            None => break,
                        }
                    }
                }
            }
        });

        Self { rx, _task: task }
    }

    pub async fn next(&mut self) -> Option<Event> {
        self.rx.recv().await
    }
}
