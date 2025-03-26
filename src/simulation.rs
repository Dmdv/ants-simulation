use rand::Rng;
use rand::rngs::SmallRng;
use rand::SeedableRng;
use rand::rng;
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

#[derive(Clone)]
struct Ant {
    moves: u32,
    location: Option<usize>,
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
    /// Vector of ants with their state
    ants: Vec<Ant>,
    /// Vector tracking destroyed colonies (better cache locality than HashSet)
    destroyed_colonies: Vec<bool>,
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

        let seed = rng().random();
        let mut rng = SmallRng::seed_from_u64(seed);
        let mut ants = Vec::with_capacity(num_ants);
        let destroyed_colonies = vec![false; colonies.len()];

        // Place ants randomly in colonies
        for _ in 0..num_ants {
            let colony_idx = rng.random_range(0..colonies.len());
            colonies[colony_idx].set_ant(Some(ants.len()));
            ants.push(Ant {
                moves: 0,
                location: Some(colony_idx),
            });
        }

        Ok(Self {
            colonies,
            ants,
            destroyed_colonies,
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
        let mut moves_to_make = Vec::with_capacity(self.ants.len());
        let mut colonies_to_destroy = Vec::new();
        let mut ants_to_kill = Vec::new();

        // First pass: collect moves and fights
        for ant_id in 0..self.ants.len() {
            if let Some(colony_idx) = self.ants[ant_id].location {
                if let Some(direction) = self.colonies[colony_idx].get_random_direction() {
                    if let Some(target_idx) = self.colonies[colony_idx].get_target_colony(&direction) {
                        if !self.destroyed_colonies[target_idx] {
                            let target_colony = &self.colonies[target_idx];
                            if target_colony.get_ant().is_none() {
                                moves_to_make.push((ant_id, colony_idx, target_idx));
                            } else {
                                // Fight detected
                                colonies_to_destroy.push(target_idx);
                                ants_to_kill.push(ant_id);
                                ants_to_kill.push(target_colony.get_ant().unwrap());
                                
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

        // Second pass: process fights
        for colony_idx in colonies_to_destroy {
            self.destroyed_colonies[colony_idx] = true;
            self.colonies[colony_idx].is_destroyed = true;
            
            // Remove tunnels to destroyed colony
            for colony in &mut self.colonies {
                colony.remove_tunnel_to(colony_idx);
            }
        }

        // Third pass: kill ants
        for ant_id in ants_to_kill {
            self.ants[ant_id].location = None;
        }

        // Fourth pass: process moves
        for (ant_id, from_idx, to_idx) in moves_to_make {
            if !self.destroyed_colonies[to_idx] && self.ants[ant_id].location.is_some() {
                // Move ant to new colony
                self.colonies[from_idx].set_ant(None);
                self.colonies[to_idx].set_ant(Some(ant_id));
                self.ants[ant_id].location = Some(to_idx);
                self.ants[ant_id].moves += 1;
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
        for ant in &self.ants {
            if ant.moves < self.max_moves && ant.location.is_some() {
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
        for colony in &self.colonies {
            if !colony.is_destroyed {
                print!("{}", colony.name);
                for direction in [Direction::North, Direction::South, Direction::East, Direction::West] {
                    if let Some(target_idx) = colony.get_target_colony(&direction) {
                        print!(" {}={}", 
                            match direction {
                                Direction::North => "north",
                                Direction::South => "south",
                                Direction::East => "east",
                                Direction::West => "west",
                            },
                            self.colonies[target_idx].name
                        );
                    }
                }
                println!();
            }
        }
    }
} 