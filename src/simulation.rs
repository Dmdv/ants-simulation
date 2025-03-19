use std::collections::{HashMap, HashSet};
use rand::seq::SliceRandom;
use crate::colony::{Colony, Direction};

pub struct Simulation {
    colonies: HashMap<String, Colony>,
    ant_positions: HashMap<usize, String>,
    ant_moves: HashMap<usize, u32>,
    colony_counts: HashMap<String, usize>,
    destroyed_colonies: HashSet<String>,  // Cache destroyed status
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
            destroyed_colonies: HashSet::new(),
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
        let mut colonies_to_destroy = HashSet::new();
        let mut ants_to_kill = HashSet::new();
        let mut moves_to_make = Vec::new();
        let mut ants_to_remove = Vec::new();

        // Single pass: collect moves and process fights
        for (ant_id, current_colony) in &self.ant_positions {
            if let Some(colony) = self.colonies.get(current_colony) {
                let targets: Vec<(&Direction, &String)> = colony.tunnels.iter().collect();

                if !targets.is_empty() {
                    let available_targets: Vec<_> = targets.iter()
                        .filter(|(_, target)| {
                            // Single HashMap lookup for count and check destroyed status from cache
                            self.colony_counts.get(*target).unwrap_or(&0) == &0 &&
                            !self.destroyed_colonies.contains(*target)
                        })
                        .collect();

                    if !available_targets.is_empty() {
                        let target = available_targets.choose(&mut rand::thread_rng()).unwrap().1;
                        
                        // Check if this colony already has an ant
                        if let Some(existing_ant) = moves_to_make.iter().find(|(_, t)| t == target) {
                            // Fight detected
                            colonies_to_destroy.insert(target.clone());
                            ants_to_kill.insert(existing_ant.0);
                            ants_to_kill.insert(*ant_id);
                            
                            // Update colony counts for killed ants
                            if let Some(old_colony) = self.ant_positions.get(&existing_ant.0) {
                                *self.colony_counts.get_mut(old_colony).unwrap() -= 1;
                            }
                            if let Some(old_colony) = self.ant_positions.get(ant_id) {
                                *self.colony_counts.get_mut(old_colony).unwrap() -= 1;
                            }
                            
                            // Collect ants to remove
                            ants_to_remove.push(existing_ant.0);
                            ants_to_remove.push(*ant_id);
                            
                            println!("{} has been destroyed by ant {} and ant {}!", 
                                target, existing_ant.0, ant_id);
                        } else {
                            moves_to_make.push((*ant_id, target.clone()));
                        }
                    }
                }
            }
        }

        // Remove killed ants
        for ant_id in ants_to_remove {
            self.ant_positions.remove(&ant_id);
        }

        // Update colonies and ant positions in a single pass
        for (ant_id, new_colony) in moves_to_make {
            if !colonies_to_destroy.contains(&new_colony) && !ants_to_kill.contains(&ant_id) {
                if let Some(old_colony) = self.ant_positions.get(&ant_id) {
                    *self.colony_counts.get_mut(old_colony).unwrap() -= 1;
                }
                *self.colony_counts.entry(new_colony.clone()).or_insert(0) += 1;
                self.ant_positions.insert(ant_id, new_colony);
                *self.ant_moves.get_mut(&ant_id).unwrap() += 1;
            }
        }

        // Update destroyed colonies
        for colony_name in &colonies_to_destroy {
            if let Some(colony) = self.colonies.get_mut(colony_name) {
                colony.is_destroyed = true;
                self.destroyed_colonies.insert(colony_name.clone());
                for other_colony in self.colonies.values_mut() {
                    other_colony.remove_tunnel_to(colony_name);
                }
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