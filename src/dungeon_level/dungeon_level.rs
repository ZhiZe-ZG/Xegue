use crate::terrain::terrain_grid::TerrainGrid;
use crate::terrain_structure::room::Room;
use crate::terrain_structure::passage::Passage;
use crate::dungeon_level::terrain_generator::TerrainGenerator;

pub struct DungeonLevel {
    pub terrain: TerrainGrid,
    pub rooms: Vec<Room>,
    pub passages: Vec<Passage>,
}

// Create a DungeonLevel, generate rooms, place them on the terrain,
// and carve doors & passages based on the generated adjacency graph.
pub fn generate_dungeon_level() -> DungeonLevel {
    let generator = TerrainGenerator::default();

    let mut terrain = TerrainGrid::init(
        generator.terrain_size.x as usize,
        generator.terrain_size.y as usize,
    );

    // Rooms must be mutable because carving will push door positions into them.
    let mut rooms = generator.generate_rooms();

    // First, put rooms on the terrain.
    generator.put_rooms_on_grid(&mut terrain, rooms.clone());

    // Then generate a random connection graph and carve passages.
    let adjacency = generator.generate_room_connections();
    let passages = generator.carve_passages(&mut terrain, &mut rooms, &adjacency);

    DungeonLevel {
        terrain,
        rooms,
        passages,
    }
}

