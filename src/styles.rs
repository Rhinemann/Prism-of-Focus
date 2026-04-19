use iced::border;
use iced::overlay::menu;
use iced::widget::pick_list;
use iced::Theme;

const TENET_BOX_BORDER_RADIUS: u32 = 12;

pub fn tenet_box_menu_style(theme: &Theme) -> menu::Style {
    let mut style = menu::default(theme);
    style.border.radius = border::Radius::new(TENET_BOX_BORDER_RADIUS);

    style
}

pub fn tenet_box_style(theme: &Theme, status: pick_list::Status) -> pick_list::Style {
    let mut style = pick_list::default(theme, status);
    style.border.radius = border::Radius::new(TENET_BOX_BORDER_RADIUS);

    style
}
