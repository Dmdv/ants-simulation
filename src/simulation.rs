use std::collections::HashSet;
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
    InvalidColony(usize),
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
    /// Vector of colonies, indexed by their position
    colonies: Vec<Colony>,
    /// Number of moves each ant has made
    ant_moves: Vec<u32>,
    /// Set of destroyed colony indices for fast lookup
    destroyed_colonies: HashSet<usize>,
    /// Maximum number of moves allowed per ant
    max_moves: u32,
    /// Current step count of the simulation
    step_count: u32,
    /// Maximum number of steps allowed
    max_steps: u32,
    /// Whether to print debug output
    debug: bool,
}

impl Simulation {
    /// Creates a new simulation with the given colonies and number of ants.
    /// 
    /// # Arguments
    /// * `colonies` - Vector of colonies
    /// * `num_ants` - Number of ants to create
    /// 
    /// # Returns
    /// * `Result<Self, SimulationError>` - The new simulation or an error
    /// 
    /// # Errors
    /// * `SimulationError::NoColonies` - If no colonies are provided
    /// * `SimulationError::NoAnts` - If num_ants is 0
    pub fn new(mut colonies: Vec<Colony>, num_ants: usize) -> Result<Self, SimulationError> {
        if colonies.is_empty() {
            return Err(SimulationError::NoColonies);
        }
        if num_ants == 0 {
            return Err(SimulationError::NoAnts);
        }

        let ant_moves = vec![0; num_ants];
        let mut rng = rand::thread_rng();

        // Place ants randomly in colonies
        for ant_id in 0..num_ants {
            let colony_indices: Vec<usize> = (0..colonies.len()).collect();
            let colony_idx = *colony_indices.choose(&mut rng).unwrap();
            colonies[colony_idx].set_ant(Some(ant_id));
        }

        Ok(Self {
            colonies,
            ant_moves,
            destroyed_colonies: HashSet::new(),
            max_moves: MAX_MOVES,
            step_count: 0,
            max_steps: MAX_STEPS,
            debug: true,
        })
    }

    /// Creates a new simulation with debug output disabled (for benchmarks).
    pub fn new_silent(colonies: Vec<Colony>, num_ants: usize) -> Result<Self, SimulationError> {
        let mut sim = Self::new(colonies, num_ants)?;
        sim.debug = false;
        Ok(sim)
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
                if self.debug {
                    println!("Simulation stopped after {} steps", self.max_steps);
                }
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

        // Single pass: collect moves and process fights
        for (colony_idx, colony) in self.colonies.iter().enumerate() {
            if let Some(ant_id) = colony.get_ant() {
                if let Some(direction) = colony.get_random_direction() {
                    if let Some(target_idx) = colony.get_target_colony(&direction) {
                        if !self.destroyed_colonies.contains(&target_idx) {
                            let target_colony = &self.colonies[target_idx];
                            if target_colony.get_ant().is_none() {
                                moves_to_make.push((ant_id, colony_idx, target_idx));
                            } else {
                                // Fight detected
                                colonies_to_destroy.insert(target_idx);
                                ants_to_kill.insert(ant_id);
                                ants_to_kill.insert(target_colony.get_ant().unwrap());
                                
                                if self.debug {
                                    println!("{} has been destroyed by ant {} and ant {}!", 
                                        target_colony.name, ant_id, target_colony.get_ant().unwrap());
                                }
                            }
                        }
                    }
                }
            }
        }

        // Process moves and fights
        for (ant_id, from_idx, to_idx) in moves_to_make {
            if !colonies_to_destroy.contains(&to_idx) && !ants_to_kill.contains(&ant_id) {
                // Move ant to new colony
                self.colonies[from_idx].set_ant(None);
                self.colonies[to_idx].set_ant(Some(ant_id));
                self.ant_moves[ant_id] += 1;
            }
        }

        // Remove killed ants
        for ant_id in ants_to_kill {
            for colony in &mut self.colonies {
                if colony.get_ant() == Some(ant_id) {
                    colony.set_ant(None);
                    break;
                }
            }
        }

        // Update destroyed colonies
        for colony_idx in &colonies_to_destroy {
            self.colonies[*colony_idx].is_destroyed = true;
            self.destroyed_colonies.insert(*colony_idx);
            
            // Remove tunnels to destroyed colony
            for colony in &mut self.colonies {
                colony.remove_tunnel_to(*colony_idx);
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
        // Check if any ants haven't reached max moves yet
        for (ant_id, moves) in self.ant_moves.iter().enumerate() {
            if *moves < self.max_moves {
                // Check if this ant is still alive
                for colony in &self.colonies {
                    if colony.get_ant() == Some(ant_id) {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Prints the final state of the simulation in the required format.
    /// 
    /// Format: "colony_name direction=target ..."
    /// Only prints non-destroyed colonies.
    pub fn print_final_state(&self) {
        for colony in &self.colonies {
            if !colony.is_destroyed {
                print!("{}", colony.name);
                for (direction, target_idx) in &colony.tunnels {
                    print!(" {}={}", 
                        match direction {
                            Direction::North => "north",
                            Direction::South => "south",
                            Direction::East => "east",
                            Direction::West => "west",
                        },
                        self.colonies[*target_idx].name
                    );
                }
                println!();
            }
        }
    }
} 