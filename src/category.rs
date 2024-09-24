use std::fmt;

const CONFIDENCE_THRESHOLD: f64 = 0.8;
const OSX_APP_CATEGORY_PREFIX: &str = "public.app-category.";

// TODO: RIght now, these categories correspond to LSApplicationCategoryType
// values for OS X.  There are also some additional GNOME registered categories
// that don't fit these; we should add those here too.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Category {
    Business,
    DeveloperTool,
    Education,
    Entertainment,
    Finance,
    Game,
    ActionGame,
    AdventureGame,
    ArcadeGame,
    BoardGame,
    CardGame,
    CasinoGame,
    DiceGame,
    EducationalGame,
    FamilyGame,
    KidsGame,
    MusicGame,
    PuzzleGame,
    RacingGame,
    RolePlayingGame,
    SimulationGame,
    SportsGame,
    StrategyGame,
    TriviaGame,
    WordGame,
    GraphicsAndDesign,
    HealthcareAndFitness,
    Lifestyle,
    Medical,
    Music,
    News,
    Photography,
    Productivity,
    Reference,
    SocialNetworking,
    Sports,
    Travel,
    Utility,
    Video,
    Weather,
}

impl Category {
    /// Given a string, returns the `AppCategory` it refers to, or the closest
    /// string that the user might have intended (if any).
    pub fn from_str(input: &str) -> Result<Category, Option<&'static str>> {
        // Canonicalize input:
        let mut input = input.to_ascii_lowercase();
        if input.starts_with(OSX_APP_CATEGORY_PREFIX) {
            input = input.split_at(OSX_APP_CATEGORY_PREFIX.len()).1.to_string();
        }
        input = input.replace(' ', "");
        input = input.replace('-', "");

