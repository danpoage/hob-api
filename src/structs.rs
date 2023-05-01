use std::collections::HashMap;
use std::collections::HashSet;

use crate::enums::*;
//use crate::traits::IsCard;

pub struct Card {
    pub front: Side,
    pub back: Option<Side>,
    pub quest_info: Option<QuestInfo>,
    pub links: Vec<Link>,
    pub tags: HashSet<Tag>,
}

pub struct Side {
    pub id: String,
    pub title: String,
    pub card_type: CardType,
    pub stats: HashMap<StatType, Stat>,
    pub traits: HashSet<Trait>,
    pub keywords: HashSet<Keyword>,
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

pub struct Link {
    pub link_source: LinkSource,
    pub link_type: LinkType,
    pub url: String,
}

pub struct CardType {
    pub main_type: CardMainType,
    pub sub_type: CardSubType,
    pub deck_type: DeckType,
    pub is_unique: bool,
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

pub struct ProductItem {
    card: Card,
    front_info: SideInfo,
    back_info: Option<SideInfo>,
    quantity: u8,
}

pub struct SideInfo {
    number: u16,
    letter: Option<char>,
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

