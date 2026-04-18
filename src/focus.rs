use std::collections::HashSet;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Practice {
    Alchemy,
    Animalism,
    Appropriation,
    ArtOfDesire,
    Bardism,
    ChaosMagick,
    Charity,
    Craftwork,
    CrazyWisdom,
    Cybernetics,
    Dominion,
    Elementalism,
    Faith,
    GodBonding,
    GutterMagick,
    HighRitualMagick,
    Hypertech,
    Investment,
    Invigoration,
    Maleficia,
    MartialArts,
    MediaControl,
    MedicineWork,
    Mediumship,
    Psionics,
    RealityHacking,
    Shamanism,
    Voudoun,
    WeirdScience,
    Witchcraft,
    Yoga,
}

impl Display for Practice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Practice::Alchemy => "Alchemy",
            Practice::Animalism => "Animalism",
            Practice::Appropriation => "Appropriation",
            Practice::ArtOfDesire => "Art of Desire",
            Practice::Bardism => "Bardism",
            Practice::ChaosMagick => "Chaos Magick",
            Practice::Charity => "Charity",
            Practice::Craftwork => "Craftwork",
            Practice::CrazyWisdom => "Crazy Wisdom",
            Practice::Cybernetics => "Cybernetics",
            Practice::Dominion => "Dominion",
            Practice::Elementalism => "Elementalism",
            Practice::Faith => "Faith",
            Practice::GodBonding => "God-Bonding",
            Practice::GutterMagick => "Gutter Magick",
            Practice::HighRitualMagick => "High Ritual Magick",
            Practice::Hypertech => "Hypertech",
            Practice::Investment => "Investment",
            Practice::Invigoration => "Invigotarion",
            Practice::Maleficia => "Maleficia",
            Practice::MartialArts => "Martial Arts",
            Practice::MediaControl => "Media Control",
            Practice::MedicineWork => "Medicine Work",
            Practice::Mediumship => "Mediumship",
            Practice::Psionics => "Prionics",
            Practice::RealityHacking => "Reality Hacking",
            Practice::Shamanism => "Shamanism",
            Practice::Voudoun => "Voudoun",
            Practice::WeirdScience => "Weird Science",
            Practice::Witchcraft => "Witchcraft",
            Practice::Yoga => "Yoga",
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Tenet {
    ARationalUniverse,
    AsAboveSoBelow,
    FallFromGrace,
    RealityIsALie,
    TheDivineIsReal,
    UnfathomableForcesAtWork,
    IAmChosen,
    IAmNotLimited,
    IAmSpecial,
    IHaveGreaterUnderstanding,
    AchieveFullKnowledge,
    AnnihilationIsFreedom,
    BuildAUtopia,
    ExistenceIsMeaningless,
    MergeWithTheDivine,
    PowerIsItsOwnReward,
    IIlluminate,
    IRule,
    IServe,
    MysticalInsights,
    ScientificExperimentation,
    DivineRevelations,
    ClosedParadigm,
    HierarchicalParadigm,
    OpenParadigm,
    Reincarnation,
    YOLO,
}

impl Tenet {
    pub const METAPHYSICAL: [Tenet; 6] = [
        Tenet::ARationalUniverse,
        Tenet::AsAboveSoBelow,
        Tenet::FallFromGrace,
        Tenet::RealityIsALie,
        Tenet::TheDivineIsReal,
        Tenet::UnfathomableForcesAtWork,
    ];
    pub const PERSONAL: [Tenet; 4] = [
        Tenet::IAmChosen,
        Tenet::IAmNotLimited,
        Tenet::IAmSpecial,
        Tenet::IHaveGreaterUnderstanding,
    ];

    pub const ASCENSION: [Tenet; 6] = [
        Tenet::AchieveFullKnowledge,
        Tenet::AnnihilationIsFreedom,
        Tenet::BuildAUtopia,
        Tenet::ExistenceIsMeaningless,
        Tenet::MergeWithTheDivine,
        Tenet::PowerIsItsOwnReward,
    ];

    pub const ROLE: [Tenet; 3] = [Tenet::IIlluminate, Tenet::IRule, Tenet::IServe];

    pub const EPISTEMOLOGY: [Tenet; 3] = [
        Tenet::MysticalInsights,
        Tenet::ScientificExperimentation,
        Tenet::DivineRevelations,
    ];

    pub const OPENNESS: [Tenet; 3] = [
        Tenet::ClosedParadigm,
        Tenet::HierarchicalParadigm,
        Tenet::OpenParadigm,
    ];

    pub const AFTERLIFE: [Tenet; 2] = [Tenet::Reincarnation, Tenet::YOLO];

