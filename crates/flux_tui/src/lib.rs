use ratatui::{widgets::Block, DefaultTerminal, Frame};

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
        let outer_block = Block::bordered().title("Outer");
        let outer_area = frame.area();

        frame.render_widget("hello world", frame.area());
        frame.render_widget(outer_block, outer_area);
    }
}
