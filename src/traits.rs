use  crate::stats::{Sphere,Stat};

pub trait IsCard {
}

pub trait HasCost {
    fn cost(&self) -> Stat;
}

pub trait HasCards {
    fn cards(&self) -> Vec<Box<dyn IsCard>>;
}

pub trait HasSphere {
    fn sphere(&self) -> Option<Sphere>;
}