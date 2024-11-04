use ratatui::{
    layout::Rect,
    style::{Style, Styled},
    widgets::{Block, Clear, Paragraph, Widget},
};

/*
TODO:
- [ ] should have a state
- [ ] should provide a state.handle_event() method
- [ ] should handle inner text being bigger than rect
*/
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
        Clear.render(popup_rect, buf);
        let block = Block::bordered().border_type(ratatui::widgets::BorderType::Rounded);
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
