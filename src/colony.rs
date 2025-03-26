use std::collections::HashMap;
use rand::seq::SliceRandom;

#[derive(Debug, Clone)]
pub struct Colony {
    pub name: String,  // Kept for display purposes only
    pub tunnels: HashMap<Direction, usize>,  // Maps direction to colony index
    pub is_destroyed: bool,
    pub ant_id: Option<usize>,  // The ant currently in this colony, if any
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Colony {
    pub fn new(name: String) -> Self {
        Self {
            name,
            tunnels: HashMap::new(),
            is_destroyed: false,
            ant_id: None,
        }
    }

    pub fn add_tunnel(&mut self, direction: Direction, target: usize) {
        self.tunnels.insert(direction, target);
    }

    pub fn get_random_direction(&self) -> Option<Direction> {
        if self.tunnels.is_empty() {
            return None;
        }
        
        let directions: Vec<Direction> = self.tunnels.keys().cloned().collect();
        Some(directions.choose(&mut rand::thread_rng()).unwrap().clone())
    }

    pub fn get_target_colony(&self, direction: &Direction) -> Option<usize> {
        self.tunnels.get(direction).copied()
    }

    pub fn remove_tunnel_to(&mut self, target: usize) {
        self.tunnels.retain(|_, t| *t != target);
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