# Xegue: Rust Rewrite Plan for Rogue 5.4.4

By claude sonnet 4.5

## Project Overview
Xegue is a modern Rust implementation of Rogue 5.4.4, maintaining the classic gameplay while leveraging Rust's safety guarantees, modern language features, and improved architecture.

## Goals
1. **Faithful Recreation**: Preserve the original game mechanics and feel
2. **Modern Architecture**: Use Rust idioms and best practices
3. **Maintainability**: Clear, documented, modular code
4. **Safety**: Leverage Rust's ownership system to prevent bugs
5. **Extensibility**: Design for future enhancements
6. **Cross-platform**: Work on major platforms (Windows, macOS, Linux)

## Technology Stack
- **Language**: Rust 2021 edition
- **Terminal UI**: `ncurses` (already in Cargo.toml) or consider `crossterm` + `tui-rs` for pure Rust
- **Serialization**: `serde` for save files
- **Random**: `rand` crate
- **Error Handling**: `anyhow` or `thiserror`
- **Optional**: `specs` or `hecs` for ECS if going that route

## Architecture Strategy

### Design Philosophy
**Option A: Direct Translation (Recommended for Phase 1)**
- Maintain similar structure to C version
- Easier to verify correctness
- Faster initial development
- Later refactor to more idiomatic Rust

**Option B: Modern Rust Idioms from Start**
- Entity Component System (ECS)
- Trait-based design
- Functional programming patterns
- Longer initial development

**Recommendation**: Start with Option A, refactor to Option B incrementally

## Module Structure

```
src/
├── main.rs                 // Entry point, game loop
├── lib.rs                  // Library exports
│
├── core/
│   ├── mod.rs              // Core types and constants
│   ├── coord.rs            // Coordinate type
│   ├── stats.rs            // Combat statistics
│   ├── constants.rs        // Game constants
│   └── rng.rs              // Random number generation
│
├── entity/
│   ├── mod.rs
│   ├── thing.rs            // Entity enum (Player/Monster/Object)
│   ├── monster.rs          // Monster types and behavior
│   ├── object.rs           // Object types and properties
│   ├── player.rs           // Player specific logic
│   └── flags.rs            // Status flags
│
├── world/
│   ├── mod.rs
│   ├── level.rs            // Level generation and management
│   ├── room.rs             // Room generation
│   ├── passage.rs          // Corridor generation
│   ├── maze.rs             // Maze algorithm
│   ├── place.rs            // Map cells
│   └── trap.rs             // Trap types and effects
│
├── items/
│   ├── mod.rs
│   ├── potion.rs           // Potion types and effects
│   ├── scroll.rs           // Scroll types and effects
│   ├── weapon.rs           // Weapon types
│   ├── armor.rs            // Armor types
│   ├── ring.rs             // Ring types and effects
│   ├── stick.rs            // Wand/staff types
│   └── pack.rs             // Inventory management
│
├── systems/
│   ├── mod.rs
│   ├── combat.rs           // Combat calculations
│   ├── movement.rs         // Movement and pathfinding
│   ├── ai.rs               // Monster AI
│   ├── vision.rs           // Line of sight, lighting
│   ├── effect.rs           // Status effects, daemons, fuses
│   ├── hunger.rs           // Hunger system
│   └── experience.rs       // XP and leveling
│
├── ui/
│   ├── mod.rs
│   ├── display.rs          // Terminal rendering
│   ├── input.rs            // Input handling
│   ├── messages.rs         // Message buffer
│   ├── status.rs           // Status line
│   └── menu.rs             // Menus (inventory, help, etc.)
│
├── game/
│   ├── mod.rs
│   ├── state.rs            // Game state management
│   ├── command.rs          // Command processing
│   ├── options.rs          // Game options
│   └── save.rs             // Save/load system
│
└── util/
    ├── mod.rs
    ├── names.rs            // Name generation for items
    └── tables.rs           // Data tables (monster stats, item info)
```

## Core Type Definitions

### Phase 1: Enum-based (Similar to C union)

