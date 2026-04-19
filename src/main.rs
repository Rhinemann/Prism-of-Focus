#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod focus;
pub mod styles;

use crate::focus::{Practice, Tenet};
use iced::alignment::Horizontal;
use iced::widget::{column, container, grid, pick_list, scrollable, text, Column};
use iced::{font, window, Element, Fill, Shrink, Size};
use std::collections::HashSet;

type PracticeSet = HashSet<Practice>;

pub fn main() -> iced::Result {
    let mut window_settings = window::Settings::default();
    window_settings.min_size = Some(Size::new(390f32, 400f32));
    window_settings.size = Size::new(500f32, 500f32);
    iced::application(State::new, State::update, State::view)
        .window(window_settings)
        .title("Prism of Focus")
        .run()
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
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

    const TENET_COMBO_PARAMETERS: [TenetParameter<'_>; 7] = [
        TenetParameter::new(
            0,
            "Metaphysical Tenet",
            Message::MetaphysicalSelected as fn(Tenet) -> Message,
            Message::MetaphysicalOpen,
        ),
        TenetParameter::new(
            1,
            "Personal Tenet",
            Message::PersonalSelected,
            Message::PersonalOpen,
        ),
        TenetParameter::new(
            2,
            "Ascension Tenet",
            Message::AscensionSelected,
            Message::AscensionOpen,
        ),
        TenetParameter::new(3, "Role Tenet", Message::RoleSelected, Message::RoleOpen),
        TenetParameter::new(
            4,
            "Epistemology Tenet",
            Message::EpistemologySelected,
            Message::EpistemologyOpen,
        ),
        TenetParameter::new(
            5,
            "Openness Tenet",
            Message::OpennessSelected,
            Message::OpennessOpen,
        ),
        TenetParameter::new(
            6,
            "Afterlife Tenet",
            Message::AfterlifeSelected,
            Message::AfterlifeOpen,
        ),
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
            Message::MetaphysicalSelected(tenet) => {
                self.tenets_chosen[0] = Some(tenet);
                self.update_practices()
            }
            Message::PersonalSelected(tenet) => {
                self.tenets_chosen[1] = Some(tenet);
                self.update_practices()
            }
            Message::AscensionSelected(tenet) => {
                self.tenets_chosen[2] = Some(tenet);
                self.update_practices()
            }
            Message::RoleSelected(tenet) => {
                self.tenets_chosen[3] = Some(tenet);
                self.update_practices()
            }
            Message::EpistemologySelected(tenet) => {
                self.tenets_chosen[4] = Some(tenet);
                self.update_practices()
            }
            Message::OpennessSelected(tenet) => {
                self.tenets_chosen[5] = Some(tenet);
                self.update_practices()
            }
            Message::AfterlifeSelected(tenet) => {
                self.tenets_chosen[6] = Some(tenet);
                self.update_practices()
            }
            Message::MetaphysicalOpen => {
                self.tenets_chosen[0] = None;
                self.update_practices()
            }
            Message::PersonalOpen => {
                self.tenets_chosen[1] = None;
                self.update_practices()
            }
            Message::AscensionOpen => {
                self.tenets_chosen[2] = None;
                self.update_practices()
            }
            Message::RoleOpen => {
                self.tenets_chosen[3] = None;
                self.update_practices()
            }
            Message::EpistemologyOpen => {
                self.tenets_chosen[4] = None;
                self.update_practices()
            }
            Message::OpennessOpen => {
                self.tenets_chosen[5] = None;
                self.update_practices()
            }
            Message::AfterlifeOpen => {
                self.tenets_chosen[6] = None;
                self.update_practices()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let mut bold_font = font::Font::DEFAULT;
        bold_font.weight = font::Weight::Bold;

        let heading = |string| text(string).size(18).center().width(Fill).font(bold_font);
        let centered = |string| container(text(string)).align_x(Horizontal::Center);
        let tenet_box = |tenet_parameter: &TenetParameter| {
            pick_list(
                self.tenet_options[tenet_parameter.index].clone(),
                self.tenets_chosen[tenet_parameter.index].as_ref(),
                tenet_parameter.selected,
            )
            .placeholder(tenet_parameter.name)
            .padding([5, 15])
            .style(styles::tenet_box_style)
            .menu_style(styles::tenet_box_menu_style)
            .font(bold_font)
            .width(Fill)
            .on_open(tenet_parameter.open)
        };

        let practice_grid = if self.tenets_chosen[0..3]
            .iter()
            .all(|option| option.is_some())
        {
            Some(
                grid!(
                    heading("Associated Practices"),
                    heading("Limited Practices"),
                    centered(practice_string(&self.associated_practices)),
                    centered(practice_string(&self.limited_practices)),
                )
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
                .map(|parameter| tenet_box(parameter).into()),
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
}

pub struct TenetParameter<'a> {
    index: usize,
    name: &'a str,
    selected: fn(Tenet) -> Message,
    open: Message,
}

impl<'a> TenetParameter<'a> {
    const fn new(
        index: usize,
        name: &'a str,
        selected: fn(Tenet) -> Message,
        open: Message,
    ) -> Self {
        Self {
            index,
            name,
            selected,
            open,
        }
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