    pub fn associated_practices(&self) -> HashSet<Practice> {
        match self {
            Tenet::ARationalUniverse => HashSet::from([
                Practice::Hypertech,
                Practice::Craftwork,
                Practice::Alchemy,
                Practice::Cybernetics,
                Practice::RealityHacking,
            ]),
            Tenet::AsAboveSoBelow => HashSet::from([
                Practice::Alchemy,
                Practice::HighRitualMagick,
                Practice::Faith,
                Practice::Witchcraft,
                Practice::Dominion,
            ]),
            Tenet::FallFromGrace => HashSet::from([
                Practice::Animalism,
                Practice::Faith,
                Practice::HighRitualMagick,
                Practice::GodBonding,
            ]),
            Tenet::RealityIsALie => HashSet::from([
                Practice::MartialArts,
                Practice::Bardism,
                Practice::ChaosMagick,
                Practice::Yoga,
                Practice::CrazyWisdom,
            ]),
            Tenet::TheDivineIsReal => HashSet::from([
                Practice::Invigoration,
                Practice::GodBonding,
                Practice::Yoga,
                Practice::Faith,
                Practice::Witchcraft,
            ]),
            Tenet::UnfathomableForcesAtWork => HashSet::from([
                Practice::Voudoun,
                Practice::Psionics,
                Practice::Invigoration,
                Practice::Elementalism,
                Practice::Maleficia,
            ]),
            Tenet::IAmChosen => HashSet::from([
                Practice::Mediumship,
                Practice::GodBonding,
                Practice::Witchcraft,
                Practice::Shamanism,
                Practice::MartialArts,
                Practice::Cybernetics,
            ]),
            Tenet::IAmNotLimited => HashSet::from([
                Practice::Psionics,
                Practice::WeirdScience,
                Practice::HighRitualMagick,
                Practice::CrazyWisdom,
                Practice::Invigoration,
            ]),
            Tenet::IAmSpecial => HashSet::from([
                Practice::Elementalism,
                Practice::Bardism,
                Practice::Dominion,
                Practice::Shamanism,
                Practice::Psionics,
            ]),
            Tenet::IHaveGreaterUnderstanding => HashSet::from([
                Practice::Hypertech,
                Practice::RealityHacking,
                Practice::Alchemy,
                Practice::MedicineWork,
                Practice::CrazyWisdom,
            ]),
            Tenet::AchieveFullKnowledge => HashSet::from([
                Practice::Hypertech,
                Practice::RealityHacking,
                Practice::Alchemy,
                Practice::MedicineWork,
                Practice::WeirdScience,
            ]),
            Tenet::AnnihilationIsFreedom => HashSet::from([
                Practice::GutterMagick,
                Practice::Maleficia,
                Practice::MartialArts,
                Practice::ChaosMagick,
            ]),
            Tenet::BuildAUtopia => HashSet::from([
                Practice::ArtOfDesire,
                Practice::Faith,
                Practice::Dominion,
                Practice::WeirdScience,
                Practice::Craftwork,
                Practice::Cybernetics,
            ]),
            Tenet::ExistenceIsMeaningless => HashSet::from([
                Practice::Bardism,
                Practice::GutterMagick,
                Practice::Animalism,
            ]),
            Tenet::MergeWithTheDivine => HashSet::from([
                Practice::Mediumship,
                Practice::Elementalism,
                Practice::Yoga,
                Practice::Faith,
                Practice::GodBonding,
                Practice::Voudoun,
            ]),
            Tenet::PowerIsItsOwnReward => HashSet::from([
                Practice::HighRitualMagick,
                Practice::Maleficia,
                Practice::ArtOfDesire,
                Practice::Dominion,
            ]),
            Tenet::IIlluminate => HashSet::from([
                Practice::Hypertech,
                Practice::Charity,
                Practice::Alchemy,
                Practice::Faith,
                Practice::HighRitualMagick,
            ]),
            Tenet::IRule => HashSet::from([
                Practice::MediaControl,
                Practice::ArtOfDesire,
                Practice::Alchemy,
                Practice::Dominion,
                Practice::HighRitualMagick,
            ]),
            Tenet::IServe => HashSet::from([
                Practice::Mediumship,
                Practice::Charity,
                Practice::Faith,
                Practice::Witchcraft,
                Practice::Shamanism,
            ]),
            Tenet::MysticalInsights => HashSet::from([
                Practice::Alchemy,
                Practice::Witchcraft,
                Practice::HighRitualMagick,
                Practice::CrazyWisdom,
                Practice::ChaosMagick,
            ]),
            Tenet::ScientificExperimentation => HashSet::from([
                Practice::Hypertech,
                Practice::Alchemy,
                Practice::WeirdScience,
                Practice::Craftwork,
                Practice::Cybernetics,
                Practice::HighRitualMagick,
            ]),
            Tenet::DivineRevelations => HashSet::from([
                Practice::Maleficia,
                Practice::Faith,
                Practice::GodBonding,
                Practice::Shamanism,
                Practice::CrazyWisdom,
            ]),
            Tenet::ClosedParadigm => HashSet::from([
                Practice::Hypertech,
                Practice::Cybernetics,
                Practice::Dominion,
            ]),
            Tenet::HierarchicalParadigm => HashSet::from([
                Practice::GodBonding,
                Practice::MartialArts,
                Practice::WeirdScience,
                Practice::Dominion,
            ]),
            Tenet::OpenParadigm => HashSet::from([
                Practice::ArtOfDesire,
                Practice::GutterMagick,
                Practice::WeirdScience,
                Practice::CrazyWisdom,
                Practice::ChaosMagick,
            ]),
            Tenet::Reincarnation => HashSet::from([
                Practice::Yoga,
                Practice::Witchcraft,
                Practice::Shamanism,
                Practice::MartialArts,
                Practice::CrazyWisdom,
                Practice::ChaosMagick,
            ]),
            Tenet::YOLO => HashSet::from([
                Practice::Hypertech,
                Practice::WeirdScience,
                Practice::Faith,
                Practice::Alchemy,
            ]),
        }
    }