```rust
// src/entity/thing.rs
pub enum Thing {
    Monster(Monster),
    Object(Object),
}

pub struct Monster {
    pub id: EntityId,
    pub pos: Coord,
    pub kind: MonsterKind,
    pub stats: Stats,
    pub flags: MonsterFlags,
    pub room: Option<usize>,
    pub pack: Vec<Object>,
    pub dest: Option<Coord>,
    pub disguise: char,
    pub old_char: char,
    pub turn: bool,
}

pub struct Object {
    pub id: EntityId,
    pub kind: ObjectKind,
    pub pos: Coord,
    pub count: u32,
    pub flags: ObjectFlags,
    pub enchantment: Enchantment,
    pub identified: bool,
    pub pack_char: Option<char>,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MonsterKind {
    Aquator,
    Bat,
    Centaur,
    Dragon,
    Emu,
    // ... all 26 monster types
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ObjectKind {
    Potion(PotionType),
    Scroll(ScrollType),
    Weapon(WeaponType),
    Armor(ArmorType),
    Ring(RingType),
    Stick(StickType),
    Food,
    Gold(u32),
    Amulet,
}
```

### Key Types

```rust
// src/core/coord.rs
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Self { Self { x, y } }
    pub fn distance(&self, other: &Coord) -> i32 { /* ... */ }
}

// src/core/stats.rs
pub struct Stats {
    pub strength: u32,
    pub experience: u32,
    pub level: u32,
    pub armor: i32,
    pub hp: i32,
    pub max_hp: i32,
    pub damage: DamageRoll,
}

pub struct DamageRoll {
    pub dice: u32,
    pub sides: u32,
    pub bonus: i32,
}

// src/world/room.rs
pub struct Room {
    pub pos: Coord,
    pub size: Coord,
    pub flags: RoomFlags,
    pub exits: Vec<Coord>,
    pub gold_pos: Option<Coord>,
    pub gold_value: u32,
}

bitflags! {
    pub struct RoomFlags: u32 {
        const DARK = 0b0001;
        const GONE = 0b0010;
        const MAZE = 0b0100;
    }
}

// src/world/place.rs
pub struct Place {
    pub ch: char,
    pub flags: PlaceFlags,
    pub monster: Option<EntityId>,
}
```

## Implementation Phases

### Phase 1: Foundation (Weeks 1-3)
**Goal**: Basic game loop and rendering

- [ ] Set up project structure and modules
- [ ] Implement core types (Coord, Stats, Thing)
- [ ] Basic ncurses wrapper
- [ ] Message system
- [ ] Input handling
- [ ] Simple game state

**Milestone**: Display "Hello, Xegue!" with proper ncurses setup

### Phase 2: World Generation (Weeks 4-6)
**Goal**: Generate and display dungeon levels

- [ ] Room generation (do_rooms)
- [ ] Passage generation (do_passages)
- [ ] Maze rooms
- [ ] Map rendering
- [ ] Player placement
- [ ] Stairs placement

**Milestone**: Navigate through a generated dungeon

### Phase 3: Player Movement (Weeks 7-8)
**Goal**: Player can move and interact with world

- [ ] Movement commands (hjklyubn)
- [ ] Running/counting
- [ ] Room detection
- [ ] Vision system (line of sight)
- [ ] Status line

**Milestone**: Player can explore full dungeon

### Phase 4: Objects (Weeks 9-11)
**Goal**: Items spawn and can be picked up

- [ ] Object placement (put_things)
- [ ] Inventory system
- [ ] Pick up items
- [ ] Drop items
- [ ] Item display
- [ ] Pack management

**Milestone**: Can pick up and manage inventory

### Phase 5: Monsters (Weeks 12-15)
**Goal**: Monsters spawn and can be seen

- [ ] Monster definitions (monster table)
- [ ] Monster spawning
- [ ] Monster rendering
- [ ] Basic AI (wander)
- [ ] Chase algorithm
- [ ] Monster movement

**Milestone**: Monsters spawn and chase player

### Phase 6: Combat (Weeks 16-18)
**Goal**: Combat system works

- [ ] Attack calculation
- [ ] Damage calculation
- [ ] Hit/miss messages
- [ ] Death handling (player and monsters)
- [ ] Experience gain
- [ ] Leveling up

**Milestone**: Can fight and kill monsters

