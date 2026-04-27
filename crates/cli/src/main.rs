use ratatui::prelude::*;
use ratatui::widgets::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = ratatui::init();

    let result = run_app(&mut terminal);

    ratatui::restore();

    result
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        terminal.draw(|f| {
            let size = f.area();
            let block = Block::default()
                .title("chromamancer")
                .borders(Borders::ALL);
            f.render_widget(block, size);
        })?;

        if crossterm::event::poll(std::time::Duration::from_millis(250))? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                if key.kind == crossterm::event::KeyEventKind::Press {
                    match key.code {
                        crossterm::event::KeyCode::Char('q') => break,
                        _ => {}
                    }
                }
            }
        }
    }

    Ok(())
}
