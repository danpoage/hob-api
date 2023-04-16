enum Sphere {
    Leadership,
    Tactics,
    Spirit,
    Lore,
    Neutral,
    Baggins,
    Fellowship
}

enum Keyword {
    Sentinel,
    Ranged,
}

pub trait HasCost {
    fn cost(&self) -> u8;
}