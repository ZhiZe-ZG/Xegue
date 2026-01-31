# Rogue 5.4.4 Architecture Analysis

By claude sonnet 4.5

## Overview
Rogue 5.4.4 is a classic dungeon-crawling roguelike game written in C, using the ncurses library for terminal-based UI. The game features procedurally generated dungeons, turn-based combat, item management, and permadeath.

## Core Data Structures

### 1. **THING (Union Type)** - `rogue.h:374-408`
The central data structure representing both monsters and objects through a union:

**Monster Fields (_t)**:
- `_l_next, _l_prev`: Linked list pointers
- `_t_pos`: Position coordinates (x, y)
- `_t_type`: Character representing monster type ('A'-'Z')
- `_t_disguise`: What the monster appears as (for mimics)
- `_t_oldch`: Character that was at the monster's position
- `_t_dest`: Destination coordinates for pathfinding
- `_t_flags`: Status flags (CANHUH, CANSEE, ISBLIND, etc.)
- `_t_stats`: Combat statistics (struct stats)
- `_t_room`: Pointer to current room
- `_t_pack`: Inventory linked list

**Object Fields (_o)**:
- `_o_type`: Object type (POTION, SCROLL, WEAPON, etc.)
- `_o_pos`: Position on the map
- `_o_text`: Text for readable items
- `_o_launch`: Weapon needed to launch (for projectiles)
- `_o_packch`: Character representation in pack
- `_o_damage`, `_o_hurldmg`: Damage strings
- `_o_count`: Stack count for plural objects
- `_o_which`: Specific variant (which potion, scroll, etc.)
- `_o_hplus`, `_o_dplus`: Hit and damage bonuses
- `_o_arm`: Armor class or charges
- `_o_flags`: Object flags (ISCURSED, ISKNOW, etc.)
- `_o_group`: Group ID for object sets
- `_o_label`: Custom label

### 2. **stats** - `rogue.h:360-368`
Combat and character statistics:
- `s_str`: Strength (unsigned int)
- `s_exp`: Experience points
- `s_lvl`: Level
- `s_arm`: Armor class (lower is better)
- `s_hpt`: Current hit points
- `s_dmg`: Damage string (e.g., "1d8")
- `s_maxhp`: Maximum hit points

### 3. **room** - `rogue.h:347-356`
Room structure for dungeon layout:
- `r_pos`: Upper left corner coordinates
- `r_max`: Room dimensions
- `r_gold`: Gold position
- `r_goldval`: Gold value
- `r_flags`: Room flags (ISDARK, ISGONE, ISMAZE)
- `r_nexits`: Number of exits
- `r_exit[12]`: Array of exit coordinates

### 4. **PLACE** - `rogue.h:444-448`
Map cell information:
- `p_ch`: Character displayed
- `p_flags`: Cell flags (F_PASS, F_SEEN, F_DROPPED, etc.)
- `p_monst`: Pointer to monster at this location

### 5. **monster** - `rogue.h:453-458`
Monster template definition:
- `m_name`: Monster name string
- `m_carry`: Probability of carrying items
- `m_flags`: Monster type flags
- `m_stats`: Base statistics

### 6. **coord** - `rogue.h:326-329`
Simple coordinate structure:
- `x`: X coordinate
- `y`: Y coordinate

## Game Constants

### Map Dimensions
- `MAXROOMS`: 9 rooms per level
- `MAXTHINGS`: 9 things per level
- `MAXOBJ`: 9 objects
- `MAXPACK`: 23 items in pack
- `MAXTRAPS`: 10 traps per level
- `NUMLINES`: 24 lines
- `NUMCOLS`: 80 columns
- `AMULETLEVEL`: 26 (goal level)

### Item Types
**Potions (14 types)**: Confuse, LSD, Poison, Strength, See Invisible, Healing, Monster Detection, Trap Detection, Raise Level, Extra Healing, Haste, Restore, Blind, Levitation

**Scrolls (18 types)**: Confuse, Map, Hold, Sleep, Armor, various ID scrolls, Scare, Find, Teleport, Enchant, Create, Remove, Aggravate, Protection

**Weapons (9 types)**: Mace, Sword, Bow, Arrow, Dagger, Two-handed Sword, Dart, Shuriken, Spear

**Armor (8 types)**: Leather through Plate Mail (increasing protection)

**Rings (14 types)**: Protection, Add Strength, Sustain Strength, Search, See Invisible, various magical effects

**Sticks (14 types)**: Light, Invisibility, Lightning, Fire, Cold, Polymorph, Missile, Haste Monster, Slow Monster, etc.

### Traps (8 types)
Door, Arrow, Sleep, Bear, Teleport, Dart, Rust, Mystery

## Key Game Systems