    pub fn limited_practices(&self) -> HashSet<Practice> {
        match self {
            Tenet::ARationalUniverse => HashSet::from([
                Practice::ChaosMagick,
                Practice::Voudoun,
                Practice::Witchcraft,
                Practice::GutterMagick,
                Practice::CrazyWisdom,
            ]),
            Tenet::AsAboveSoBelow => HashSet::from([
                Practice::ChaosMagick,
                Practice::ArtOfDesire,
                Practice::WeirdScience,
                Practice::RealityHacking,
                Practice::Hypertech,
            ]),
            Tenet::FallFromGrace => HashSet::from([
                Practice::Yoga,
                Practice::RealityHacking,
                Practice::MedicineWork,
                Practice::Hypertech,
                Practice::Invigoration,
            ]),
            Tenet::RealityIsALie => HashSet::from([
                Practice::HighRitualMagick,
                Practice::Cybernetics,
                Practice::WeirdScience,
                Practice::Elementalism,
                Practice::Craftwork,
            ]),
            Tenet::TheDivineIsReal => HashSet::from([
                Practice::WeirdScience,
                Practice::RealityHacking,
                Practice::Bardism,
            ]),
            Tenet::UnfathomableForcesAtWork => HashSet::from([
                Practice::Cybernetics,
                Practice::ArtOfDesire,
                Practice::WeirdScience,
                Practice::RealityHacking,
                Practice::Dominion,
            ]),
            Tenet::IAmChosen => HashSet::from([
                Practice::HighRitualMagick,
                Practice::ArtOfDesire,
                Practice::Elementalism,
                Practice::Alchemy,
                Practice::Hypertech,
                Practice::Invigoration,
            ]),
            Tenet::IAmNotLimited => HashSet::from([
                Practice::Witchcraft,
                Practice::Craftwork,
                Practice::MartialArts,
            ]),
            Tenet::IAmSpecial => HashSet::from([
                Practice::ChaosMagick,
                Practice::GutterMagick,
                Practice::Mediumship,
            ]),
            Tenet::IHaveGreaterUnderstanding => HashSet::from([
                Practice::GodBonding,
                Practice::Psionics,
                Practice::GutterMagick,
                Practice::Bardism,
            ]),
            Tenet::AchieveFullKnowledge => HashSet::from([
                Practice::Faith,
                Practice::Animalism,
                Practice::Yoga,
                Practice::Bardism,
                Practice::GodBonding,
                Practice::Mediumship,
            ]),
            Tenet::AnnihilationIsFreedom => HashSet::from([
                Practice::Animalism,
                Practice::ArtOfDesire,
                Practice::Elementalism,
                Practice::MedicineWork,
                Practice::Craftwork,
            ]),
            Tenet::BuildAUtopia => {
                HashSet::from([Practice::Voudoun, Practice::Maleficia, Practice::Animalism])
            }
            Tenet::ExistenceIsMeaningless => HashSet::from([
                Practice::GodBonding,
                Practice::Alchemy,
                Practice::Faith,
                Practice::MedicineWork,
            ]),
            Tenet::MergeWithTheDivine => HashSet::from([
                Practice::ChaosMagick,
                Practice::WeirdScience,
                Practice::Cybernetics,
                Practice::Hypertech,
            ]),
            Tenet::PowerIsItsOwnReward => HashSet::from([
                Practice::Yoga,
                Practice::Faith,
                Practice::Shamanism,
                Practice::Voudoun,
                Practice::CrazyWisdom,
            ]),
            Tenet::IIlluminate => HashSet::from([
                Practice::Appropriation,
                Practice::Animalism,
                Practice::MediaControl,
                Practice::Maleficia,
                Practice::Psionics,
            ]),
            Tenet::IRule => HashSet::from([
                Practice::Faith,
                Practice::Shamanism,
                Practice::Charity,
                Practice::Witchcraft,
                Practice::Mediumship,
            ]),
            Tenet::IServe => HashSet::from([
                Practice::HighRitualMagick,
                Practice::MediaControl,
                Practice::ArtOfDesire,
                Practice::Alchemy,
                Practice::Dominion,
            ]),
            Tenet::MysticalInsights => HashSet::from([
                Practice::MediaControl,
                Practice::Faith,
                Practice::Cybernetics,
                Practice::Hypertech,
            ]),
            Tenet::ScientificExperimentation => HashSet::from([
                Practice::ChaosMagick,
                Practice::Faith,
                Practice::Shamanism,
                Practice::Maleficia,
                Practice::Witchcraft,
                Practice::CrazyWisdom,
            ]),
            Tenet::DivineRevelations => HashSet::from([
                Practice::HighRitualMagick,
                Practice::WeirdScience,
                Practice::MartialArts,
                Practice::Alchemy,
                Practice::Hypertech,
            ]),
            Tenet::ClosedParadigm => HashSet::from([
                Practice::ChaosMagick,
                Practice::ArtOfDesire,
                Practice::WeirdScience,
                Practice::GutterMagick,
                Practice::CrazyWisdom,
            ]),
            Tenet::HierarchicalParadigm => HashSet::from([
                Practice::ChaosMagick,
                Practice::HighRitualMagick,
                Practice::Faith,
                Practice::Cybernetics,
                Practice::Hypertech,
            ]),
            Tenet::OpenParadigm => HashSet::from([
                Practice::Dominion,
                Practice::Cybernetics,
                Practice::Hypertech,
            ]),
            Tenet::Reincarnation => HashSet::from([
                Practice::WeirdScience,
                Practice::Alchemy,
                Practice::Faith,
                Practice::Hypertech,
            ]),
            Tenet::YOLO => HashSet::from([
                Practice::ChaosMagick,
                Practice::Yoga,
                Practice::Shamanism,
                Practice::MartialArts,
                Practice::Witchcraft,
                Practice::CrazyWisdom,
            ]),
        }
    }
}

