use ratatui::{DefaultTerminal, Frame};

#[derive(Debug, Default)]
pub struct FluxTui {}

impl FluxTui {
    pub fn app(&self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        loop {
            terminal.draw(FluxTui::render)?;
            if crossterm::event::read()?.is_key_press() {
                break Ok(());
            }
        }
    }

    fn render(frame: &mut Frame) {
        frame.render_widget("hello world", frame.area());
    }
}