### 1. **Level Generation** - `new_level.c`
```
new_level()
├── Clean previous level data
├── do_rooms()        → Generate 9 rooms in 3x3 grid
├── do_passages()     → Connect rooms with corridors
├── put_things()      → Place objects
├── Place traps (random based on level)
├── Place stairs
└── Place player
```

**Room Types**:
- Normal rooms: Can be lit or dark
- Maze rooms: Generated with maze algorithm
- Gone rooms: Removed to create variety
- Treasure rooms: Special rooms with extra loot (1/20 chance)

### 2. **Monster AI** - `monsters.c`, `chase.c`
- **randmonster()**: Select monster based on dungeon level
- **new_monster()**: Create and initialize monster
- **move_monst()**: Execute monster turn
- **chase()**: Pathfinding toward player
- **runto()**: Set monster to run toward target

Monster behavior influenced by:
- Distance to player
- Line of sight
- Room boundaries
- Special flags (ISGREED, ISMEAN, ISHASTE, etc.)

### 3. **Combat System** - `fight.c`
```
fight(mp, weap, thrown)
├── Identify target monster
├── Calculate hit probability
│   ├── Strength bonus (str_plus[])
│   ├── Weapon bonuses (o_hplus)
│   └── Monster armor class
├── Roll to hit: swing(at_lvl, op_arm, wplus)
├── If hit:
│   ├── Calculate damage (roll_em())
│   ├── Apply strength bonus (add_dam[])
│   ├── Apply weapon bonuses (o_dplus)
│   └── Reduce monster HP
└── Check for death
```

**Damage Calculation**:
- Parse damage string (e.g., "1d8")
- Roll dice: `roll(number, sides)`
- Add bonuses
- Special effects (poison, confusion, etc.)

### 4. **Command Processing** - `command.c`
Main game loop in `command()`:
```
while (ntimes--)
    do_daemons(BEFORE)    → Process timed effects
    do_fuses(BEFORE)      → Process delayed actions
    look(TRUE)            → Update vision
    status()              → Display status line
    readchar()            → Get player input
    Process command       → Execute player action
    do_daemons(AFTER)     → Post-action effects
    do_fuses(AFTER)       → Post-action timers
```

**Major Commands**:
- Movement: `hjklyubn` or arrow keys
- Actions: `i` (inventory), `w` (wield), `W` (wear), `q` (quaff), `r` (read), `e` (eat)
- Combat: Auto-attack when moving into monster
- Utility: `s` (search), `>` (down stairs), `?` (help), `S` (save), `Q` (quit)

### 5. **Inventory System** - `pack.c`
- Linked list of objects in player's pack
- `add_pack()`: Add item to inventory
- `leave_pack()`: Remove item
- `pack_char()`: Get next available letter
- Weight/capacity system (MAXPACK items)

### 6. **Item Identification** - Various files
- Unknown items have random names (colors for potions, materials for wands, etc.)
- ID scrolls reveal true nature
- Using items provides clues
- Knowledge persists across items of same type

### 7. **Vision & Display** - `io.c`
- `look()`: Update visible area
- Line of sight calculations
- Dark rooms require light source
- Hallucination effects (`ISHALU` flag)
- Status line updates

### 8. **Daemon/Fuse System** - `daemon.c`, `daemons.c`
**Daemons**: Recurring effects checked each turn
- `doctor()`: Natural healing
- `stomach()`: Hunger system
- `runners()`: Move running monsters

**Fuses**: One-time delayed effects
- Timed spell effects (haste, confusion, etc.)
- Delayed actions

### 9. **Save System** - `save.c`, `state.c`
- Serialize entire game state
- Includes: player, monsters, objects, map, RNG state
- Uses encryption (xcrypt.c) to prevent tampering
- Only one save per game (deleted on restore)

### 10. **Scoring System** - Score based on:
- Gold collected
- Experience gained
- Amulet retrieval
- Levels achieved

## File Organization

### Core Game Loop
- `main.c`: Entry point, initialization, option parsing
- `command.c`: Command processing, main game loop

### World Generation
- `new_level.c`: Level generation coordinator
- `rooms.c`: Room creation and layout
- `passages.c`: Corridor generation
- `things.c`: Object placement

### Entities
- `monsters.c`: Monster creation and selection
- `chase.c`: Monster AI and pathfinding
- `move.c`: Monster movement
- `fight.c`: Combat system

### Items
- `potions.c`: Potion effects
- `scrolls.c`: Scroll effects
- `rings.c`: Ring effects
- `sticks.c`: Wand/staff effects
- `weapons.c`: Weapon handling
- `armor.c`: Armor handling
- `pack.c`: Inventory management

### Player Actions
- `wear.c`: Equip items (implicit in pack.c)
- `misc.c`: Miscellaneous utilities

