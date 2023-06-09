use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::enums::*;
//use crate::traits::IsCard;

#[derive(Debug, Deserialize, Serialize)]
pub struct Card {
    pub front: Side,
    pub back: Option<Side>,
    pub quest_info: Option<QuestInfo>,
    pub links: Vec<Link>,
    pub tags: HashSet<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Side {
    pub id: String,
    pub title: String,
    pub card_type: CardType,
    pub stats: HashMap<StatType, Stat>,
    pub traits: HashSet<Trait>,
    pub keywords: HashSet<Keyword>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QuestInfo {
    pub encounter_set: EncounterSet,
    pub included_sets: Vec<EncounterSet>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EncounterSet {
    name: String,
    set_type: EncounterSetType,
    links: Vec<Link>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Link {
    pub link_source: LinkSource,
    pub link_type: LinkType,
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CardType {
    pub main_type: CardMainType,
    pub sub_type: Option<CardSubType>,
    pub deck_type: DeckType,
    pub is_unique: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Product {
    pub title: String,
    pub code: ProductCode,
    pub product_type: ProductType,
    pub items: Vec<ProductItem>,
    pub scenarios: Vec<Scenario>,
    pub links: Vec<Link>,
    pub tags: HashSet<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProductItem {
    pub card: Card,
    pub front_info: SideInfo,
    pub back_info: Option<SideInfo>,
    pub quantity: u8,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SideInfo {
    pub number: u16,
    pub letter: Option<char>,
    pub links: Option<Vec<Link>>,
    pub tags: Option<HashSet<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Scenario {
    title: String,
    encounter_sets: Vec<EncounterSet>,
    items: Vec<ScenarioItem>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ScenarioItem {
    card: Card,
    quantity: u8,
    easy_quantity: Option<u8>,
    nightmare_quantity: Option<u8>,
}

