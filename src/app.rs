use std::error;

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Estado de la aplicación
#[derive(Debug)]
pub struct App {
    /// Indica si la aplicación debe terminar
    pub should_quit: bool,
    /// Sección actual seleccionada
    pub current_section: Section,
    /// Indica si está en modo comando
    pub command_mode: bool,
    /// Buffer del comando actual
    pub command_buffer: String,
}

/// Secciones principales de la aplicación
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Section {
    Dashboard,
    Profile,
    Projects,
    Experience,
    Skills,
    Contact,
}

impl App {
    /// Crea una nueva instancia de la aplicación
    pub fn new() -> Self {
        Self {
            should_quit: false,
            current_section: Section::Dashboard,
            command_mode: false,
            command_buffer: String::new(),
        }
    }

    /// Maneja los eventos de la aplicación
    pub fn handle_event(&mut self, event: crate::event::Event) -> AppResult<()> {
        if let crate::event::Event::Key(key_event) = event {
            if self.command_mode {
                self.handle_command_mode(key_event)?;
            } else {
                self.handle_normal_mode(key_event)?;
            }
        }
        Ok(())
    }

    /// Maneja eventos en modo normal
    fn handle_normal_mode(&mut self, key_event: crossterm::event::KeyEvent) -> AppResult<()> {
        use crossterm::event::{KeyCode, KeyModifiers};

        match key_event.code {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Char(':') => {
                self.command_mode = true;
                self.command_buffer.clear();
            }
            KeyCode::Char('j') | KeyCode::Down => self.next_section(),
            KeyCode::Char('k') | KeyCode::Up => self.previous_section(),
            _ => {}
        }
        Ok(())
    }

    /// Maneja eventos en modo comando
    fn handle_command_mode(&mut self, key_event: crossterm::event::KeyEvent) -> AppResult<()> {
        use crossterm::event::KeyCode;

        match key_event.code {
            KeyCode::Enter => {
                self.execute_command()?;
                self.command_mode = false;
            }
            KeyCode::Esc => {
                self.command_mode = false;
                self.command_buffer.clear();
            }
            KeyCode::Backspace => {
                self.command_buffer.pop();
            }
            KeyCode::Char(c) => {
                self.command_buffer.push(c);
            }
            _ => {}
        }
        Ok(())
    }

    /// Ejecuta el comando ingresado
    fn execute_command(&mut self) -> AppResult<()> {
        match self.command_buffer.as_str() {
            "q" | "quit" => self.should_quit = true,
            // Aquí añadiremos más comandos en el futuro
            _ => {}
        }
        Ok(())
    }

    /// Cambia a la siguiente sección
    fn next_section(&mut self) {
        self.current_section = match self.current_section {
            Section::Dashboard => Section::Profile,
            Section::Profile => Section::Projects,
            Section::Projects => Section::Experience,
            Section::Experience => Section::Skills,
            Section::Skills => Section::Contact,
            Section::Contact => Section::Dashboard,
        };
    }

    /// Cambia a la sección anterior
    fn previous_section(&mut self) {
        self.current_section = match self.current_section {
            Section::Dashboard => Section::Contact,
            Section::Contact => Section::Skills,
            Section::Skills => Section::Experience,
            Section::Experience => Section::Projects,
            Section::Projects => Section::Profile,
            Section::Profile => Section::Dashboard,
        };
    }
}