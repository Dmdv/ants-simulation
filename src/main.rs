use clap::Parser;
use std::time::Instant;

pub mod colony;
pub mod simulation;
pub mod parser;

use simulation::Simulation;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of ants to create
    #[arg(short, long)]
    ants: usize,

    /// Path to the map file
    #[arg(short, long)]
    map: String,
}

fn main() {
    let args = Args::parse();
    
    // Read and parse the map file
    let colonies = parser::parse_map_file(&args.map).expect("Failed to parse map file");
    
    // Create and run simulation
    let mut simulation = Simulation::new(colonies, args.ants);
    
    // Start timing after map is loaded
    let start_time = Instant::now();
    
    // Run the simulation
    simulation.run();
    
    // Calculate and print execution time
    let duration = start_time.elapsed();
    println!("\nSimulation completed in {:?}", duration);
    
    // Print final state
    simulation.print_final_state();
}