        // Find best match:
        let mut best_confidence = 0.0;
        let mut best_category: Option<Category> = None;
        for &(string, category) in CATEGORY_STRINGS.iter() {
            if input == string {
                return Ok(category);
            }
            let confidence = strsim::jaro_winkler(&input, string);
            if confidence >= CONFIDENCE_THRESHOLD && confidence > best_confidence {
                best_confidence = confidence;
                best_category = Some(category);
            }
        }
        Err(best_category.map(Category::canonical))
    }

    /// Map an AppCategory to the string we recommend to use in Cargo.toml if
    /// the users misspells the category name.
    fn canonical(self) -> &'static str {
        match self {
            Category::Business => "Business",
            Category::DeveloperTool => "Developer Tool",
            Category::Education => "Education",
            Category::Entertainment => "Entertainment",
            Category::Finance => "Finance",
            Category::Game => "Game",
            Category::ActionGame => "Action Game",
            Category::AdventureGame => "Adventure Game",
            Category::ArcadeGame => "Arcade Game",
            Category::BoardGame => "Board Game",
            Category::CardGame => "Card Game",
            Category::CasinoGame => "Casino Game",
            Category::DiceGame => "Dice Game",
            Category::EducationalGame => "Educational Game",
            Category::FamilyGame => "Family Game",
            Category::KidsGame => "Kids Game",
            Category::MusicGame => "Music Game",
            Category::PuzzleGame => "Puzzle Game",
            Category::RacingGame => "Racing Game",
            Category::RolePlayingGame => "Role-Playing Game",
            Category::SimulationGame => "Simulation Game",
            Category::SportsGame => "Sports Game",
            Category::StrategyGame => "Strategy Game",
            Category::TriviaGame => "Trivia Game",
            Category::WordGame => "Word Game",
            Category::GraphicsAndDesign => "Graphics and Design",
            Category::HealthcareAndFitness => "Healthcare and Fitness",
            Category::Lifestyle => "Lifestyle",
            Category::Medical => "Medical",
            Category::Music => "Music",
            Category::News => "News",
            Category::Photography => "Photography",
            Category::Productivity => "Productivity",
            Category::Reference => "Reference",
            Category::SocialNetworking => "Social Networking",
            Category::Sports => "Sports",
            Category::Travel => "Travel",
            Category::Utility => "Utility",
            Category::Video => "Video",
            Category::Weather => "Weather",
        }
    }

    /// Map an AppCategory to the closest set of GNOME desktop registered
    /// categories that matches that category.
    pub fn gnome_desktop_categories(&self) -> &'static str {
        match &self {
            Category::Business => "Office;",
            Category::DeveloperTool => "Development;",
            Category::Education => "Education;",
            Category::Entertainment => "Network;",
            Category::Finance => "Office;Finance;",
            Category::Game => "Game;",
            Category::ActionGame => "Game;ActionGame;",
            Category::AdventureGame => "Game;AdventureGame;",
            Category::ArcadeGame => "Game;ArcadeGame;",
            Category::BoardGame => "Game;BoardGame;",
            Category::CardGame => "Game;CardGame;",
            Category::CasinoGame => "Game;",
            Category::DiceGame => "Game;",
            Category::EducationalGame => "Game;Education;",
            Category::FamilyGame => "Game;",
            Category::KidsGame => "Game;KidsGame;",
            Category::MusicGame => "Game;",
            Category::PuzzleGame => "Game;LogicGame;",
            Category::RacingGame => "Game;",
            Category::RolePlayingGame => "Game;RolePlaying;",
            Category::SimulationGame => "Game;Simulation;",
            Category::SportsGame => "Game;SportsGame;",
            Category::StrategyGame => "Game;StrategyGame;",
            Category::TriviaGame => "Game;",
            Category::WordGame => "Game;",
            Category::GraphicsAndDesign => "Graphics;",
            Category::HealthcareAndFitness => "Science;",
            Category::Lifestyle => "Education;",
            Category::Medical => "Science;MedicalSoftware;",
            Category::Music => "AudioVideo;Audio;Music;",
            Category::News => "Network;News;",
            Category::Photography => "Graphics;Photography;",
            Category::Productivity => "Office;",
            Category::Reference => "Education;",
            Category::SocialNetworking => "Network;",
            Category::Sports => "Education;Sports;",
            Category::Travel => "Education;",
            Category::Utility => "Utility;",
            Category::Video => "AudioVideo;Video;",
            Category::Weather => "Science;",
        }
    }

    /// Map an AppCategory to the closest LSApplicationCategoryType value that
    /// matches that category.
    pub fn osx_application_category_type(&self) -> &'static str {
        match &self {
            Category::Business => "public.app-category.business",
            Category::DeveloperTool => "public.app-category.developer-tools",
            Category::Education => "public.app-category.education",
            Category::Entertainment => "public.app-category.entertainment",
            Category::Finance => "public.app-category.finance",
            Category::Game => "public.app-category.games",
            Category::ActionGame => "public.app-category.action-games",
            Category::AdventureGame => "public.app-category.adventure-games",
            Category::ArcadeGame => "public.app-category.arcade-games",
            Category::BoardGame => "public.app-category.board-games",
            Category::CardGame => "public.app-category.card-games",
            Category::CasinoGame => "public.app-category.casino-games",
            Category::DiceGame => "public.app-category.dice-games",
            Category::EducationalGame => "public.app-category.educational-games",
            Category::FamilyGame => "public.app-category.family-games",
            Category::KidsGame => "public.app-category.kids-games",
            Category::MusicGame => "public.app-category.music-games",
            Category::PuzzleGame => "public.app-category.puzzle-games",
            Category::RacingGame => "public.app-category.racing-games",
            Category::RolePlayingGame => "public.app-category.role-playing-games",
            Category::SimulationGame => "public.app-category.simulation-games",
            Category::SportsGame => "public.app-category.sports-games",
            Category::StrategyGame => "public.app-category.strategy-games",
            Category::TriviaGame => "public.app-category.trivia-games",
            Category::WordGame => "public.app-category.word-games",
            Category::GraphicsAndDesign => "public.app-category.graphics-design",
            Category::HealthcareAndFitness => "public.app-category.healthcare-fitness",
            Category::Lifestyle => "public.app-category.lifestyle",
            Category::Medical => "public.app-category.medical",
            Category::Music => "public.app-category.music",
            Category::News => "public.app-category.news",
            Category::Photography => "public.app-category.photography",
            Category::Productivity => "public.app-category.productivity",
            Category::Reference => "public.app-category.reference",
            Category::SocialNetworking => "public.app-category.social-networking",
            Category::Sports => "public.app-category.sports",
            Category::Travel => "public.app-category.travel",
            Category::Utility => "public.app-category.utilities",
            Category::Video => "public.app-category.video",
            Category::Weather => "public.app-category.weather",
        }
    }
}

