use std::env;
use std::io::{self, Stdout};
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, Paragraph};
use tui::Terminal;

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let username = whoami::username();
    let hostname = whoami::hostname();
    let prompt = prompt(username, hostname);

    terminal.clear()?;

    render(&mut terminal, &prompt)?;

    Ok(())
}

fn render(terminal: &mut Terminal<CrosstermBackend<Stdout>>, text: &str) -> Result<(), io::Error> {
    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(f.size());
        let block = Block::default().title("NeoSH").borders(Borders::ALL);
        let paragraph = Paragraph::new(text).block(block);
        f.render_widget(paragraph, chunks[0]);
    })?;
    Ok(())
}

fn prompt(username: String, hostname: String) -> String {
    let cwd = env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();

    let cwd = cwd.replace(&{
        #[cfg(not(target_os = "windows"))]
        let homedir = env::var("HOME").unwrap();

        #[cfg(target_os = "windows")]
        let homedir = env::var("USERPROFILE").unwrap();

        homedir
    }, "~");

    format!("{}@{} {} > ", username, hostname, cwd)
}
