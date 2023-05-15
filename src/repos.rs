use std::collections::HashSet;

use crate::enums::{ProductCode, ProductType};
use crate::structs::{Product, ProductItem, Scenario, Link};
pub struct ProductRepo {}

impl ProductRepo {
    pub fn list() -> Vec<Product> {
        let mut products = Vec::<Product>::new();

        let mec01 = Product{
            title: "Core Set".to_string(),
            code: ProductCode::MEC01,
            product_type: ProductType::CoreSet,
            items: Vec::<ProductItem>::new(),
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
    
        products.push(mec01);
        products.push(mec02);    

        products
    }
}