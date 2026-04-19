#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod focus;
pub mod styles;

use crate::focus::{Practice, Tenet};
use iced::alignment;
use iced::widget::{column, container, grid, pick_list, scrollable, text, Column};
use iced::{color, font, theme, window, Color, Element, Fill, Shrink, Size, Theme};
use std::collections::HashSet;

type PracticeSet = HashSet<Practice>;

pub fn main() -> iced::Result {
    let mut window_settings = window::Settings::default();
    window_settings.min_size = Some(Size::new(390f32, 400f32));
    window_settings.size = Size::new(500f32, 500f32);
    iced::application(State::new, State::update, State::view)
        .window(window_settings)
        .theme(State::theme)
        .title("Prism of Focus")
        .run()
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    TenetSelected(usize, Tenet),
    TenetOpen(usize),
}

struct State {
    associated_practices: PracticeSet,
    limited_practices: PracticeSet,
    tenet_options: [Vec<Tenet>; 7],
    tenets_chosen: [Option<Tenet>; 7],
}

impl State {
    pub fn new() -> Self {
        let mut res = Self {
            associated_practices: PracticeSet::new(),
            limited_practices: PracticeSet::new(),
            tenets_chosen: [None; 7],
            tenet_options: [
                Tenet::METAPHYSICAL.to_vec(),
                Tenet::PERSONAL.to_vec(),
                Tenet::ASCENSION.to_vec(),
                Tenet::ROLE.to_vec(),
                Tenet::EPISTEMOLOGY.to_vec(),
                Tenet::OPENNESS.to_vec(),
                Tenet::AFTERLIFE.to_vec(),
            ],
        };

        res.update_practices();

        res
    }

    const TENET_COMBO_PARAMETERS: [&str; 7] = [
        "Metaphysical",
        "Personal",
        "Ascension",
        "Role",
        "Epistemology",
        "Openness",
        "Afterlife",
    ];

    fn update_practices(&mut self) {
        let full_associated: PracticeSet = self
            .tenets_chosen
            .iter()
            .flatten()
            .flat_map(|tenet| tenet.associated_practices())
            .collect();
        let full_limited: PracticeSet = self
            .tenets_chosen
            .iter()
            .flatten()
            .flat_map(|tenet| tenet.limited_practices())
            .collect();

        self.associated_practices = &full_associated - &full_limited;
        self.limited_practices = &full_limited - &full_associated;
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::TenetSelected(index, tenet) => {
                self.tenets_chosen[index] = Some(tenet);
                self.update_practices()
            }
            Message::TenetOpen(index) => {
                self.tenets_chosen[index] = None;
                self.update_practices()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let mut bold_font = font::Font::DEFAULT;
        bold_font.weight = font::Weight::Bold;

        let heading = |string| text(string).size(18).center().width(Fill).font(bold_font);
        let centered = |string| container(text(string)).align_x(alignment::Horizontal::Center);
        let tenet_box = |index: usize, name: &str| {
            pick_list(
                self.tenet_options[index].clone(),
                self.tenets_chosen[index].as_ref(),
                Message::TenetSelected.with(index),
            )
            .placeholder(format!("Choose {} Tenet", name))
            .padding([5, 15])
            .style(styles::tenet_box_style)
            .menu_style(styles::tenet_box_menu_style)
            .font(bold_font)
            .width(Fill)
            .on_open(Message::TenetOpen(index))
            .into()
        };

        let practice_grid = if self.tenets_chosen[0..3]
            .iter()
            .all(|option| option.is_some())
        {
            Some(
                grid![
                    heading("Associated Practices"),
                    heading("Limited Practices"),
                    centered(practice_string(&self.associated_practices)),
                    centered(practice_string(&self.limited_practices)),
                ]
                .spacing(15)
                .height(Shrink)
                .columns(2),
            )
        } else {
            None
        };

        let tenet_grid = Column::with_children(
            Self::TENET_COMBO_PARAMETERS
                .iter()
                .enumerate()
                .map(|(index, parameter)| tenet_box(index, parameter)),
        )
        .spacing(15)
        .height(Shrink);

        scrollable(
            column![
                tenet_grid,
                if let Some(grid) = practice_grid {
                    Element::from(grid)
                } else {
                    heading("Pick a Metaphysical, Personal, and Ascension tenet").into()
                }
            ]
            .spacing(15)
            .padding(15),
        )
        .into()
    }

    fn theme(&self) -> Theme {
        Theme::custom(
            String::from("Custom"),
            theme::Palette {
                background: Color::WHITE,
                primary: color!(0x4B0082),
                text: color!(0x4B0082),
                success: color!(0xFFD700),
                warning: color!(0xCC2936),
                danger: color!(0x9A031E),
            },
        )
    }
}

fn practice_string(practice_set: &PracticeSet) -> String {
    let mut practice_vector = practice_set
        .iter()
        .map(|practice| practice.to_string())
        .collect::<Vec<String>>();
    practice_vector.sort();

    practice_vector.join("\n")
}
