use flux_tui::FluxTui;

fn main() -> std::io::Result<()> {
    let tui = FluxTui::default();
    ratatui::run(|terminal| tui.app(terminal))?;
    Ok(())
}