impl<'d> serde::Deserialize<'d> for Category {
    fn deserialize<D: serde::Deserializer<'d>>(deserializer: D) -> Result<Category, D::Error> {
        deserializer.deserialize_str(AppCategoryVisitor { did_you_mean: None })
    }
}

struct AppCategoryVisitor {
    did_you_mean: Option<&'static str>,
}

impl<'d> serde::de::Visitor<'d> for AppCategoryVisitor {
    type Value = Category;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self.did_you_mean {
            Some(string) => {
                write!(
                    formatter,
                    "a valid app category string (did you mean \"{string}\"?)"
                )
            }
            None => write!(formatter, "a valid app category string"),
        }
    }

    fn visit_str<E: serde::de::Error>(mut self, value: &str) -> Result<Category, E> {
        match Category::from_str(value) {
            Ok(category) => Ok(category),
            Err(did_you_mean) => {
                self.did_you_mean = did_you_mean;
                let unexp = serde::de::Unexpected::Str(value);
                Err(serde::de::Error::invalid_value(unexp, &self))
            }
        }
    }
}

const CATEGORY_STRINGS: &[(&str, Category)] = &[
    ("actiongame", Category::ActionGame),
    ("actiongames", Category::ActionGame),
    ("adventuregame", Category::AdventureGame),
    ("adventuregames", Category::AdventureGame),
    ("arcadegame", Category::ArcadeGame),
    ("arcadegames", Category::ArcadeGame),
    ("boardgame", Category::BoardGame),
    ("boardgames", Category::BoardGame),
    ("business", Category::Business),
    ("cardgame", Category::CardGame),
    ("cardgames", Category::CardGame),
    ("casinogame", Category::CasinoGame),
    ("casinogames", Category::CasinoGame),
    ("developer", Category::DeveloperTool),
    ("developertool", Category::DeveloperTool),
    ("developertools", Category::DeveloperTool),
    ("development", Category::DeveloperTool),
    ("dicegame", Category::DiceGame),
    ("dicegames", Category::DiceGame),
    ("education", Category::Education),
    ("educationalgame", Category::EducationalGame),
    ("educationalgames", Category::EducationalGame),
    ("entertainment", Category::Entertainment),
    ("familygame", Category::FamilyGame),
    ("familygames", Category::FamilyGame),
    ("finance", Category::Finance),
    ("fitness", Category::HealthcareAndFitness),
    ("game", Category::Game),
    ("games", Category::Game),
    ("graphicdesign", Category::GraphicsAndDesign),
    ("graphicsanddesign", Category::GraphicsAndDesign),
    ("graphicsdesign", Category::GraphicsAndDesign),
    ("healthcareandfitness", Category::HealthcareAndFitness),
    ("healthcarefitness", Category::HealthcareAndFitness),
    ("kidsgame", Category::KidsGame),
    ("kidsgames", Category::KidsGame),
    ("lifestyle", Category::Lifestyle),
    ("logicgame", Category::PuzzleGame),
    ("medical", Category::Medical),
    ("medicalsoftware", Category::Medical),
    ("music", Category::Music),
    ("musicgame", Category::MusicGame),
    ("musicgames", Category::MusicGame),
    ("news", Category::News),
    ("photography", Category::Photography),
    ("productivity", Category::Productivity),
    ("puzzlegame", Category::PuzzleGame),
    ("puzzlegames", Category::PuzzleGame),
    ("racinggame", Category::RacingGame),
    ("racinggames", Category::RacingGame),
    ("reference", Category::Reference),
    ("roleplaying", Category::RolePlayingGame),
    ("roleplayinggame", Category::RolePlayingGame),
    ("roleplayinggames", Category::RolePlayingGame),
    ("rpg", Category::RolePlayingGame),
    ("simulationgame", Category::SimulationGame),
    ("simulationgames", Category::SimulationGame),
    ("socialnetwork", Category::SocialNetworking),
    ("socialnetworking", Category::SocialNetworking),
    ("sports", Category::Sports),
    ("sportsgame", Category::SportsGame),
    ("sportsgames", Category::SportsGame),
    ("strategygame", Category::StrategyGame),
    ("strategygames", Category::StrategyGame),
    ("travel", Category::Travel),
    ("triviagame", Category::TriviaGame),
    ("triviagames", Category::TriviaGame),
    ("utilities", Category::Utility),
    ("utility", Category::Utility),
    ("video", Category::Video),
    ("weather", Category::Weather),
    ("wordgame", Category::WordGame),
    ("wordgames", Category::WordGame),
];

