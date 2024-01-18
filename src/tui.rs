use anyhow::Result;
use crossterm::event::{Event as CrosstermEvent, KeyEvent, KeyEventKind};
use futures::{FutureExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::{
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
};

use crate::util::waiting_time_to_sync;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Event {
    Init,
    Quit,
    Error,
    Render,
    Timer,
    Key(KeyEvent),
    Resize(u16, u16),
}

pub struct Tui {
    frame_rate: f64,
    event_rx: UnboundedReceiver<Event>,
    event_tx: UnboundedSender<Event>,
    task: Option<JoinHandle<()>>,
}

impl Tui {
    pub fn new() -> Result<Self> {
        let frame_rate = 60.0;
        let (event_tx, event_rx) = mpsc::unbounded_channel();
        let task = None;
        Ok(Self {
            task,
            event_rx,
            event_tx,
            frame_rate,
        })
    }

    pub fn run(&mut self) {
        let render_delay = std::time::Duration::from_secs_f64(1.0 / self.frame_rate);
        let timer_delay = std::time::Duration::from_secs_f64(1.0);
        let _event_tx = self.event_tx.clone();

        let task = tokio::spawn(async move {
            let mut reader = crossterm::event::EventStream::new();
            let mut render_interval = tokio::time::interval(render_delay);
            let mut timer_interval = tokio::time::interval(timer_delay);
            _event_tx.send(Event::Init).unwrap();

            waiting_time_to_sync();
            loop {
                let render_delay = render_interval.tick();
                let timer_delay = timer_interval.tick();
                let crossterm_event = reader.next().fuse();
                tokio::select! {
                    maybe_event = crossterm_event => {
                        match maybe_event {
                            Some(Ok(evt)) => {
                                match evt {
                                    CrosstermEvent::Key(key) => {
                                        if key.kind == KeyEventKind::Press {
                                            _event_tx.send(Event::Key(key)).unwrap();
                                        }
                                    },
                                    CrosstermEvent::Resize(x, y) => {
                                        _event_tx.send(Event::Resize(x, y)).unwrap();
                                    },
                                    _ => ()
                                }
                            }
                            Some(Err(_)) => {
                                _event_tx.send(Event::Error).unwrap();
                            }
                            None => (),
                        }
                    },
                    _ = render_delay => {
                        _event_tx.send(Event::Render).unwrap();
                    },
                    _ = timer_delay => {
                        _event_tx.send(Event::Timer).unwrap();
                    },
                }
            }
        });

        self.task = Some(task);
    }

    pub async fn next(&mut self) -> Option<Event> {
        self.event_rx.recv().await
    }
}