### Phase 7: Item Effects (Weeks 19-24)
**Goal**: Items have proper effects

- [ ] Weapon system
  - Wielding weapons
  - Weapon damage
  - Projectiles (arrows, darts)
- [ ] Armor system
  - Wearing armor
  - Armor class calculation
- [ ] Potion effects (all 14 types)
- [ ] Scroll effects (all 18 types)
- [ ] Ring effects (all 14 types)
- [ ] Stick effects (all 14 types)
- [ ] Food and hunger

**Milestone**: All item types functional

### Phase 8: Game Systems (Weeks 25-28)
**Goal**: All game mechanics work

- [ ] Daemon system (recurring effects)
- [ ] Fuse system (delayed effects)
- [ ] Hunger system
- [ ] Status effects (confusion, blind, haste, etc.)
- [ ] Traps (all 8 types)
- [ ] Search functionality
- [ ] Doors (open/close)
- [ ] Treasure rooms

**Milestone**: Full game mechanics working

### Phase 9: Polish (Weeks 29-32)
**Goal**: Game is feature complete

- [ ] Save/load system
- [ ] Scoring system
- [ ] Options system
- [ ] Help system
- [ ] Death screen (RIP)
- [ ] Item identification system
- [ ] Name generation
- [ ] Message improvements
- [ ] Bug fixes

**Milestone**: Playable from start to finish

### Phase 10: Testing & Refinement (Weeks 33-36)
**Goal**: Game is stable and polished

- [ ] Playtesting
- [ ] Balance adjustments
- [ ] Performance optimization
- [ ] Documentation
- [ ] Code cleanup
- [ ] Unit tests for core systems
- [ ] Integration tests

**Milestone**: Release version 1.0

## Rust-Specific Considerations

### 1. **Entity Management**
**C Version**: Uses raw pointers and linked lists
**Rust Version**: 
- Option A: Vec with Entity IDs (indices)
- Option B: HashMap<EntityId, Entity>
- Option C: ECS library (specs/hecs)

**Recommendation**: Start with Vec<Option<Thing>> with stable indices

```rust
pub struct EntityId(usize);

pub struct World {
    entities: Vec<Option<Thing>>,
    free_list: Vec<EntityId>,
}

impl World {
    pub fn spawn(&mut self, thing: Thing) -> EntityId {
        if let Some(id) = self.free_list.pop() {
            self.entities[id.0] = Some(thing);
            id
        } else {
            let id = EntityId(self.entities.len());
            self.entities.push(Some(thing));
            id
        }
    }
    
    pub fn despawn(&mut self, id: EntityId) {
        self.entities[id.0] = None;
        self.free_list.push(id);
    }
}
```

### 2. **Global State**
**C Version**: Extensive global variables
**Rust Version**: 
- Wrap in GameState struct
- Pass as mutable reference
- Consider RefCell/Rc for shared ownership if needed

```rust
pub struct GameState {
    pub player: Player,
    pub level: Level,
    pub monsters: Vec<Monster>,
    pub objects: Vec<Object>,
    pub rng: Rng,
    pub messages: MessageLog,
    pub options: Options,
    pub turn: u64,
}
```

### 3. **Error Handling**
**C Version**: Return codes, global errno
**Rust Version**: Result<T, E> for fallible operations

```rust
pub enum GameError {
    SaveFailed(std::io::Error),
    LoadFailed(String),
    InvalidCommand,
    // ...
}

pub type GameResult<T> = Result<T, GameError>;
```

### 4. **Random Number Generation**
**C Version**: Seeded rand()
**Rust Version**: Use `rand` crate with seedable RNG

```rust
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

pub struct GameRng {
    rng: StdRng,
}

impl GameRng {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: StdRng::seed_from_u64(seed),
        }
    }
    
    pub fn roll(&mut self, num: u32, sides: u32) -> u32 {
        (0..num).map(|_| self.rng.gen_range(1..=sides)).sum()
    }
}
```

### 5. **String Handling**
**C Version**: char* with manual memory
**Rust Version**: String and &str

```rust
// For item names that change
pub struct ItemInfo {
    pub true_name: &'static str,
    pub random_name: String,  // Generated per game
    pub known: bool,
}
```

