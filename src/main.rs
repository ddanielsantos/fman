use std::{env::current_dir, time::Duration};

use clap::Parser;
use color_eyre::{eyre::Context, Result};
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    widgets::Paragraph,
    DefaultTerminal, Frame,
};

#[derive(Parser, Debug)]
#[command(version, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,
}

#[derive(Debug, Default)]
struct App {
    path: String,
}

impl App {
    pub fn with_args(args: Args) -> Self {
        Self { path: args.path }
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
        let p = current_dir().expect("to get the current dir");

        frame.render_widget(
            Paragraph::new(format!("{:?}", p.into_os_string())),
            frame.area(),
        );
        frame.render_widget(Paragraph::new(self.path.as_str()), frame.area());
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
    let args = Args::parse();
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app = App::with_args(args);
    let app_result = app.run(terminal).context("app loop failed");
    ratatui::restore();
    app_result
}
