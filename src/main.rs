#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod focus;
pub mod styles;

use crate::focus::Tenet;
use iced::widget::{column, container, grid, pick_list, scrollable, text, Column};
use iced::{alignment, Function};
use iced::{color, font, theme, window, Color, Element, Fill, Shrink, Size, Theme};
use std::collections::HashSet;
use std::{fs, process};

pub fn main() -> iced::Result {
    let window_settings = window::Settings {
        min_size: Some(Size::new(420f32, 400f32)),
        size: Size::new(420f32, 400f32),
        ..window::Settings::default()
    };
    iced::application(State::new, State::update, State::view)
        .window(window_settings)
        .theme(State::theme)
        .title("Prism of Focus")
        .run()
}

#[derive(Debug, Clone)]
pub enum Message {
    TenetSelected(usize, Tenet),
    TenetOpen(usize),
}

struct State {
    associated_practices: HashSet<String>,
    limited_practices: HashSet<String>,
    tenet_options: [Vec<Tenet>; 7],
    tenets_chosen: [Option<Tenet>; 7],
}

impl State {
    pub fn new() -> Self {
        let tenet_string = fs::read_to_string("tenets.json").unwrap_or_else(|e| {
            println!("Error reading file: {e}");
            process::exit(1)
        });

        let tenet_options: [Vec<Tenet>; 7] =
            serde_json::from_str(&tenet_string).unwrap_or_else(|e| {
                println!("Error parsing file: {e}");
                process::exit(1)
            });

        Self {
            associated_practices: HashSet::new(),
            limited_practices: HashSet::new(),
            tenets_chosen: [None, None, None, None, None, None, None],
            tenet_options,
        }
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
        let combine_practices = |f: fn(&Tenet) -> HashSet<String>| {
            self.tenets_chosen.iter().flatten().flat_map(f).collect()
        };
        let full_associated: HashSet<String> =
            combine_practices(|tenet| tenet.associated_practices.clone());
        let full_limited: HashSet<String> =
            combine_practices(|tenet| tenet.limited_practices.clone());

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
            .on_open(Message::TenetOpen(index))
            .width(Fill)
            .padding([5, 15])
            .font(bold_font)
            .style(styles::tenet_box_style)
            .menu_style(styles::tenet_box_menu_style)
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
                .spacing(10)
                .height(Shrink)
                .columns(2),
            )
        } else {
            None
        };

        let tenet_column = Column::with_children(
            Self::TENET_COMBO_PARAMETERS
                .iter()
                .enumerate()
                .map(|(index, name)| tenet_box(index, name).into()),
        )
        .spacing(10)
        .height(Shrink);

        scrollable(
            column![
                tenet_column,
                if let Some(grid) = practice_grid {
                    Element::from(grid)
                } else {
                    heading("Pick a Metaphysical, Personal, and Ascension tenet").into()
                }
            ]
            .spacing(10)
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

fn practice_string(practice_set: &HashSet<String>) -> String {
    let mut practice_vector = practice_set
        .iter()
        .map(|practice| practice.to_string())
        .collect::<Vec<String>>();
    practice_vector.sort();

    practice_vector.join("\n")
}