impl Display for Tenet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Tenet::ARationalUniverse => "A Rational Universe",
            Tenet::AsAboveSoBelow => "Above So Below",
            Tenet::FallFromGrace => "Fall From Grace",
            Tenet::RealityIsALie => "Reality is a Lie",
            Tenet::TheDivineIsReal => "The Divine is Real",
            Tenet::UnfathomableForcesAtWork => "Unfathomable Forces at Work",
            Tenet::IAmChosen => "I Am Chosen",
            Tenet::IAmNotLimited => "I Am Not Limited",
            Tenet::IAmSpecial => "I Am Special",
            Tenet::IHaveGreaterUnderstanding => "I Have Greater Understanding",
            Tenet::AchieveFullKnowledge => "Achieve Full Knowledge",
            Tenet::AnnihilationIsFreedom => "Annihilation is Freedom",
            Tenet::BuildAUtopia => "Build a Utopia",
            Tenet::ExistenceIsMeaningless => "Existence is Meaningless",
            Tenet::MergeWithTheDivine => "Merge With The Divine",
            Tenet::PowerIsItsOwnReward => "Power is its Own Reward",
            Tenet::IIlluminate => "I Illuminate",
            Tenet::IRule => "I Rule",
            Tenet::IServe => "I Serve",
            Tenet::MysticalInsights => "Mystical Insights",
            Tenet::ScientificExperimentation => "Scientific Experimentation",
            Tenet::DivineRevelations => "Divine Revelations",
            Tenet::ClosedParadigm => "Closed Paradigm",
            Tenet::HierarchicalParadigm => "Hierarchical Paradigm",
            Tenet::OpenParadigm => "Open Paradigm",
            Tenet::Reincarnation => "Reincarnation",
            Tenet::YOLO => "YOLO",
        })
    }
}
