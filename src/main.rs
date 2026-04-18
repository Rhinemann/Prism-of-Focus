mod focus;

use crate::focus::{Practice, Tenet};
use iced::alignment::Horizontal;
use iced::border::Radius;
use iced::overlay::menu;
use iced::widget::{column, combo_box, container, grid, scrollable, text, text_input};
use iced::Element;
use iced::Length::{Fill, Shrink};
use iced::Theme;
use std::collections::HashSet;

pub fn main() -> iced::Result {
    iced::application(State::new, State::update, State::view)
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

    fn practice_string(practice_set: &HashSet<Practice>) -> String {
        let mut practice_vector = practice_set
            .iter()
            .map(|practice| practice.to_string())
            .collect::<Vec<String>>();
        practice_vector.sort();

        practice_vector.join("\n")
    }

    fn update_practices(&mut self) {
        let full_associated: HashSet<_> = self
            .chosen
            .iter()
            .flatten()
            .map(|tenet| tenet.associated_practices())
            .flatten()
            .collect();
        let full_limited: HashSet<_> = self
            .chosen
            .iter()
            .flatten()
            .map(|tenet| tenet.limited_practices())
            .flatten()
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
        }
    }

    fn combo_box_menu_style(theme: &Theme) -> menu::Style {
        let mut style = menu::default(theme);
        style.border.radius = Radius::new(10);

        style
    }

    fn view(&self) -> Element<'_, Message> {
        let practice_grid = if self.chosen[0..3].iter().all(|option| option.is_some()) {
            Some(
                grid!(
                    text("Associated Practices").center(),
                    text("Limited Practices").center(),
                    container(text(Self::practice_string(&self.associated_practices)))
                        .align_x(Horizontal::Center),
                    container(text(Self::practice_string(&self.limited_practices)))
                        .align_x(Horizontal::Center),
                )
                .spacing(10)
                .height(Shrink)
                .columns(2),
            )
        } else {
            None
        };

        let tenet_grid = column![
            // text("Metaphysical Tenet").width(Length::Shrink),
            combo_box(
                &self.tenet_options[0],
                "Metaphysical Tenet",
                self.chosen[0].as_ref(),
                Message::MetaphysicalSelected
            )
            .input_style(|theme, status| {
                let mut style = text_input::default(theme, status);
                style.border.radius = Radius::new(10);

                style
            })
            .menu_style(Self::combo_box_menu_style),
            // text("Personal Tenet").width(Length::Shrink),
            combo_box(
                &self.tenet_options[1],
                "Personal Tenet",
                self.chosen[1].as_ref(),
                Message::PersonalSelected
            ),
            // text("Ascension Tenet").width(Length::Shrink),
            combo_box(
                &self.tenet_options[2],
                "Ascension Tenet",
                self.chosen[2].as_ref(),
                Message::AscensionSelected
            ),
            // text("Role Tenet").width(Length::Shrink),
            combo_box(
                &self.tenet_options[3],
                "Role Tenet",
                self.chosen[3].as_ref(),
                Message::RoleSelected
            ),
            // text("Epistemology Tenet").width(Length::Shrink),
            combo_box(
                &self.tenet_options[4],
                "Epistemology Tenet",
                self.chosen[4].as_ref(),
                Message::EpistemologySelected
            ),
            // text("Openness Tenet").width(Length::Shrink),
            combo_box(
                &self.tenet_options[5],
                "Openness Tenet",
                self.chosen[5].as_ref(),
                Message::OpennessSelected
            ),
            // text("Afterlife Tenet").width(Length::Shrink),
            combo_box(
                &self.tenet_options[6],
                "Afterlife Tenet",
                self.chosen[6].as_ref(),
                Message::AfterlifeSelected
            ),
        ]
        .spacing(10)
        // .columns(2)
        .height(Shrink);

        scrollable(
            column![
                tenet_grid,
                if let Some(grid) = practice_grid {
                    Element::from(grid)
                } else {
                    text("Pick a Metaphysical, Personal, and Ascension tenet")
                        .width(Fill)
                        .center()
                        .into()
                }
            ]
            .spacing(10),
        )
        .into()
    }
}
