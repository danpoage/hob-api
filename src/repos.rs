use std::collections::{HashMap, HashSet};

use crate::enums::{CardMainType, DeckType, ProductCode, ProductType, Stat, StatType, Trait};
use crate::structs::{Card, CardType, Link, Product, ProductItem, Scenario, Side, SideInfo};
pub struct ProductRepo {}

impl ProductRepo {
    pub fn list() -> Vec<Product> {

        let gandalf: Card = Card{
            front: Side{
                id: "Gandalf-Core".to_string(),
                title: "Gandalf".to_string(),
                card_type: CardType{
                    main_type: CardMainType::Ally,
                    sub_type: None,
                    deck_type: DeckType::Player,
                    is_unique: true
                },
                stats: HashMap::from([
                    (StatType::ResourceCost, Stat::Number(5)),
                    (StatType::Willpower, Stat::Number(4)),
                    (StatType::Attack, Stat::Number(4)),
                    (StatType::Defense, Stat::Number(4)),
                    (StatType::HitPoints, Stat::Number(4)),
                ]),
                traits: HashSet::from([Trait::Istari]),
                keywords: HashSet::from([])
            },
            back: None,
            quest_info: None,
            links: Vec::<Link>::new(),
            tags: HashSet::<String>::new()
        };

        let mec01 = Product{
            title: "Core Set".to_string(),
            code: ProductCode::MEC01,
            product_type: ProductType::CoreSet,
            items: Vec::<ProductItem>::from([
                ProductItem{
                    card: gandalf,
                    front_info: SideInfo{
                        number: 73,
                        letter: None,
                        links: None,
                        tags: None,
                    },
                    back_info: None,
                    quantity: 4,
                }
            ]),
            scenarios: Vec::<Scenario>::new(),
            links: Vec::<Link>::new(),
            tags: HashSet::<String>::new(),
        };
    
        let mec02 = Product {
            title: "The Hunt for Gollum".to_string(),
            code: ProductCode::MEC02,
            product_type: ProductType::AdventurePack,
            items: Vec::<ProductItem>::new(),
            scenarios: Vec::<Scenario>::new(),
            links: Vec::<Link>::new(),
            tags: HashSet::<String>::new(),
        };

        let products = Vec::<Product>::from([mec01, mec02]);

        products
    }
}