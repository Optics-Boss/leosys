use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    widgets::*,
};
use std::io::{stdout, Result};
use std::process::{Command, Stdio};

fn main() -> Result<()> {
    get_windows_system_information();

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;


    let os_info = ["Item 1", "Item 2", "Item 3"];

    let os_info_list = List::new(os_info)
                .block(Block::bordered().title("List"))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
                .highlight_symbol(">>")
                .repeat_highlight_symbol(true);

    loop {
        terminal.draw(|frame| {
            let os_info_list = os_info_list.clone();

            let area = frame.size();
            frame.render_widget(
                os_info_list,
                area,
            );
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn get_windows_system_information() -> Vec<String> {
    let str;

    let system_info = Command::new("systeminfo")
        .stdout(Stdio::piped())
        .output()
        .unwrap();

    str = match std::str::from_utf8(&system_info.stdout) {
        Ok(val) => val,
        Err(_) => panic!("No system info found"),
    };

    let parts = str.split("\r\n");
    let mut result : Vec<String> = vec![];

    for part in parts {
        result.push(part.to_string())
    }

    return result;
}