#[cfg(test)]
mod tests {
    use super::Category;

    #[test]
    fn category_from_string_ok() {
        // Canonical name of category works:
        assert_eq!(Category::from_str("Education"), Ok(Category::Education));
        assert_eq!(
            Category::from_str("Developer Tool"),
            Ok(Category::DeveloperTool)
        );
        // Lowercase, spaces, and hyphens are fine:
        assert_eq!(
            Category::from_str(" puzzle  game "),
            Ok(Category::PuzzleGame)
        );
        assert_eq!(
            Category::from_str("Role-playing game"),
            Ok(Category::RolePlayingGame)
        );
        // Using macOS LSApplicationCategoryType value is fine:
        assert_eq!(
            Category::from_str("public.app-category.developer-tools"),
            Ok(Category::DeveloperTool)
        );
        assert_eq!(
            Category::from_str("public.app-category.role-playing-games"),
            Ok(Category::RolePlayingGame)
        );
        // Using GNOME category name is fine:
        assert_eq!(
            Category::from_str("Development"),
            Ok(Category::DeveloperTool)
        );
        assert_eq!(Category::from_str("LogicGame"), Ok(Category::PuzzleGame));
        // Using common abbreviations is fine:
        assert_eq!(Category::from_str("RPG"), Ok(Category::RolePlayingGame));
    }

    #[test]
    fn category_from_string_did_you_mean() {
        assert_eq!(Category::from_str("gaming"), Err(Some("Game")));
        assert_eq!(Category::from_str("photos"), Err(Some("Photography")));
        assert_eq!(Category::from_str("strategery"), Err(Some("Strategy Game")));
    }

    #[test]
    fn category_from_string_totally_wrong() {
        assert_eq!(Category::from_str("fhqwhgads"), Err(None));
        assert_eq!(Category::from_str("WHARRGARBL"), Err(None));
    }

    #[test]
    fn ls_application_category_type_round_trip() {
        let values = &[
            "public.app-category.business",
            "public.app-category.developer-tools",
            "public.app-category.education",
            "public.app-category.entertainment",
            "public.app-category.finance",
            "public.app-category.games",
            "public.app-category.action-games",
            "public.app-category.adventure-games",
            "public.app-category.arcade-games",
            "public.app-category.board-games",
            "public.app-category.card-games",
            "public.app-category.casino-games",
            "public.app-category.dice-games",
            "public.app-category.educational-games",
            "public.app-category.family-games",
            "public.app-category.kids-games",
            "public.app-category.music-games",
            "public.app-category.puzzle-games",
            "public.app-category.racing-games",
            "public.app-category.role-playing-games",
            "public.app-category.simulation-games",
            "public.app-category.sports-games",
            "public.app-category.strategy-games",
            "public.app-category.trivia-games",
            "public.app-category.word-games",
            "public.app-category.graphics-design",
            "public.app-category.healthcare-fitness",
            "public.app-category.lifestyle",
            "public.app-category.medical",
            "public.app-category.music",
            "public.app-category.news",
            "public.app-category.photography",
            "public.app-category.productivity",
            "public.app-category.reference",
            "public.app-category.social-networking",
            "public.app-category.sports",
            "public.app-category.travel",
            "public.app-category.utilities",
            "public.app-category.video",
            "public.app-category.weather",
        ];
        // Test that if the user uses an LSApplicationCategoryType string as
        // the category string, they will get back that same string for the
        // macOS app bundle LSApplicationCategoryType.
        for &value in values.iter() {
            let category = Category::from_str(value).expect(value);
            assert_eq!(category.osx_application_category_type(), value);
        }
    }
}
