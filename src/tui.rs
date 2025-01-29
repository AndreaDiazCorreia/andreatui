use std::io::{self, Stdout};
use crossterm::{
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    execute,
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use anyhow::Result;

use crate::app::App;
use crate::ui;

/// Estructura que maneja el terminal y su configuración
pub struct Tui {
    /// Terminal de ratatui
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Tui {
    /// Crea una nueva instancia de Tui
    pub fn new() -> Result<Self> {
        // Configuramos el terminal
        let mut stdout = io::stdout();
        
        // Entramos a la pantalla alternativa y activamos el modo raw
        execute!(stdout, EnterAlternateScreen)?;
        terminal::enable_raw_mode()?;

        // Creamos el backend y el terminal
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        Ok(Self { terminal })
    }

    /// Dibuja la interfaz de usuario
    pub fn draw(&mut self, app: &mut App) -> Result<()> {
        self.terminal.draw(|frame| {
            ui::render(frame, app);
        })?;
        Ok(())
    }

    /// Restaura el terminal a su estado original
    pub fn restore(&mut self) -> Result<()> {
        // Deshabilitamos el modo raw y salimos de la pantalla alternativa
        terminal::disable_raw_mode()?;
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen)?;
        Ok(())
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        // Restauramos el terminal incluso si ocurre un panic
        if let Err(e) = self.restore() {
            eprintln!("Error restaurando el terminal: {}", e);
        }
    }
}