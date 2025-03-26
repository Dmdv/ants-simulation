use rand::prelude::*;
use rand::rng;

#[derive(Debug, Clone)]
pub struct Colony {
    pub name: String,  // Kept for display purposes only
    tunnels: [Option<usize>; 4],  // Fixed-size array for tunnels, indexed by Direction
    pub is_destroyed: bool,
    pub ant_id: Option<usize>,  // The ant currently in this colony, if any
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Copy)]
pub enum Direction {
    North = 0,
    South = 1,
    East = 2,
    West = 3,
}

const ALL_DIRECTIONS: [Direction; 4] = [Direction::North, Direction::South, Direction::East, Direction::West];

impl Colony {
    pub fn new(name: String) -> Self {
        Self {
            name,
            tunnels: [None; 4],
            is_destroyed: false,
            ant_id: None,
        }
    }

    pub fn add_tunnel(&mut self, direction: Direction, target: usize) {
        self.tunnels[direction as usize] = Some(target);
    }

    pub fn get_random_direction(&self) -> Option<Direction> {
        // Use a static array to avoid allocation
        let mut available_count = 0;
        let mut available = [Direction::North; 4];
        
        for (i, &tunnel) in self.tunnels.iter().enumerate() {
            if tunnel.is_some() {
                available[available_count] = ALL_DIRECTIONS[i];
                available_count += 1;
            }
        }
        
        if available_count == 0 {
            return None;
        }
        
        // Use slice to avoid copying the whole array
        Some(*(&available[..available_count]).choose(&mut rng()).unwrap())
    }

    pub fn get_target_colony(&self, direction: &Direction) -> Option<usize> {
        self.tunnels[*direction as usize]
    }

    pub fn remove_tunnel_to(&mut self, target: usize) {
        for tunnel in &mut self.tunnels {
            if *tunnel == Some(target) {
                *tunnel = None;
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