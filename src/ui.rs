use crate::fs::get_delimiter;
use ratatui::{
    layout::Rect,
    style::{palette::tailwind::SLATE, Modifier, Style, Styled},
    widgets::{Block, Clear, List, ListState, Paragraph, StatefulWidget, Widget},
};

pub const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

pub struct Input<'a> {
    input: &'a crate::Input,
    style: Style,
    popup: bool,
}

impl<'a> Input<'a> {
    pub fn new(input: &'a crate::Input) -> Self {
        Self {
            input,
            style: Style::default(),
            popup: false,
        }
    }

    pub fn style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = style.into();
        self
    }

    pub fn popup(mut self) -> Self {
        self.popup = true;
        self
    }

    pub fn get_cursor_position(&self, area: Rect) -> ratatui::prelude::Position {
        let popup_rect = if self.popup {
            Self::get_popup_rect(area)
        } else {
            area
        };

        ratatui::layout::Position::new(
            popup_rect.x + self.input.char_index as u16 + 1,
            popup_rect.y + 1,
        )
    }

    fn get_popup_rect(area: Rect) -> Rect {
        Rect {
            x: area.width / 4,
            y: area.height / 3,
            width: area.width / 2,
            height: 3,
        }
    }
}

impl Widget for Input<'_> {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let popup_rect = if self.popup {
            Self::get_popup_rect(area)
        } else {
            area
        };
        let delimiter = get_delimiter();

        Clear.render(popup_rect, buf);
        let block = Block::bordered()
            .title_top(format!("create item or folders ({} ended)", delimiter))
            .border_type(ratatui::widgets::BorderType::Rounded);
        Paragraph::new(self.input.text.as_str())
            .block(block)
            .render(popup_rect, buf);
    }
}
impl<'a> Styled for Input<'a> {
    type Item = Self;

    fn style(&self) -> Style {
        self.style
    }

    fn set_style<S: Into<Style>>(self, style: S) -> Self::Item {
        self.style(style)
    }
}

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
