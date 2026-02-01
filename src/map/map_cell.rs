use crate::thing::Thing;

#[derive(Clone)]
pub struct MapCell {
    pub ch: char,
    pub flags: u8,
    pub monst: Option<Thing>,
}
