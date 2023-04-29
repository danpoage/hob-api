pub enum Stat {
    NA,
    X,
    Number(u8),
    Letter(char),
    Sphere(Sphere),
    Trait(Trait),
    Keyword(Keyword),
}

pub enum StatType {
    Willpower,
    Attack,
    Defense,
    HitPoints,
    Trait,
    Keyword,
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