### 6. **Flags and Bitsets**
**C Version**: Integer bitflags
**Rust Version**: Use `bitflags!` macro

```rust
bitflags! {
    pub struct MonsterFlags: u32 {
        const CAN_CONFUSE = 0b0000_0001;
        const CAN_SEE_INVISIBLE = 0b0000_0010;
        const IS_BLIND = 0b0000_0100;
        const IS_CANCELLED = 0b0000_1000;
        const IS_FOUND = 0b0001_0000;
        const IS_GREEDY = 0b0010_0000;
        const IS_HASTED = 0b0100_0000;
        const IS_HELD = 0b1000_0000;
        // ... more flags
    }
}
```

### 7. **Function Pointers (Daemons/Fuses)**
**C Version**: void (*func)()
**Rust Version**: Trait objects or enum

```rust
pub enum DaemonFn {
    Doctor,
    Stomach,
    Runners,
}

pub struct Daemon {
    pub func: DaemonFn,
    pub arg: i32,
}

impl Daemon {
    pub fn execute(&self, state: &mut GameState) {
        match self.func {
            DaemonFn::Doctor => doctor(state, self.arg),
            DaemonFn::Stomach => stomach(state, self.arg),
            DaemonFn::Runners => runners(state, self.arg),
        }
    }
}
```

## Testing Strategy

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_damage_calculation() {
        let roll = DamageRoll::new(2, 6, 3);
        let mut rng = GameRng::new(42);
        let damage = roll.calculate(&mut rng);
        assert!(damage >= 5 && damage <= 15);
    }
    
    #[test]
    fn test_pathfinding() {
        let start = Coord::new(0, 0);
        let end = Coord::new(5, 5);
        let path = find_path(&start, &end, &|_| true);
        assert!(path.is_some());
    }
}
```

### Integration Tests
```rust
// tests/gameplay_tests.rs
#[test]
fn test_new_game_creation() {
    let mut game = Game::new(GameOptions::default());
    game.initialize();
    assert!(game.state.player.stats.hp > 0);
    assert!(!game.state.level.rooms.is_empty());
}
```

## Data-Driven Design

### Monster Table
```rust
// src/util/tables.rs
pub struct MonsterTemplate {
    pub name: &'static str,
    pub carry_chance: u32,
    pub flags: MonsterFlags,
    pub base_stats: Stats,
}

lazy_static! {
    pub static ref MONSTERS: HashMap<char, MonsterTemplate> = {
        let mut m = HashMap::new();
        m.insert('A', MonsterTemplate {
            name: "aquator",
            carry_chance: 15,
            flags: MonsterFlags::IS_MEAN,
            base_stats: Stats::new(5, 0, 5, 2, 50, "0d0"),
        });
        // ... all 26 monsters
        m
    };
}
```

### Item Tables
Similar approach for potions, scrolls, weapons, etc.

## Build and Distribution

### Cargo.toml
```toml
[package]
name = "xegue"
version = "0.1.0"
edition = "2021"
authors = ["Your Name"]
license = "MIT"

[dependencies]
ncurses = "6.0"
rand = "0.8"
bitflags = "2.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"
lazy_static = "1.4"

[dev-dependencies]
criterion = "0.5"

[[bin]]
name = "xegue"
path = "src/main.rs"

[profile.release]
opt-level = 3
lto = true
```

### Build Instructions
```bash
# Development
cargo build

# Release
cargo build --release

# Run
cargo run

# Test
cargo test

