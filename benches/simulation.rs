use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::HashMap;
use ants::colony::{Colony, Direction};
use ants::simulation::Simulation;

fn create_test_map(_num_ants: usize) -> (HashMap<String, Colony>, Vec<String>) {
    let mut colonies = HashMap::new();
    let colony_names = vec!["A".to_string(), "B".to_string()];
    
    // Create just two colonies that are directly connected
    let mut colony_a = Colony::new("A".to_string());
    colony_a.add_tunnel(Direction::North, "B".to_string());
    
    let mut colony_b = Colony::new("B".to_string());
    colony_b.add_tunnel(Direction::South, "A".to_string());
    
    colonies.insert("A".to_string(), colony_a);
    colonies.insert("B".to_string(), colony_b);

    (colonies, colony_names)
}

fn simulation_benchmark(c: &mut Criterion) {
    let ant_counts: [usize; 3] = [3, 6, 9];
    
    for &num_ants in ant_counts.iter() {
        let (colonies, colony_names) = create_test_map(num_ants);
        c.bench_function(&format!("simulation/{}_ants", num_ants), |b| {
            b.iter(|| {
                let mut simulation = Simulation::new(black_box(colonies.clone()), black_box(num_ants))
                    .expect("Failed to create simulation");
                simulation.run().expect("Simulation failed");
            })
        });
    }
}

criterion_group!(benches, simulation_benchmark);
criterion_main!(benches); 