use std::collections::{HashMap, HashSet};
use rand::seq::SliceRandom;
use crate::colony::{Colony, Direction};

/// Maximum number of moves allowed per ant
const MAX_MOVES: u32 = 10_000;
/// Maximum number of steps allowed in the simulation
const MAX_STEPS: u32 = 100_000;

/// Error type for simulation errors
#[derive(Debug)]
pub enum SimulationError {
    NoColonies,
    NoAnts,
    InvalidColony(String),
}

/// A simulation of ants moving between colonies, fighting when they meet.
/// 
/// The simulation follows these rules:
/// - Ants start at random colonies
/// - Each step, ants can move to any connected colony that has no other ants
/// - When two ants meet in a colony, they fight and destroy it
/// - Destroyed colonies are removed from the map and can't be traveled to
/// - Simulation ends when all ants are destroyed or max moves reached
pub struct Simulation {
    /// Map of colony names to their data
    colonies: HashMap<String, Colony>,
    /// Current position of each ant
    ant_positions: HashMap<usize, String>,
    /// Number of moves each ant has made
    ant_moves: HashMap<usize, u32>,
    /// Number of ants currently in each colony
    colony_counts: HashMap<String, usize>,
    /// Set of destroyed colony names for fast lookup
    destroyed_colonies: HashSet<String>,
    /// Maximum number of moves allowed per ant
    max_moves: u32,
    /// Current step count of the simulation
    step_count: u32,
    /// Maximum number of steps allowed
    max_steps: u32,
}

impl Simulation {
    /// Creates a new simulation with the given colonies and number of ants.
    /// 
    /// # Arguments
    /// * `colonies` - Map of colony names to their data
    /// * `num_ants` - Number of ants to create
    /// 
    /// # Returns
    /// * `Result<Self, SimulationError>` - The new simulation or an error
    /// 
    /// # Errors
    /// * `SimulationError::NoColonies` - If no colonies are provided
    /// * `SimulationError::NoAnts` - If num_ants is 0
    pub fn new(colonies: HashMap<String, Colony>, num_ants: usize) -> Result<Self, SimulationError> {
        if colonies.is_empty() {
            return Err(SimulationError::NoColonies);
        }
        if num_ants == 0 {
            return Err(SimulationError::NoAnts);
        }

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

        Ok(Self {
            colonies,
            ant_positions,
            ant_moves,
            colony_counts,
            destroyed_colonies: HashSet::new(),
            max_moves: MAX_MOVES,
            step_count: 0,
            max_steps: MAX_STEPS,
        })
    }

    /// Runs the simulation until completion.
    /// 
    /// The simulation ends when:
    /// - All ants are destroyed
    /// - Each ant has moved max_moves times
    /// - The simulation reaches max_steps
    /// 
    /// # Errors
    /// * `SimulationError` - If any step fails
    pub fn run(&mut self) -> Result<(), SimulationError> {
        while self.are_ants_active() {
            self.step()?;
            self.step_count += 1;
            if self.step_count >= self.max_steps {
                println!("Simulation stopped after {} steps", self.max_steps);
                break;
            }
        }
        Ok(())
    }

    /// Performs a single step of the simulation.
    /// 
    /// In each step:
    /// 1. Ants attempt to move to random available colonies
    /// 2. If two ants meet, they fight and destroy the colony
    /// 3. Destroyed colonies are removed from the map
    /// 4. Ant positions and colony counts are updated
    /// 
    /// # Errors
    /// * `SimulationError::InvalidColony` - If an ant is in a non-existent colony
    fn step(&mut self) -> Result<(), SimulationError> {
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
            } else {
                return Err(SimulationError::InvalidColony(current_colony.clone()));
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

        Ok(())
    }

    /// Checks if any ants are still active in the simulation.
    /// 
    /// An ant is considered active if:
    /// - It hasn't been destroyed in a fight
    /// - It hasn't reached max_moves
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

    /// Prints the final state of the simulation in the required format.
    /// 
    /// Format: "colony_name direction=target ..."
    /// Only prints non-destroyed colonies.
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