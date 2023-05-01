pub enum Stat {
    NA,
    X,
    Number(u8),
    Letter(char),
    Sphere(Sphere),
}

pub enum StatType {
    Willpower,
    Attack,
    Defense,
    HitPoints,
    VictoryPoints,
}

pub enum Trait {
    None,
    Gondor,
    Warrior,
}

pub enum Keyword {
    None,
    Ranged,
    Sentinel,
}

pub enum Sphere {
    Leadership,
    Tactics,
    Spirit,
    Lore,
    Neutral,
    Baggins,
    Fellowship,
}

pub enum EncounterSetType {
    Cycle,
    Standalone,
    Nightmare,
    Saga,
}

pub enum CardMainType {
    Hero,
    Ally,
    Attachment,
    Event,
    PlayerSideQuest,
    Contract,
}

pub enum CardSubType {
    None,
    Boon,
    Burden,
}

pub enum DeckType {
    None,
    Player,
    Encounter,
    Quest,
}

pub enum ProductCode {
    MEC01,  //Core Set
    MEC73,  //Limited Edition Two Player Starter
    MEC101, //Revised Core Set
}

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

pub enum LinkType {
    None,
    EncounterSetIcon,
    CardImage,
    CardThumbnail,
    Illustrator,
    Blog,
    Video,
}

pub enum LinkSource {
    None,
    CardboardOfTheRings,
    HallOfBeorn,
    VisionOfThePalantir,
}

pub enum Tag {
    None,
    ElrondVilya,
}
