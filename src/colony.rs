use rand::prelude::*;
use rand::rng;

#[derive(Debug, Clone)]
pub struct Colony {
    tunnels: [Option<usize>; 4],
    available_directions: u8, // Bit field tracking available directions
    ant_id: Option<usize>,    // The ant currently in this colony, if any
    is_destroyed: bool,
    pub name: String,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Copy)]
pub enum Direction {
    North = 0,
    South = 1,
    East = 2,
    West = 3,
}

const ALL_DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::South,
    Direction::East,
    Direction::West,
];
const DIRECTION_MASKS: [u8; 4] = [0b0001, 0b0010, 0b0100, 0b1000];

impl Colony {
    pub fn new(name: String) -> Self {
        Self {
            tunnels: [None; 4],
            available_directions: 0,
            ant_id: None,
            is_destroyed: false,
            name,
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

        // Find the nth available direction using bit manipulation
        let mut remaining = self.available_directions;
        let mut current = 0;
        for i in 0..4 {
            let mask = DIRECTION_MASKS[i];
            if (remaining & mask) != 0 {
                if current == idx {
                    return Some(ALL_DIRECTIONS[i]);
                }
                remaining &= !mask;
                current += 1;
            }
        }
        None
    }

    #[inline]
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

    #[inline]
    pub fn set_ant(&mut self, ant_id: Option<usize>) {
        self.ant_id = ant_id;
    }

    #[inline]
    pub fn get_ant(&self) -> Option<usize> {
        self.ant_id
    }

    #[inline]
    pub fn is_destroyed(&self) -> bool {
        self.is_destroyed
    }

    #[inline]
    pub fn set_destroyed(&mut self, destroyed: bool) {
        self.is_destroyed = destroyed;
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
