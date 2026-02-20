use crate::terrain::terrain_grid::TerrainGrid;
use crate::terrain_structure::room::Room;
use crate::dungeon_level::terrain_generator::TerrainGenerator;

pub struct DungeonLevel {
    pub terrain: TerrainGrid,
    pub rooms: Vec<Room>,
}

    // Create a DungeonLevel, generate rooms, and place them on the terrain.
    pub fn generate_dungeon_level() -> DungeonLevel {
        let generator = TerrainGenerator::default();

        // Adjust this if TerrainGrid has a different constructor or size API.
        let mut terrain = TerrainGrid::init(
            generator.terrain_size.x as usize,
            generator.terrain_size.y as usize,
        );

        let rooms = generator.generate_rooms();
        generator.put_rooms_on_grid(&mut terrain, rooms.clone());

        DungeonLevel { terrain, rooms }
    }

