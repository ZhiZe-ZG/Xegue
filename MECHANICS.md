# Rogue 5.4.4 Quick Reference Guide

## Essential Game Mechanics Cheat Sheet

This document provides quick reference for implementing core Rogue mechanics.

## Combat System

### To-Hit Calculation
```
hit_chance = swing(attacker_level, defender_armor, weapon_bonus)

swing(level, armor, plus):
    need = (21 - level - armor) / 2
    chance = (20 - need) + plus
    roll d20, hit if roll >= need
```

### Damage Calculation
```
base_damage = roll(weapon_dice, weapon_sides)
strength_mod = add_dam[strength] or str_plus[strength]
total_damage = base_damage + strength_mod + weapon_plus
```

### Strength Modifiers
```c
// To-hit bonus
str_plus[] = {-7,-6,-5,-4,-3,-2,-1,0,0,0,0,0,0,0,0,0,0,1,1,1,1,2,2,2,2,2,2,2,2,2,2,3}

// Damage bonus  
add_dam[] = {-7,-6,-5,-4,-3,-2,-1,0,0,0,0,0,0,0,0,0,1,1,2,3,3,4,5,5,5,5,5,5,5,5,5,6}
```

### Armor Class
- Lower is better
- Starts at 10 (no armor)
- Leather armor: 8, Ring mail: 7, ... Plate mail: 3
- Rings of protection: -1 per plus
- Dexterity doesn't affect AC in original Rogue

## Monster Stats Table

```
Letter | Name          | Level | HP   | AC | Damage   | XP  | Flags
-------|---------------|-------|------|----|---------|----|------------------
A      | Aquator       | 5     | 5d8  | 2  | 0d0     | 20 | ISMEAN (rusts armor)
B      | Bat           | 1     | 1d8  | 3  | 1d2     | 1  | ISFLY
C      | Centaur       | 4     | 4d8  | 4  | 1d2/1d5 | 17 | 
D      | Dragon        | 10    | 10d8 | -1 | 1d8/1d8/3d10 | 9000 | ISMEAN
E      | Emu           | 1     | 1d8  | 7  | 1d2     | 2  | ISMEAN
F      | Venus Flytrap | 8     | 8d8  | 3  | 4d4     | 90 | ISMEAN
G      | Griffin       | 13    | 13d8 | 2  | 4d3/3d5 | 2000 | ISFLY, ISMEAN, ISREGEN
H      | Hobgoblin     | 1     | 1d8  | 5  | 1d8     | 3  | ISMEAN
I      | Ice Monster   | 1     | 1d8  | 9  | 0d0     | 5  | 
J      | Jabberwock    | 15    | 15d8 | 6  | 2d12/2d4 | 3000 | 
K      | Kestrel       | 1     | 1d8  | 7  | 1d4     | 1  | ISMEAN, ISFLY
L      | Leprechaun    | 10    | 10d8 | 8  | 1d1     | 0  | (steals gold)
M      | Medusa        | 8     | 8d8  | 2  | 3d4/3d4/2d5 | 200 | ISMEAN
N      | Nymph         | 3     | 3d8  | 9  | 0d0     | 37 | (steals items)
O      | Orc           | 1     | 1d8  | 6  | 1d8     | 5  | ISGREED
P      | Phantom       | 8     | 8d8  | 3  | 4d4     | 120 | ISINVIS
Q      | Quagga        | 3     | 3d8  | 3  | 1d5/1d5 | 32 | ISMEAN
R      | Rattlesnake   | 2     | 2d8  | 3  | 1d6     | 9  | ISMEAN
S      | Snake         | 1     | 1d8  | 5  | 1d3     | 2  | ISMEAN
T      | Troll         | 6     | 6d8  | 4  | 1d8/1d8/2d6 | 120 | ISREGEN, ISMEAN
U      | Black Unicorn | 7     | 7d8  | 2  | 1d9/1d9/2d9 | 190 | ISMEAN
V      | Vampire       | 8     | 8d8  | 1  | 1d10    | 350 | ISREGEN, ISMEAN
W      | Wraith        | 5     | 5d8  | 4  | 1d6     | 55 | (level drain)
X      | Xeroc         | 7     | 7d8  | 7  | 3d4     | 100 | (mimic)
Y      | Yeti          | 4     | 4d8  | 6  | 1d6/1d6 | 50 | 
Z      | Zombie        | 2     | 2d8  | 8  | 1d8     | 6  | ISMEAN
```

