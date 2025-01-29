use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::App;

/// Renderiza la interfaz de usuario completa
pub fn render(frame: &mut Frame, app: &App) {
    // Creamos el layout principal
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Barra superior
            Constraint::Min(0),     // Contenido principal
            Constraint::Length(3),  // Barra de estado
        ])
        .split(frame.size());

    // Renderizamos la barra superior
    frame.render_widget(
        Block::default()
            .title(" AndreaTUI v0.1.0 ")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Cyan)),
        chunks[0],
    );

    // Contenido principal (por ahora solo mostramos la sección actual)
    let content = format!("Sección actual: {:?}", app.current_section);
    frame.render_widget(
        Paragraph::new(content)
            .block(Block::default().borders(Borders::ALL)),
        chunks[1],
    );

    // Barra de estado
    let status = if app.command_mode {
        format!(": {}", app.command_buffer)
    } else {
        String::from(" Normal Mode - Presiona ':' para comando, 'q' para salir ")
    };
    frame.render_widget(
        Paragraph::new(status)
            .style(Style::default().bg(Color::Blue).fg(Color::White))
            .block(Block::default().borders(Borders::ALL)),
        chunks[2],
    );
}