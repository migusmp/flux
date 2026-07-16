use flux_tui::FluxTui;
use ratatui::DefaultTerminal;

#[derive(Debug, Default)]
pub struct FluxCore {
    flux_tui: FluxTui,
}

impl FluxCore {
    pub fn new() -> Self {
        FluxCore {
            flux_tui: FluxTui::default(),
        }
    }

    pub fn run(&self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        self.flux_tui.app(terminal)?;
        Ok(())
    }
}