## Item Types and Probabilities

### Potions (14 types, probabilities out of 41)
```
Type            | Prob | Effect
----------------|------|------------------------------------------
Confusion       | 7    | Confuse player for 20 turns
Hallucination   | 8    | Hallucinate for 850 turns
Poison          | 8    | Lose 1-3 hp, lose 1 strength (can't go below 3)
Gain Strength   | 13   | Increase strength by 1
See Invisible   | 3    | See invisible monsters for 850 turns
Healing         | 13   | Heal 1d8 HP (or cure blindness)
Monster Detect  | 6    | Show all monsters for 1 turn
Magic Detect    | 6    | Show all magic items for 1 turn
Raise Level     | 2    | Gain 1 experience level
Extra Healing   | 5    | Heal to full, increase max HP
Haste Self      | 5    | Double speed for 3-13 turns
Restore Str     | 13   | Restore strength to max
Blindness       | 5    | Blind for 850 turns
Levitation      | 6    | Levitate for 3-13 turns
```

### Scrolls (18 types, probabilities out of 73)
```
Type               | Prob | Effect
-------------------|------|------------------------------------------
Confuse Monster    | 7    | Next monster hit is confused
Magic Mapping      | 4    | Reveal entire level
Hold Monster       | 2    | Freeze monsters in room for 3-13 turns
Sleep              | 3    | Sleep for 4-8 turns (dangerous!)
Enchant Armor      | 7    | +1 AC to armor (can uncurse)
Identify Potion    | 10   | Identify 1 potion
Identify Scroll    | 10   | Identify 1 scroll
Identify Weapon    | 6    | Identify 1 weapon
Identify Armor     | 7    | Identify 1 armor
Identify Ring/Wand | 10   | Identify 1 ring or wand
Scare Monster      | 3    | Drop to ward off monsters
Food Detection     | 2    | Show food on level
Teleportation      | 5    | Teleport to random location
Enchant Weapon     | 8    | +1 to-hit and damage (can uncurse)
Create Monster     | 4    | Summon a monster
Remove Curse       | 7    | Uncurse worn items
Aggravate Monsters | 1    | Wake and anger all monsters
Protect Armor      | 2    | Make armor rustproof
```

### Weapons (probabilities out of 36)
```
Type            | Prob | Damage | Throw | Launch
----------------|------|--------|-------|--------
Mace            | 11   | 2d4    | 1d3   | 
Sword           | 12   | 3d4    | 1d2   |
Bow             | 12   | 1d1    | 1d1   | Arrows
Arrow           | 12   | 1d1    | 2d3   | Bow
Dagger          | 8    | 1d6    | 1d4   |
Two-hand Sword  | 10   | 4d4    | 1d2   |
Dart            | 12   | 1d1    | 1d3   |
Shuriken        | 12   | 1d2    | 2d4   |
Spear           | 12   | 2d3    | 1d6   |
```

### Armor (probabilities out of 20)
```
Type            | Prob | AC | Protection
----------------|------|----|------------
Leather         | 20   | 8  | 
Ring Mail       | 15   | 7  |
Studded Leather | 15   | 7  |
Scale Mail      | 13   | 6  |
Chain Mail      | 12   | 5  |
Splint Mail     | 10   | 4  |
Banded Mail     | 10   | 4  |
Plate Mail      | 5    | 3  |
```

### Rings (14 types, probabilities out of 39)
```
Type            | Prob | Effect
----------------|------|------------------------------------------
Protection      | 9    | +1 AC per plus, +1 hunger
Add Strength    | 9    | +1 strength per plus, +1 hunger
Sustain Str     | 5    | Prevent strength loss
Searching       | 10   | Auto-search for traps/doors
See Invisible   | 10   | See invisible monsters
Adornment       | 1    | Useless (joke)
Aggravate       | 11   | Wake and anger monsters
Dexterity       | 8    | +1 to-hit per plus, +1 hunger
Increase Damage | 8    | +1 damage per plus, +1 hunger
Regeneration    | 4    | Fast healing, +2 hunger
Slow Digestion  | 9    | -1 hunger
Teleportation   | 5    | Random teleports
Stealth         | 7    | Monsters don't wake
Maintain Armor  | 5    | Prevent rust
```

