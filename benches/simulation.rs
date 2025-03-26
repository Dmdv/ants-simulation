use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pprof::criterion::{Output, PProfProfiler};
use std::time::Duration;
use ants::colony::{Colony, Direction};
use ants::simulation::Simulation;

fn create_test_map(_num_ants: usize) -> Vec<Colony> {
    let mut colonies = Vec::new();
    
    // Create just two colonies that are directly connected
    let mut colony_a = Colony::new("A".to_string());
    colony_a.add_tunnel(Direction::North, 1); // Index of colony B
    colonies.push(colony_a);
    
    let mut colony_b = Colony::new("B".to_string());
    colony_b.add_tunnel(Direction::South, 0); // Index of colony A
    colonies.push(colony_b);

    colonies
}

fn simulation_benchmark(c: &mut Criterion) {
    let ant_counts: [usize; 3] = [3, 6, 9];
    
    for &num_ants in ant_counts.iter() {
        let colonies = create_test_map(num_ants);
        c.bench_function(&format!("simulation/{}_ants", num_ants), |b| {
            b.iter(|| {
                let mut simulation = Simulation::new_silent(black_box(colonies.clone()), black_box(num_ants))
                    .expect("Failed to create simulation");
                simulation.run().expect("Simulation failed");
            })
        });
    }
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)))
        .measurement_time(Duration::from_secs(5))
        .warm_up_time(Duration::from_secs(1));
    targets = simulation_benchmark
}

criterion_main!(benches); 