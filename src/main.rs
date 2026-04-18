#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod components;
mod focus;

use crate::focus::{Practice, Tenet};
use iced::widget::{column, combo_box, grid, scrollable, Column};
use iced::Element;
use iced::Length::{Fill, Shrink};
use std::collections::HashSet;

pub fn main() -> iced::Result {
    let mut settings = iced::window::Settings::default();
    settings.max_size = Some(iced::Size::new(500f32, 0f32));
    settings.size = iced::Size::new(500f32, 500f32);
    iced::application(State::new, State::update, State::view)
        .window(settings)
        .title("Prism of Focus")
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    MetaphysicalSelected(Tenet),
    PersonalSelected(Tenet),
    AscensionSelected(Tenet),
    RoleSelected(Tenet),
    EpistemologySelected(Tenet),
    OpennessSelected(Tenet),
    AfterlifeSelected(Tenet),
    MetaphysicalOpen,
    PersonalOpen,
    AscensionOpen,
    RoleOpen,
    EpistemologyOpen,
    OpennessOpen,
    AfterlifeOpen,
}

struct State {
    associated_practices: HashSet<Practice>,
    limited_practices: HashSet<Practice>,
    tenet_options: [combo_box::State<Tenet>; 7],
    chosen: [Option<Tenet>; 7],
}

impl State {
    pub fn new() -> Self {
        Self {
            associated_practices: HashSet::new(),
            limited_practices: HashSet::new(),
            chosen: [None; 7],
            tenet_options: [
                combo_box::State::new(Tenet::METAPHYSICAL.to_vec()),
                combo_box::State::new(Tenet::PERSONAL.to_vec()),
                combo_box::State::new(Tenet::ASCENSION.to_vec()),
                combo_box::State::new(Tenet::ROLE.to_vec()),
                combo_box::State::new(Tenet::EPISTEMOLOGY.to_vec()),
                combo_box::State::new(Tenet::OPENNESS.to_vec()),
                combo_box::State::new(Tenet::AFTERLIFE.to_vec()),
            ],
        }
    }

    const TENET_COMBO_PARAMETERS: [TenetParameter<'_>; 7] = [
        TenetParameter(
            0,
            "Metaphysical Tenet",
            Message::MetaphysicalSelected as fn(Tenet) -> Message,
            Message::MetaphysicalOpen,
        ),
        TenetParameter(
            1,
            "Personal Tenet",
            Message::PersonalSelected,
            Message::PersonalOpen,
        ),
        TenetParameter(
            2,
            "Ascension Tenet",
            Message::AscensionSelected,
            Message::AscensionOpen,
        ),
        TenetParameter(3, "Role Tenet", Message::RoleSelected, Message::RoleOpen),
        TenetParameter(
            4,
            "Epistemology Tenet",
            Message::EpistemologySelected,
            Message::EpistemologyOpen,
        ),
        TenetParameter(
            5,
            "Openness Tenet",
            Message::OpennessSelected,
            Message::OpennessOpen,
        ),
        TenetParameter(
            6,
            "Afterlife Tenet",
            Message::AfterlifeSelected,
            Message::AfterlifeOpen,
        ),
    ];

    fn update_practices(&mut self) {
        let full_associated: HashSet<_> = self
            .chosen
            .iter()
            .flatten()
            .flat_map(|tenet| tenet.associated_practices())
            .collect();
        let full_limited: HashSet<_> = self
            .chosen
            .iter()
            .flatten()
            .flat_map(|tenet| tenet.limited_practices())
            .collect();

        self.associated_practices = &full_associated - &full_limited;
        self.limited_practices = &full_limited - &full_associated;
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::MetaphysicalSelected(tenet) => {
                self.chosen[0] = Some(tenet);
                self.update_practices()
            }
            Message::PersonalSelected(tenet) => {
                self.chosen[1] = Some(tenet);
                self.update_practices()
            }
            Message::AscensionSelected(tenet) => {
                self.chosen[2] = Some(tenet);
                self.update_practices()
            }
            Message::RoleSelected(tenet) => {
                self.chosen[3] = Some(tenet);
                self.update_practices()
            }
            Message::EpistemologySelected(tenet) => {
                self.chosen[4] = Some(tenet);
                self.update_practices()
            }
            Message::OpennessSelected(tenet) => {
                self.chosen[5] = Some(tenet);
                self.update_practices()
            }
            Message::AfterlifeSelected(tenet) => {
                self.chosen[6] = Some(tenet);
                self.update_practices()
            }
            Message::MetaphysicalOpen => {
                self.chosen[0] = None;
                self.update_practices()
            }
            Message::PersonalOpen => {
                self.chosen[1] = None;
                self.update_practices()
            }
            Message::AscensionOpen => {
                self.chosen[2] = None;
                self.update_practices()
            }
            Message::RoleOpen => {
                self.chosen[3] = None;
                self.update_practices()
            }
            Message::EpistemologyOpen => {
                self.chosen[4] = None;
                self.update_practices()
            }
            Message::OpennessOpen => {
                self.chosen[5] = None;
                self.update_practices()
            }
            Message::AfterlifeOpen => {
                self.chosen[6] = None;
                self.update_practices()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let practice_grid = if self.chosen[0..3].iter().all(|option| option.is_some()) {
            Some(
                grid!(
                    components::heading("Associated Practices"),
                    components::heading("Limited Practices"),
                    components::centered_column(practice_string(&self.associated_practices)),
                    components::centered_column(practice_string(&self.limited_practices)),
                )
                .spacing(15)
                .height(Shrink)
                .columns(2),
            )
        } else {
            None
        };

        let mut tenet_grid = Column::new().spacing(15).height(Shrink);

        for parameters in Self::TENET_COMBO_PARAMETERS {
            tenet_grid = tenet_grid.push(
                combo_box(
                    &self.tenet_options[parameters.0],
                    parameters.1,
                    self.chosen[parameters.0].as_ref(),
                    parameters.2,
                )
                .size(18)
                .padding([5, 15])
                .input_style(components::combo_box_input_style)
                .menu_style(components::combo_box_menu_style)
                .on_open(parameters.3),
            );
        }

        scrollable(
            column![
                tenet_grid,
                if let Some(grid) = practice_grid {
                    Element::from(grid)
                } else {
                    components::heading("Pick a Metaphysical, Personal, and Ascension tenet")
                        .width(Fill)
                        .into()
                }
            ]
            .spacing(15)
            .max_width(500)
            .padding(15),
        )
        .into()
    }
}

struct TenetParameter<'a>(usize, &'a str, fn(Tenet) -> Message, Message);

fn practice_string(practice_set: &HashSet<Practice>) -> String {
    let mut practice_vector = practice_set
        .iter()
        .map(|practice| practice.to_string())
        .collect::<Vec<String>>();
    practice_vector.sort();

    practice_vector.join("\n")
}
