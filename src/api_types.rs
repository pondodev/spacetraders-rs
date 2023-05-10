pub mod general {
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    pub struct Metadata {
        pub total: i32,
        pub page: i32,
        pub limit: i32,
    }
}

pub mod factions {
    use serde::Deserialize;
    use serde::de::{self, Visitor};

    #[derive(Deserialize, Debug)]
    pub struct Data {
        pub symbol: String,
        pub name: String,
        pub description: String,
        pub headquarters: String,
        pub traits: Vec<Trait>,
    }

    #[derive(Debug)]
    pub enum Symbol {
        Bureaucratic,   Secretive,      Capitalistic,    Industrious,
        Peaceful,       Distrustful,    Welcoming,       Anarchist,
        Conflicted,     Authoritarian,  Oligarchical,    Dynastic,
        Democratic,     Decentralized,  Smugglers,       Scavengers,
        Rebellious,     Exiles,         Pirates,         Raiders,
        Clan,           Guild,          Dominion,        Fringe,
        Forsaken,       Isolated,       Localized,       Established,
        Notable,        Dominant,       Inescapable,     Innovative,
        Bold,           Visionary,      Curious,         Daring,
        Exploratory,    Resourceful,    Flexible,        Cooperative,
        United,         Strategic,      Intelligent,     ResearchFocused,
        Collaborative,  Progressive,    Militaristic,    TechnologicallyAdvanced,
        Aggressive,     Imperialistic,  TreasureHunters, Dexterous,
        Unpredictable,  Brutal,         Fleeting,        Adaptable,
        SelfSufficient, Defensive,      Proud,           Diverse,
        Independent,    SelfInterested, Fragmented,      Commercial,
        FreeMarkets,    Entrepreneurial,
    }

