fn main() -> std::io::Result<()> {
    let app = core::FluxCore::new();
    ratatui::run(|terminal| app.run(terminal))?;
    Ok(())
}
