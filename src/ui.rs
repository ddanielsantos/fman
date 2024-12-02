use ratatui::{
    layout::Rect,
    style::{palette::tailwind::SLATE, Modifier, Style},
    widgets::{Block, Clear, List, ListState, StatefulWidget, Widget},
};

pub const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

pub struct MainList {
    current_path: String,
    current_path_content: Vec<String>,
}

impl MainList {
    pub fn new(current_path: String, current_path_content: Vec<String>) -> Self {
        Self {
            current_path,
            current_path_content,
        }
    }
}

impl StatefulWidget for MainList {
    type State = ListState;

    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {
        let block = Block::bordered()
            .title(self.current_path)
            .border_type(ratatui::widgets::BorderType::Rounded);
        StatefulWidget::render(
            List::new(self.current_path_content)
                .block(block)
                .highlight_style(SELECTED_STYLE)
                .highlight_spacing(ratatui::widgets::HighlightSpacing::Always),
            area,
            buf,
            state,
        )
    }
}

pub struct CommandPicker {
    commands: Vec<String>,
}

impl CommandPicker {
    pub fn new(commands: Vec<String>) -> Self {
        Self { commands }
    }

    fn get_rect(area: &Rect) -> Rect {
        Rect {
            x: area.width / 4,
            y: area.height / 3,
            width: area.width / 2,
            height: area.height / 3,
        }
    }
}

impl StatefulWidget for CommandPicker {
    type State = ListState;

    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State)
    where
        Self: Sized,
    {
        let rect = Self::get_rect(&area);
        Clear.render(rect, buf);

        let block = Block::bordered()
            .title("command list")
            .border_type(ratatui::widgets::BorderType::Rounded);

        StatefulWidget::render(
            List::new(self.commands)
                .block(block)
                .highlight_style(SELECTED_STYLE)
                .highlight_spacing(ratatui::widgets::HighlightSpacing::Always),
            rect,
            buf,
            state,
        )
    }
}
