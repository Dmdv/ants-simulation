use std::collections::HashMap;
use rand::seq::SliceRandom;

#[derive(Debug, Clone)]
pub struct Colony {
    pub name: String,
    pub tunnels: HashMap<Direction, String>,
    pub is_destroyed: bool,
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
        }
    }

    pub fn add_tunnel(&mut self, direction: Direction, target: String) {
        self.tunnels.insert(direction, target);
    }

    pub fn get_random_direction(&self) -> Option<Direction> {
        if self.tunnels.is_empty() {
            return None;
        }
        
        let directions: Vec<Direction> = self.tunnels.keys().cloned().collect();
        Some(directions.choose(&mut rand::thread_rng()).unwrap().clone())
    }

    pub fn get_target_colony(&self, direction: &Direction) -> Option<&String> {
        self.tunnels.get(direction)
    }

    pub fn remove_tunnel_to(&mut self, target: &str) {
        self.tunnels.retain(|_, t| t != target);
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