### Sticks (14 types, probabilities out of 41)
```
Type            | Prob | Charges | Effect
----------------|------|---------|----------------------------------
Light           | 12   | 12-22   | Light room
Invisibility    | 6    | 4-8     | Turn invisible
Lightning       | 3    | 4-8     | Lightning bolt damage
Fire            | 3    | 4-8     | Fire bolt damage
Cold            | 3    | 4-8     | Cold bolt damage
Polymorph       | 15   | 5-15    | Change monster type
Magic Missile   | 10   | 5-15    | Force bolt damage
Haste Monster   | 10   | 4-8     | Speed up monster
Slow Monster    | 11   | 5-15    | Slow down monster
Drain Life      | 9    | 4-8     | Damage and reduce level
Nothing         | 1    | 4-8     | Does nothing (joke)
Teleport Away   | 6    | 4-8     | Teleport monster away
Teleport To     | 6    | 4-8     | Teleport monster to player
Cancellation    | 5    | 4-8     | Remove special abilities
```

## Critical Game Formulas

### Experience and Leveling
```
Experience needed for level N:
    Level 1: 10 XP
    Level 2: 20 XP
    Level 3: 40 XP
    Level 4: 80 XP
    ...
    Level N: 10 * 2^(N-1) XP

HP gain per level:
    roll(1, d10) + max(0, (CON-14)/2)
    (Rogue doesn't use CON, just roll 1d10)

Monster XP:
    base_xp = monster.m_stats.s_exp
    level_bonus = (level - 26) * 10  (if level > 26)
    hp_bonus = exp_add(monster)  (based on HP rolls)
    total = base_xp + level_bonus + hp_bonus
```

### Hunger System
```
HUNGERTIME = 1300 turns
MORETIME = 150 turns
STOMACHSIZE = 2000 "food units"

States:
    food_left > 1300: Not hungry
    1300 >= food_left > 300: Hungry (no penalty)
    300 >= food_left > 150: Weak (-1 to combat)
    150 >= food_left > 0: Fainting (no action some turns)
    0: Starving (take damage)

Food restores 1300 food units
```

### Healing
```
Natural healing (when quiet > 0):
    Every 30 turns: heal 1 HP
    
Ring of Regeneration:
    Every 15 turns: heal 1 HP (but increases hunger)
    
Healing potion:
    Heal roll(player_level, 8) HP
    
Extra healing potion:
    Heal to max_hp
    Increase max_hp by 1
```

### Gold Calculation
```
Gold value = random(50 + 10 * dungeon_level) + 2

Example:
    Level 1: 12 to 62 gold
    Level 10: 12 to 152 gold
    Level 26: 12 to 312 gold
```

### Item Enchantment
```
Weapons/Armor:
    Start at +0/+0 (to-hit/damage or AC)
    Enchant weapon scroll: +1/+1
    Enchant armor scroll: +1 AC
    Cursed: negative values
    
Maximum practical enchantment: Around +5 to +10
```

### Trap Probabilities
Number of traps on level:
```
if (random(10) < dungeon_level):
    num_traps = random(dungeon_level / 4) + 1
    num_traps = min(num_traps, MAXTRAPS)
```

### Room Flags
```
Dark room chance: (dungeon_level - 1) / 10
Maze room chance (if dark): 1 / 15
Gone room count: random(4) rooms removed
```

## Status Effect Durations

```
Effect          | Duration (turns) | Source
----------------|------------------|---------------------------
Confusion       | 20              | Potion, monster attack
Hallucination   | 850             | Potion
Blindness       | 850             | Potion, gas trap
See Invisible   | 850             | Potion, ring (permanent)
Haste           | 3-13            | Potion, ring (permanent)
Slow            | 3-13            | Wand
Hold            | 3-13            | Scroll
Levitation      | 3-13            | Potion, ring (permanent)
Paralysis       | 3-13            | Monster attack
Held by trap    | 3-13            | Bear trap
```

## Pathfinding and Movement