### Game Systems
- `daemon.c`: Daemon/fuse infrastructure
- `daemons.c`: Specific daemon implementations
- `init.c`: Initialization, item tables
- `io.c`: Input/output, display
- `list.c`: Linked list operations

### Support
- `options.c`: Game options and configuration
- `save.c`: Save/restore game
- `state.c`: State serialization
- `wizard.c`: Debug/cheat commands
- `rip.c`: Death screen
- `mach_dep.c`, `mdport.c`: Platform-specific code
- `xcrypt.c`: Save file encryption

## Global State Variables

### Player State
- `player`: THING struct for player character
- `hero`: Player position (macro for player.t_pos)
- `pstats`: Player stats (macro for player.t_stats)
- `pack`: Player inventory (macro for player.t_pack)
- `cur_weapon`, `cur_armor`, `cur_ring[]`: Equipped items

### World State
- `places[]`: Map grid (MAXCOLS × MAXLINES)
- `rooms[]`: Room array
- `passages[]`: Passage array
- `mlist`: Monster linked list
- `lvl_obj`: Objects on current level
- `level`: Current dungeon level
- `max_level`: Deepest level reached

### Game State
- `purse`: Gold amount
- `food_left`: Hunger counter
- `hungry_state`: Hunger level
- `amulet`: Whether player has amulet
- `playing`: Game in progress flag
- `running`, `count`: Movement state
- `seed`, `dnum`: RNG state

### Display State
- `msg_esc`: Message waiting flag
- `terse`: Terse message mode
- `see_floor`: Show floor objects
- `jump`: Fast display mode

## Key Algorithms

### 1. **Random Number Generation**
Uses seeded RNG (`seed`, `dnum` variables) for reproducibility

### 2. **Pathfinding** - `chase.c`
- Simple line-of-sight chase
- Diagonal movement preferred
- Room/corridor awareness

### 3. **Line of Sight** - `io.c`
- Bresenham-like algorithm
- Room lighting
- Torch radius (LAMPDIST = 3)

### 4. **Maze Generation** - `rooms.c`
Recursive backtracking algorithm for maze rooms

### 5. **Damage Rolling**
Parse strings like "2d6+3":
- Roll dice: `roll(2, 6)` → sum of 2 six-sided dice
- Add modifiers

## Design Patterns

### 1. **Union Type for Polymorphism**
`THING` union allows same linked list to hold both monsters and objects

### 2. **Macro-based Accessors**
Heavy use of macros for field access (e.g., `hero`, `pstats`, `on()`, `attach()`)

### 3. **Global State**
Extensive use of global variables (common in C, era-appropriate)

### 4. **Event System**
Daemon/fuse system provides time-based event handling

### 5. **Data-Driven Design**
Item properties stored in tables (`things[]`, `pot_info[]`, etc.)

## Special Game Mechanics

### 1. **Hunger System**
- `HUNGERTIME`: 1300 turns
- States: Not hungry → Hungry → Weak → Fainting
- Food items restore hunger

### 2. **Experience & Leveling**
- Killing monsters grants XP
- Level up increases max HP, combat ability
- XP based on monster level and HP

### 3. **Item Identification**
- Procedurally generated names each game
- Scrolls/potions have random appearances
- Knowledge builds through use

### 4. **Hallucination**
- Potion of LSD causes hallucination
- Random monster appearances
- Visual distortion

### 5. **Status Effects**
Flags like: ISHASTE, ISSLOW, ISCONFUSED, ISBLIND, ISLEVIT, ISHELD, etc.

### 6. **Ring Mechanics**
- Can wear two rings (LEFT, RIGHT)
- Effects stack
- Some rings have ongoing costs (hunger)

### 7. **Cursed Items**
- Can't be removed once equipped
- Often have negative effects
- Remove curse scroll needed

## Technical Considerations

### Memory Management
- Manual allocation/deallocation
- Linked lists for dynamic collections
- Static arrays for map grid

### Platform Portability
- `mach_dep.c`, `mdport.c` abstract platform differences
- ncurses for terminal control
- Autoconf build system

### Save File Security
- Encryption prevents save scumming
- Single save enforces permadeath
- File permissions checked

## Game Balance

### Difficulty Curve
- Deeper levels → stronger monsters
- More traps at deeper levels
- Better items at deeper levels
- Monster stats scale with level

### Resource Management
- Limited inventory (23 items)
- Hunger forces forward progress
- Consumable items (scrolls, potions)
- Equipment durability (rust traps)

This architecture represents a mature, well-designed roguelike that has influenced countless games in the genre. The clean separation of concerns, data-driven design, and clever use of C's features make it an excellent reference for understanding roguelike game development.
