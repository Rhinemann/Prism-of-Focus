use crate::Message;
use iced::{Fill, Theme};
use iced::alignment::Horizontal;
use iced::border;
use iced::overlay::menu;
use iced::widget::{container, text, text_input};

pub fn heading(str: &str) -> text::Text<'_> {
    text(str).size(18).center().width(Fill)
}

pub fn centered_column(str: String) -> container::Container<'static, Message> {
    container(text(str)).align_x(Horizontal::Center)
}

pub fn combo_box_menu_style(theme: &Theme) -> menu::Style {
    let mut style = menu::default(theme);
    style.border.radius = border::Radius::new(15);

    style
}

pub fn combo_box_input_style(theme: &Theme, status: text_input::Status) -> text_input::Style {
    let mut style = text_input::default(theme, status);
    style.border.radius = border::Radius::new(15);

    style
}
