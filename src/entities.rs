use std::collections::HashMap;
use std::collections::HashSet;

use crate::stats::*;
//use crate::traits::IsCard;

pub struct Card {
    pub id: String,
    pub title: String,
    pub card_type: CardType,
    pub stats: HashMap<StatType, Stat>,
    pub traits: HashSet<Trait>,
    pub keywords: HashSet<Keyword>,
    pub quest_info: Option<QuestInfo>,
    pub links: Vec<Link>,
    pub tags: HashSet<Tag>,
}

pub struct QuestInfo {
    pub encounter_set: EncounterSet,
    pub included_sets: Vec<EncounterSet>,
}

pub struct EncounterSet {
    name: String,
    set_type: EncounterSetType,
    links: Vec<Link>,
}

pub enum EncounterSetType {
    Cycle,
    Standalone,
    Nightmare,
    Saga,
}

pub struct Link {
    link_source: LinkSource,
    link_type: LinkType,
    url: String,
}

pub enum LinkType {
    None,
    EncounterSetIcon,
    CardImage,
    CardThumbnail,
    Blog,
    Video,
}

pub enum LinkSource {
    None,
    CardboardOfTheRings,
    HallOfBeorn,
    VisionOfThePalantir,
}

pub struct CardType {
    main_type: CardMainType,
    sub_type: CardSubType,
    is_unique: bool,
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

pub struct Product {
    title: String,
    code: ProductCode,
    product_type: ProductType,
    items: Vec<ProductItem>,
    scenarios: Vec<Scenario>,
    links: Vec<Link>,
    tags: HashSet<Tag>,
}

pub enum ProductCode {
    MEC01,
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

pub struct ProductItem {
    card: Card,
    number: u16,
    quantity: u8,
    links: Vec<Link>,
    tags: HashSet<Tag>,
}

pub struct Scenario {
    title: String,
    encounter_sets: Vec<EncounterSet>,
    items: Vec<ScenarioItem>,
}

pub struct ScenarioItem {
    card: Card,
    quantity: u8,
    easy_quantity: Option<u8>,
    nightmare_quantity: Option<u8>,
}

pub enum Tag {
    None,
    ElrondVilya,
}
