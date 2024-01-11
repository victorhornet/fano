mod app;
mod cli;
mod ui;
use app::Editor;
use color_eyre::eyre::Result;
use crossterm::{
    cursor,
    event::{self, Event, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::io;

fn main() -> Result<()> {
    let args = cli::parse();
    let file = args.file;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, cursor::Hide)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut editor = Editor::new(file)?;
    run_app(&mut terminal, &mut editor)?;

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    terminal.show_cursor()?;
    // println!("{}", editor.text);
    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, editor: &mut Editor) -> Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, editor))?;
        if let Event::Key(event) = event::read()? {
            if matches!(event.kind, KeyEventKind::Press) {
                match event.code {
                    crossterm::event::KeyCode::Char(c) => editor.type_char(c),
                    crossterm::event::KeyCode::Backspace => editor.backspace(),
                    crossterm::event::KeyCode::Enter => editor.appendln(),
                    crossterm::event::KeyCode::Esc => break,
                    _ => {}
                }
            }
        }
    }
    editor.save()?;
    Ok(())
}