    struct SymbolVisitor;
    impl <'de> Visitor<'de> for SymbolVisitor {
        type Value = Symbol;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            return formatter.write_str("a valid string from the list of possible faction symbols")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error, {
            match v {
                "BUREAUCRATIC"             => return Ok(Symbol::Bureaucratic),
                "SECRETIVE"                => return Ok(Symbol::Secretive),
                "CAPITALISTIC"             => return Ok(Symbol::Capitalistic),
                "INDUSTRIOUS"              => return Ok(Symbol::Industrious),
                "PEACEFUL"                 => return Ok(Symbol::Peaceful),
                "DISTRUSTFUL"              => return Ok(Symbol::Distrustful),
                "WELCOMING"                => return Ok(Symbol::Welcoming),
                "ANARCHIST"                => return Ok(Symbol::Anarchist),
                "CONFLICTED"               => return Ok(Symbol::Conflicted),
                "AUTHORITARIAN"            => return Ok(Symbol::Authoritarian),
                "OLIGARCHICAL"             => return Ok(Symbol::Oligarchical),
                "DYNASTIC"                 => return Ok(Symbol::Dynastic),
                "DEMOCRATIC"               => return Ok(Symbol::Democratic),
                "DECENTRALIZED"            => return Ok(Symbol::Decentralized),
                "SMUGGLERS"                => return Ok(Symbol::Smugglers),
                "SCAVENGERS"               => return Ok(Symbol::Scavengers),
                "REBELLIOUS"               => return Ok(Symbol::Rebellious),
                "EXILES"                   => return Ok(Symbol::Exiles),
                "PIRATES"                  => return Ok(Symbol::Pirates),
                "RAIDERS"                  => return Ok(Symbol::Raiders),
                "CLAN"                     => return Ok(Symbol::Clan),
                "GUILD"                    => return Ok(Symbol::Guild),
                "DOMINION"                 => return Ok(Symbol::Dominion),
                "FRINGE"                   => return Ok(Symbol::Fringe),
                "FORSAKEN"                 => return Ok(Symbol::Forsaken),
                "ISOLATED"                 => return Ok(Symbol::Isolated),
                "LOCALIZED"                => return Ok(Symbol::Localized),
                "ESTABLISHED"              => return Ok(Symbol::Established),
                "NOTABLE"                  => return Ok(Symbol::Notable),
                "DOMINANT"                 => return Ok(Symbol::Dominant),
                "INESCAPABLE"              => return Ok(Symbol::Inescapable),
                "INNOVATIVE"               => return Ok(Symbol::Innovative),
                "BOLD"                     => return Ok(Symbol::Bold),
                "VISIONARY"                => return Ok(Symbol::Visionary),
                "CURIOUS"                  => return Ok(Symbol::Curious),
                "DARING"                   => return Ok(Symbol::Daring),
                "EXPLORATORY"              => return Ok(Symbol::Exploratory),
                "RESOURCEFUL"              => return Ok(Symbol::Resourceful),
                "FLEXIBLE"                 => return Ok(Symbol::Flexible),
                "COOPERATIVE"              => return Ok(Symbol::Cooperative),
                "UNITED"                   => return Ok(Symbol::United),
                "STRATEGIC"                => return Ok(Symbol::Strategic),
                "INTELLIGENT"              => return Ok(Symbol::Intelligent),
                "RESEARCH_FOCUSED"         => return Ok(Symbol::ResearchFocused),
                "COLLABORATIVE"            => return Ok(Symbol::Collaborative),
                "PROGRESSIVE"              => return Ok(Symbol::Progressive),
                "MILITARISTIC"             => return Ok(Symbol::Militaristic),
                "TECHNOLOGICALLY_ADVANCED" => return Ok(Symbol::TechnologicallyAdvanced),
                "AGGRESSIVE"               => return Ok(Symbol::Aggressive),
                "IMPERIALISTIC"            => return Ok(Symbol::Imperialistic),
                "TREASURE_HUNTERS"         => return Ok(Symbol::TreasureHunters),
                "DEXTEROUS"                => return Ok(Symbol::Dexterous),
                "UNPREDICTABLE"            => return Ok(Symbol::Unpredictable),
                "BRUTAL"                   => return Ok(Symbol::Brutal),
                "FLEETING"                 => return Ok(Symbol::Fleeting),
                "ADAPTABLE"                => return Ok(Symbol::Adaptable),
                "SELF_SUFFICIENT"          => return Ok(Symbol::SelfSufficient),
                "DEFENSIVE"                => return Ok(Symbol::Defensive),
                "PROUD"                    => return Ok(Symbol::Proud),
                "DIVERSE"                  => return Ok(Symbol::Diverse),
                "INDEPENDENT"              => return Ok(Symbol::Independent),
                "SELF_INTERESTED"          => return Ok(Symbol::SelfInterested),
                "FRAGMENTED"               => return Ok(Symbol::Fragmented),
                "COMMERCIAL"               => return Ok(Symbol::Commercial),
                "FREE_MARKETS"             => return Ok(Symbol::FreeMarkets),
                "ENTREPRENEURIAL"          => return Ok(Symbol::Entrepreneurial),
                _ => return Err(de::Error::custom(format!("unrecognised symbol enum value (got '{}')", v))),
            }
        }
    }

    impl<'de> Deserialize<'de> for Symbol {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
            return deserializer.deserialize_str(SymbolVisitor)
        }
    }

    #[derive(Deserialize, Debug)]
    pub struct Trait {
        pub symbol: Symbol,
        pub name: String,
        pub description: String,
    }

    pub mod list {
        use serde::Deserialize;
        use crate::api_types::{general, factions};

        #[derive(Deserialize, Debug)]
        pub struct Response {
            pub data: Vec<factions::Data>,
            pub meta: general::Metadata,
        }
    }

    pub mod get {
        use serde::Deserialize;
        use crate::api_types::factions;

        #[derive(Deserialize, Debug)]
        pub struct Response {
            pub data: factions::Data,
        }
    }
}
