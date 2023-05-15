use crate::structs::Link;
//use crate::enums::Tag;
use std::collections::HashSet;

pub trait Linked {
    fn links(&self) -> Vec<Link>;
}

pub trait Tagged {
    fn tags(&self) -> HashSet<String>;
}
