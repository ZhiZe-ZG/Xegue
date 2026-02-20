/// Number of rooms in the classic Rogue 3x3 grid.
pub const MAX_ROOMS: usize = 9;

/// Possible room adjacency matrix (fixed topology).
///
/// This is a direct translation of the C `conn` arrays in `struct rdes`.
/// `POSSIBLE_ADJACENT[i][j] == true` means room `i` may connect to room `j`.
pub const POSSIBLE_ADJACENT: [[bool; MAX_ROOMS]; MAX_ROOMS] = [
    // room 0
    [false, true,  false, true,  false, false, false, false, false],
    // room 1
    [true,  false, true,  false, true,  false, false, false, false],
    // room 2
    [false, true,  false, false, false, true,  false, false, false],
    // room 3
    [true,  false, false, false, true,  false, true,  false, false],
    // room 4
    [false, true,  false, true,  false, true,  false, true,  false],
    // room 5
    [false, false, true,  false, true,  false, false, false, true ],
    // room 6
    [false, false, false, true,  false, false, false, true,  false],
    // room 7
    [false, false, false, false, true,  false, true,  false, true ],
    // room 8
    [false, false, false, false, false, true,  false, true,  false],
];

/// Actual adjacency matrix: connections that have been realized
/// on the current level.
///
/// You can rebuild this from scratch when generating a level
/// to mirror the `isconn` arrays in the C code.
#[derive(Clone, Debug)]
pub struct RoomAdjacency {
    pub connected: [[bool; MAX_ROOMS]; MAX_ROOMS],
}

impl RoomAdjacency {
    /// Create a new adjacency matrix with no realized connections.
    pub fn new() -> Self {
        Self {
            connected: [[false; MAX_ROOMS]; MAX_ROOMS],
        }
    }

    /// Mark a realized connection between rooms `a` and `b`.
    pub fn connect(&mut self, a: usize, b: usize) {
        if a >= MAX_ROOMS || b >= MAX_ROOMS {
            return;
        }
        self.connected[a][b] = true;
        self.connected[b][a] = true;
    }

    /// Check if rooms `a` and `b` are allowed to be adjacent
    /// according to the fixed topology.
    pub fn is_possible(&self, a: usize, b: usize) -> bool {
        a < MAX_ROOMS && b < MAX_ROOMS && POSSIBLE_ADJACENT[a][b]
    }

    /// Check if rooms `a` and `b` are actually connected
    /// on the current level.
    pub fn is_connected(&self, a: usize, b: usize) -> bool {
        a < MAX_ROOMS && b < MAX_ROOMS && self.connected[a][b]
    }

    /// Generate a random room connection graph.
    ///
    /// This mirrors Rogue's `do_passages`:
    /// - First builds a random spanning tree using `POSSIBLE_ADJACENT`.
    /// - Then adds a small number of extra random edges.
    pub fn generate_random_graph<R: rand::Rng + ?Sized>(rng: &mut R) -> RoomAdjacency {
        let mut adj = RoomAdjacency::new();

        // Track which rooms are already in the graph (ingraph in C).
        let mut in_graph = [false; MAX_ROOMS];

        // Pick a random starting room.
        let mut current = rng.random_range(0..MAX_ROOMS);
        in_graph[current] = true;
        let mut room_count = 1;

        // Build a spanning tree over all rooms.
        while room_count < MAX_ROOMS {
            // Collect neighbors that are possible and not yet in the graph.
            let mut candidates = Vec::new();
            for i in 0..MAX_ROOMS {
                if POSSIBLE_ADJACENT[current][i] && !in_graph[i] {
                    candidates.push(i);
                }
            }

            if candidates.is_empty() {
                // No adjacent rooms outside the graph; pick a new room already in the graph.
                loop {
                    let r = rng.random_range(0..MAX_ROOMS);
                    if in_graph[r] {
                        current = r;
                        break;
                    }
                }
            } else {
                // Connect to a random neighbor and mark it in the graph.
                let idx = rng.random_range(0..candidates.len());
                let next = candidates[idx];
                adj.connect(current, next);
                in_graph[next] = true;
                current = next;
                room_count += 1;
            }
        }

        // Add a few extra random connections (like Rogue's extra passages).
        let extra_edges = rng.random_range(0..5); // 0..=4 extra edges
        for _ in 0..extra_edges {
            let from = rng.random_range(0..MAX_ROOMS);

            // Collect neighbors this room could connect to but isn't yet.
            let mut candidates = Vec::new();
            for to in 0..MAX_ROOMS {
                if POSSIBLE_ADJACENT[from][to] && !adj.is_connected(from, to) {
                    candidates.push(to);
                }
            }

            if candidates.is_empty() {
                continue;
            }

            let idx = rng.random_range(0..candidates.len());
            let to = candidates[idx];
            adj.connect(from, to);
        }

        adj
    }
}
