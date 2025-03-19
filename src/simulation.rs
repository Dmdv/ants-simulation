use std::collections::{HashMap, HashSet};
use rand::seq::SliceRandom;
use crate::colony::{Colony, Direction};

pub struct Simulation {
    colonies: HashMap<String, Colony>,
    ant_positions: HashMap<usize, String>,
    ant_moves: HashMap<usize, u32>,
    colony_counts: HashMap<String, usize>,
    max_moves: u32,
    step_count: u32,
    max_steps: u32,
}

impl Simulation {
    pub fn new(colonies: HashMap<String, Colony>, num_ants: usize) -> Self {
        let mut ant_positions = HashMap::new();
        let mut ant_moves = HashMap::new();
        let mut colony_counts = HashMap::new();
        let colony_names: Vec<String> = colonies.keys().cloned().collect();

        for ant_id in 0..num_ants {
            let random_colony = colony_names.choose(&mut rand::thread_rng()).unwrap().clone();
            ant_positions.insert(ant_id, random_colony.clone());
            *colony_counts.entry(random_colony).or_insert(0) += 1;
            ant_moves.insert(ant_id, 0);
        }

        Self {
            colonies,
            ant_positions,
            ant_moves,
            colony_counts,
            max_moves: 10_000,
            step_count: 0,
            max_steps: 100_000,
        }
    }

    pub fn run(&mut self) {
        while self.are_ants_active() {
            self.step();
            self.step_count += 1;
            if self.step_count >= self.max_steps {
                println!("Simulation stopped after {} steps", self.max_steps);
                break;
            }
        }
    }

    fn step(&mut self) {
        let mut new_positions = HashMap::new();
        let mut colonies_to_destroy = HashSet::new();

        // Move all ants
        for (ant_id, current_colony) in &self.ant_positions {
            if let Some(colony) = self.colonies.get(current_colony) {
                // Get all possible directions and their target colonies
                let targets: Vec<(&Direction, &String)> = colony.tunnels.iter().collect();

                if !targets.is_empty() {
                    // Filter out colonies that already have an ant
                    let available_targets: Vec<_> = targets.iter()
                        .filter(|(_, target)| self.colony_counts.get(*target).unwrap_or(&0) == &0)
                        .collect();

                    if !available_targets.is_empty() {
                        // Randomly choose from available targets
                        let target = available_targets.choose(&mut rand::thread_rng()).unwrap().1;
                        new_positions.insert(*ant_id, target.clone());
                        *self.ant_moves.get_mut(ant_id).unwrap() += 1;
                    }
                }
            }
        }

        // Check for fights
        let mut positions: HashMap<String, Vec<usize>> = HashMap::new();
        for (ant_id, colony) in &new_positions {
            positions.entry(colony.clone()).or_default().push(*ant_id);
        }

        // Process fights
        for (colony_name, ants) in positions {
            if ants.len() >= 2 {
                println!("{} has been destroyed by ant {} and ant {}!", 
                    colony_name, ants[0], ants[1]);
                colonies_to_destroy.insert(colony_name);
                
                // Remove all ants that were in this fight
                for &ant_id in &ants {
                    self.ant_positions.remove(&ant_id);
                }
            }
        }

        // Update positions and remove destroyed colonies
        let destroyed_colonies: HashSet<String> = colonies_to_destroy.iter().cloned().collect();
        for colony_name in &destroyed_colonies {
            if let Some(colony) = self.colonies.get_mut(colony_name) {
                colony.is_destroyed = true;
                // Remove tunnels to this colony from other colonies
                for other_colony in self.colonies.values_mut() {
                    other_colony.remove_tunnel_to(colony_name);
                }
            }
        }

        // Update ant positions and colony counts
        for (ant_id, new_colony) in new_positions {
            if !destroyed_colonies.contains(&new_colony) {
                if let Some(old_colony) = self.ant_positions.get(&ant_id) {
                    *self.colony_counts.get_mut(old_colony).unwrap() -= 1;
                }
                *self.colony_counts.entry(new_colony.clone()).or_insert(0) += 1;
                self.ant_positions.insert(ant_id, new_colony);
            }
        }
    }

    fn are_ants_active(&self) -> bool {
        if self.ant_positions.is_empty() {
            return false; // All ants have been destroyed
        }

        // Check if any ants haven't reached max moves yet
        for (ant_id, moves) in &self.ant_moves {
            if *moves < self.max_moves && self.ant_positions.contains_key(ant_id) {
                return true;
            }
        }
        false
    }

    pub fn print_final_state(&self) {
        for (name, colony) in &self.colonies {
            if !colony.is_destroyed {
                print!("{}", name);
                for (direction, target) in &colony.tunnels {
                    print!(" {}={}", 
                        match direction {
                            Direction::North => "north",
                            Direction::South => "south",
                            Direction::East => "east",
                            Direction::West => "west",
                        },
                        target
                    );
                }
                println!();
            }
        }
    }
} 