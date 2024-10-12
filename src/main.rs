use std::{env::current_dir, time::Duration};

use clap::Parser;
use color_eyre::{eyre::Context, Result};
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    widgets::Paragraph,
    DefaultTerminal, Frame,
};

#[derive(Parser, Debug, Default)]
#[command(version, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: Option<String>,
}

#[derive(Debug, Default)]
struct App {
    args: Args,
}

impl App {
    pub fn with_args(args: Args) -> Self {
        Self { args }
    }

    pub fn run(&self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|f| self.draw(f))?;
            if self.should_quit()? {
                break;
            }
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let c_dirr = current_dir()
            .expect("should be able to get the current dir")
            .into_os_string();
        let c_dirr = c_dirr.to_str().expect("valid utf8 please");

        let path = self.args.path.as_deref().unwrap_or(c_dirr);

        frame.render_widget(Paragraph::new(path), frame.area());
    }

    fn should_quit(&self) -> Result<bool> {
        if event::poll(Duration::from_millis(250)).context("event poll failed")? {
            if let Event::Key(key) = event::read().context("event read failed")? {
                return Ok(KeyCode::Char('q') == key.code);
            }
        }
        Ok(false)
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();
    let terminal = ratatui::init();
    let app = App::with_args(args);
    let app_result = app.run(terminal).context("app loop failed");
    ratatui::restore();
    app_result
}
