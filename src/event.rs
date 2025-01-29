use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, MouseEvent};
use anyhow::Result;

/// Eventos que puede manejar nuestra aplicación
#[derive(Clone, Debug)]
pub enum Event {
    /// Evento de teclado
    Key(KeyEvent),
    /// Evento del mouse
    Mouse(MouseEvent),
    /// Evento de tick para actualizaciones periódicas
    Tick,
}

/// Manejador de eventos que combina eventos del terminal y ticks periódicos
pub struct EventHandler {
    /// Canal receptor de eventos
    rx: mpsc::Receiver<Event>,
    /// Handle del thread de eventos
    _thread_handle: thread::JoinHandle<()>,
}

impl EventHandler {
    /// Crea un nuevo manejador de eventos con un intervalo específico para los ticks
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (tx, rx) = mpsc::channel();

        // Iniciamos un thread dedicado para el polling de eventos
        let _thread_handle = {
            let tx = tx.clone();
            thread::spawn(move || {
                let mut last_tick = std::time::Instant::now();
                loop {
                    // Timeout para el siguiente tick
                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or(tick_rate);

                    // Revisamos si hay eventos del terminal
                    if event::poll(timeout).expect("Error polling eventos") {
                        match event::read().expect("Error leyendo evento") {
                            CrosstermEvent::Key(e) => {
                                if tx.send(Event::Key(e)).is_err() {
                                    return;
                                }
                            }
                            CrosstermEvent::Mouse(e) => {
                                if tx.send(Event::Mouse(e)).is_err() {
                                    return;
                                }
                            }
                            // Ignoramos otros tipos de eventos por ahora
                            _ => {}
                        }
                    }

                    // Emitimos un evento Tick si ha pasado suficiente tiempo
                    if last_tick.elapsed() >= tick_rate {
                        if tx.send(Event::Tick).is_err() {
                            return;
                        }
                        last_tick = std::time::Instant::now();
                    }
                }
            })
        };

        Self {
            rx,
            _thread_handle,
        }
    }

    /// Obtiene el siguiente evento. Este método bloquea hasta que haya un evento disponible.
    pub fn next(&self) -> Result<Event> {
        Ok(self.rx.recv()?)
    }
}