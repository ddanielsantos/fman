use ratatui::{
    layout::Rect,
    widgets::{Block, Clear, Paragraph, Widget},
};

pub struct InputPopup<'a> {
    input: &'a crate::Input,
}
impl<'a> InputPopup<'a> {
    pub fn new(input: &'a crate::Input) -> Self {
        Self { input }
    }

    pub fn get_cursor_position(&self, area: Rect) -> ratatui::prelude::Position {
        let popup_rect = Self::get_popup_rect(area);

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
            height: area.height / 3,
        }
    }
}

impl Widget for InputPopup<'_> {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let popup_rect = Self::get_popup_rect(area);
        Clear.render(popup_rect, buf);
        let block = Block::bordered()
            .border_type(ratatui::widgets::BorderType::Rounded)
            .title_top("create item");
        Paragraph::new(&*self.input.text)
            .block(block)
            .render(popup_rect, buf);
    }
}
