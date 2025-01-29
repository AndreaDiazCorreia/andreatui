use std::io;
use anyhow::Result;
use andreatui::app::{App, AppResult};
use andreatui::event::EventHandler;
use andreatui::tui::Tui;

fn main() -> Result<()> {
    // Inicializar el manejador de errores
    color_eyre::install()?;

    // Crear la aplicación y el manejador de eventos
    let mut app = App::new();
    let mut tui = Tui::new()?;
    let events = EventHandler::new(250); // 250ms de timeout para eventos

    // Ejecutar el loop principal
    while !app.should_quit {
        // Dibujar la UI
        tui.draw(&mut app)?;

        // Manejar eventos
        if let Ok(event) = events.next() {
            app.handle_event(event)?;
        }
    }

    // Restaurar el terminal
    tui.restore()?;
    Ok(())
}