### Chase Algorithm (Simplified)
```
1. Calculate distance to player
2. If in same room and can see:
   - Move toward player directly
3. If in different room:
   - Move toward room exit leading to player's room
4. Diagonal movement preferred (faster)
5. Don't move into walls or other monsters
```

### Running Algorithm
```
Player enters run mode with Shift+direction
Continue running until:
    - Monster visible
    - Adjacent to something interesting (door, item)
    - Corridor junction
    - Player presses a key
```

## Display Characters

```
Symbol | Meaning
-------|------------------
@      | Player
A-Z    | Monsters
#      | Corridor
+      | Door
.      | Floor
%      | Stairs down
^      | Trap
*      | Gold
!      | Potion
?      | Scroll
/      | Wand or staff
=      | Ring
)      | Weapon
]      | Armor
:      | Food
,      | Amulet of Yendor
```

## Save File Format

The save file contains (in order):
1. Random seed
2. Dungeon level
3. Max level reached
4. Gold
5. Player stats
6. Player position
7. Player flags
8. Inventory
9. Map state (all cells)
10. All monsters
11. All objects
12. Room data
13. Daemon/fuse states
14. Message buffer

Encrypted with XOR cipher to prevent tampering.

## Implementation Tips

### Critical Systems to Get Right
1. **RNG seeding**: Must be reproducible for same seed
2. **Combat math**: Off-by-one errors break balance
3. **Line of sight**: Affects gameplay significantly
4. **Item identification**: Track known items properly
5. **Save/load**: Test extensively, easy to corrupt

### Common Pitfalls
1. **Coordinate confusion**: (y, x) vs (x, y) - be consistent!
2. **Off-by-one errors**: Especially in grid calculations
3. **Pointer invalidation**: In C, be careful; in Rust, use IDs
4. **Message timing**: Don't overwrite important messages
5. **Status effect stacking**: Handle multiple effects correctly

### Testing Checklist
- [ ] Can reach level 26 and get amulet
- [ ] All item types can be used
- [ ] All monster types spawn correctly
- [ ] Combat math produces reasonable results
- [ ] Save/load preserves game state exactly
- [ ] Hunger system doesn't starve too fast
- [ ] Experience system levels appropriately
- [ ] Pathfinding doesn't get stuck
- [ ] Item identification tracks correctly
- [ ] Score calculation matches original

## Performance Targets

```
Operation               | Target Time
------------------------|-------------
Frame render           | < 16ms (60 FPS)
Monster AI (per turn)  | < 1ms per monster
Pathfinding            | < 1ms per query
Level generation       | < 100ms
Save game             | < 50ms
Load game             | < 100ms
```

## Debugging Tips

### Common Issues
**Problem**: Monster walks through walls
**Solution**: Check move validation logic

**Problem**: Items disappear when dropped
**Solution**: Verify object list management

**Problem**: Combat damage seems wrong
**Solution**: Print all intermediate calculations

**Problem**: Save file corrupts
**Solution**: Add version number and checksums

### Debug Commands (Wizard Mode)
In original Rogue, wizard mode provides:
- C: Create item
- E: Show all items
- F: Show entire level
- T: Teleport
- ?: Debug info

Implement similar for testing!

## Constants Reference

```rust
// Map
pub const MAXROOMS: usize = 9;
pub const MAXLINES: usize = 24;
pub const MAXCOLS: usize = 80;
pub const STATLINE: usize = 23;

// Items
pub const MAXTHINGS: usize = 9;
pub const MAXOBJ: usize = 9;
pub const MAXPACK: usize = 23;
pub const MAXTRAPS: usize = 10;

// Game progression
pub const AMULETLEVEL: usize = 26;
pub const NUMTHINGS: usize = 7;

// Timing
pub const HUNGERTIME: i32 = 1300;
pub const MORETIME: i32 = 150;
pub const STOMACHSIZE: i32 = 2000;
pub const STARVETIME: i32 = 850;
pub const HEALTIME: i32 = 30;

// Vision
pub const LAMPDIST: i32 = 3;
pub const BOLT_LENGTH: i32 = 6;
```

This quick reference should help you implement the core mechanics correctly. Cross-reference with the original C code when in doubt!
