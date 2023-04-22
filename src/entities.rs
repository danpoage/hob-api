use crate::stats::Stat;
use crate::traits::IsCard;

pub enum Card {
    Hero(Hero),
    Ally(Ally)
}

pub struct Hero {
    threat_cost: Stat
}

pub struct Ally {
    resource_cost: Stat,
    is_unique: bool
}

pub struct CardSet {
    cards: Vec<Box<dyn IsCard>>,
}