# Documentation
cargo doc --open
```

## Migration Strategy: C to Rust

### Step-by-Step Translation Process

1. **Understand C module**: Read and document C code
2. **Define Rust types**: Create equivalent types
3. **Implement functions**: Translate logic to Rust
4. **Write tests**: Verify behavior matches C version
5. **Integrate**: Connect to rest of system
6. **Refactor**: Improve to be more idiomatic

### Example: Translating `roll()` function

**C Version (misc.c)**:
```c
int roll(int number, int sides) {
    int dtotal = 0;
    while (number--)
        dtotal += rnd(sides) + 1;
    return dtotal;
}
```

**Rust Version (src/core/rng.rs)**:
```rust
pub fn roll(rng: &mut impl Rng, number: u32, sides: u32) -> u32 {
    (0..number)
        .map(|_| rng.gen_range(1..=sides))
        .sum()
}
```

### Priority Order for Translation

1. **Core utilities**: coord.rs, constants.rs, rng.rs
2. **Basic types**: stats.rs, flags.rs
3. **World structure**: place.rs, room.rs
4. **Display**: UI system
5. **Generation**: Level generation
6. **Entities**: Monster and object types
7. **Systems**: Combat, movement, AI
8. **Items**: Effects and usage
9. **Game loop**: Command processing
10. **Persistence**: Save/load

## Documentation Standards

### Code Comments
```rust
/// Calculates damage for an attack.
///
/// # Arguments
/// * `attacker` - The attacking entity's stats
/// * `defender` - The defending entity's stats
/// * `weapon` - Optional weapon being used
/// * `rng` - Random number generator
///
/// # Returns
/// The amount of damage dealt (always >= 0)
///
/// # Examples
/// ```
/// let damage = calculate_damage(&player_stats, &monster_stats, Some(&sword), &mut rng);
/// ```
pub fn calculate_damage(
    attacker: &Stats,
    defender: &Stats,
    weapon: Option<&Weapon>,
    rng: &mut GameRng,
) -> u32 {
    // Implementation
}
```

### Module Documentation
Each module should have a header explaining its purpose and usage.

## Performance Considerations

### Optimization Opportunities
1. **Spatial hashing**: For entity lookups by position
2. **Dirty flags**: Only redraw changed parts of screen
3. **Object pooling**: Reuse entity allocations
4. **Lazy evaluation**: Defer expensive calculations

### Profiling
```bash
# Profile with flamegraph
cargo install flamegraph
cargo flamegraph

# Benchmark critical paths
cargo bench
```

## Future Enhancements (Post 1.0)

### Quality of Life
- Mouse support
- Tile graphics mode
- Sound effects
- Multiple save slots
- Replay system
- Seeded runs

### Extended Content
- New monster types
- New item types
- Additional levels
- Boss monsters
- Side quests

### Modern Features
- Daily challenge mode
- Achievements
- Statistics tracking
- Online leaderboards
- Mod support

## Learning Resources

### Rust Learning
- The Rust Book: https://doc.rust-lang.org/book/
- Rust by Example: https://doc.rust-lang.org/rust-by-example/
- Rustlings: https://github.com/rust-lang/rustlings

### Game Development
- Roguelike Tutorial in Rust: https://bfnightly.bracketproductions.com/
- Game Programming Patterns: https://gameprogrammingpatterns.com/

### Original Rogue
- Source code analysis (this document)
- Playing original Rogue for reference
- Roguelike Development resources: r/roguelikedev

## Version Control Strategy

### Git Workflow
- Main branch: stable releases
- Develop branch: integration
- Feature branches: individual features
- Tag releases: v0.1.0, v0.2.0, etc.

### Commit Messages
```
feat(combat): implement basic attack calculation
fix(display): correct status line positioning
docs(architecture): add monster AI explanation
test(items): add potion effect tests
refactor(entity): simplify Thing enum
```

## Success Criteria

### Alpha (v0.1)
- [ ] Can generate and display levels
- [ ] Player can move
- [ ] Basic combat works
- [ ] Can pick up items

### Beta (v0.5)
- [ ] All item types functional
- [ ] All monster types present
- [ ] Can reach and retrieve amulet
- [ ] Save/load works

### Release (v1.0)
- [ ] Feature-complete vs original Rogue
- [ ] No game-breaking bugs
- [ ] Playable start to finish
- [ ] Documentation complete
- [ ] Builds on all platforms

## Conclusion

This rewrite plan provides a structured approach to recreating Rogue 5.4.4 in Rust. By working through phases systematically, we can build confidence in each system before moving to the next. The key is to maintain playability at each milestone while gradually adding complexity.

Start with Phase 1 and work forward. Don't try to perfect everything on the first pass—get it working, then refactor. Rust's compiler will help catch mistakes along the way.

Good luck, and may your dungeon crawling be bug-free! 🦀⚔️
