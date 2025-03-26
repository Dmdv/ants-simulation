use rand::prelude::*;
use rand::rng;

#[derive(Debug, Clone)]
pub struct Colony {
    pub name: String,  // Kept for display purposes only
    tunnels: [Option<usize>; 4],  // Fixed-size array for tunnels, indexed by Direction
    pub is_destroyed: bool,
    pub ant_id: Option<usize>,  // The ant currently in this colony, if any
    available_directions: u8,  // Bit field tracking available directions
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Copy)]
pub enum Direction {
    North = 0,
    South = 1,
    East = 2,
    West = 3,
}

const ALL_DIRECTIONS: [Direction; 4] = [Direction::North, Direction::South, Direction::East, Direction::West];
const DIRECTION_MASKS: [u8; 4] = [0b0001, 0b0010, 0b0100, 0b1000];

impl Colony {
    pub fn new(name: String) -> Self {
        Self {
            name,
            tunnels: [None; 4],
            is_destroyed: false,
            ant_id: None,
            available_directions: 0,
        }
    }

    pub fn add_tunnel(&mut self, direction: Direction, target: usize) {
        self.tunnels[direction as usize] = Some(target);
        self.available_directions |= DIRECTION_MASKS[direction as usize];
    }

    pub fn get_random_direction(&self) -> Option<Direction> {
        if self.available_directions == 0 {
            return None;
        }

        // Count available directions
        let count = self.available_directions.count_ones() as usize;
        if count == 0 {
            return None;
        }

        // Generate random index
        let mut rng = rng();
        let idx = rng.random_range(0..count);

        // Find the nth available direction
        let mut current = 0;
        for (i, &mask) in DIRECTION_MASKS.iter().enumerate() {
            if (self.available_directions & mask) != 0 {
                if current == idx {
                    return Some(ALL_DIRECTIONS[i]);
                }
                current += 1;
            }
        }
        None
    }

    pub fn get_target_colony(&self, direction: &Direction) -> Option<usize> {
        self.tunnels[*direction as usize]
    }

    pub fn remove_tunnel_to(&mut self, target: usize) {
        for (i, tunnel) in self.tunnels.iter_mut().enumerate() {
            if *tunnel == Some(target) {
                *tunnel = None;
                self.available_directions &= !DIRECTION_MASKS[i];
                break;
            }
        }
    }

    pub fn set_ant(&mut self, ant_id: Option<usize>) {
        self.ant_id = ant_id;
    }

    pub fn get_ant(&self) -> Option<usize> {
        self.ant_id
    }
}

impl Direction {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "north" => Some(Direction::North),
            "south" => Some(Direction::South),
            "east" => Some(Direction::East),
            "west" => Some(Direction::West),
            _ => None,
        }
    }
} 