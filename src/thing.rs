use glam::IVec2;

#[derive(Debug, Clone)]
pub enum Thing {
    Monster(MonsterThing),
    Object(ObjectThing),
}

#[derive(Debug, Clone)]
pub struct MonsterThing {
    pub pos: IVec2,
    pub turn: bool,
    pub kind: char,
    pub disguise: char,
    pub old_ch: char,
    pub dest: Option<IVec2>,
    pub flags: i16,
    //pub stats: Stats,
    pub room_id: Option<usize>,
    pub pack: Vec<Thing>, // or Vec<ObjectThing> if you want only objects
    pub reserved: i32,
}

#[derive(Debug, Clone)]
pub struct ObjectThing {
    pub kind: i32,
    pub pos: IVec2,
    pub text: Option<String>,
    pub launch: i32,
    pub pack_ch: char,
    pub damage: String,
    pub hurl_dmg: String,
    pub count: i32,
    pub which: i32,
    pub hplus: i32,
    pub dplus: i32,
    pub armor: i32,   // or charges / gold value depending on type
    pub flags: i32,
    pub group: i32,
    pub label: Option<String>,
}