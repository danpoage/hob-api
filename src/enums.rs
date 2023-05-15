use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Stat {
    NA,
    X,
    Number(u8),
    Letter(char),
    Sphere(Sphere),
}

#[derive(Debug, Deserialize, Serialize)]
pub enum StatType {
    Willpower,
    Attack,
    Defense,
    HitPoints,
    VictoryPoints,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Trait {
    None,
    Gondor,
    Warrior,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Keyword {
    None,
    Ranged,
    Sentinel,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Sphere {
    Leadership,
    Tactics,
    Spirit,
    Lore,
    Neutral,
    Baggins,
    Fellowship,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum EncounterSetType {
    Cycle,
    Standalone,
    Nightmare,
    Saga,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum CardMainType {
    Hero,
    Ally,
    Attachment,
    Event,
    PlayerSideQuest,
    Contract,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum CardSubType {
    None,
    Boon,
    Burden,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum DeckType {
    None,
    Player,
    Encounter,
    Quest,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum ProductCode {
    MEC01,  //Core Set
    MEC02,  //The Hunt for Gollum
    MEC73,  //Limited Edition Two Player Starter
    MEC101, //Revised Core Set
}

impl ProductCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            ProductCode::MEC01 =>  "MEC01",
            ProductCode::MEC02 =>  "MEC02",
            ProductCode::MEC73 =>  "MEC73",
            ProductCode::MEC101 => "MEC101",
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ProductType {
    CoreSet,
    DeluxeExpansion,
    AdventurePack,
    SagaExpansion,
    LimitedEditionStarter,
    GenCon,
    Fellowship,
    CustomScenarioKit,
    Community,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum LinkType {
    None,
    EncounterSetIcon,
    CardImage,
    CardThumbnail,
    Illustrator,
    Blog,
    Video,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum LinkSource {
    None,
    CardboardOfTheRings,
    HallOfBeorn,
    VisionOfThePalantir,
}
