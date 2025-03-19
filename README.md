# Ant Mania

A Rust implementation of the Ant Mania simulation challenge. This program simulates the invasion of giant space ants on the planet.

## Features

- Efficient simulation of ant movement and colony destruction
- Command-line interface for easy configuration
- Performance benchmarking capabilities
- Thread-safe implementation

## Building

```bash
cargo build --release
```

## Usage

```bash
cargo run --release -- -a <number_of_ants> -m <path_to_map_file>
```

Example:
```bash
cargo run --release -- -a 100 -m map.txt
```

## Map File Format

The map file should contain one colony per line, with the following format:
```
ColonyName direction1=Target1 direction2=Target2 ...
```

Example:
```
Fizz north=Buzz west=Bla south=Blub
Buzz south=Fizz west=Blip
```

## Performance Optimization Steps

1. **Efficient Data Structures**
   - Used `HashMap` for O(1) lookups of colonies and ant positions
   - Implemented `HashSet` for tracking colonies to destroy
   - Minimized allocations during simulation steps

2. **Memory Management**
   - Reused data structures where possible
   - Minimized string cloning
   - Used references instead of cloning where appropriate

3. **Algorithm Optimization**
   - Batch processing of ant movements
   - Efficient colony destruction handling
   - Early termination when no ants are active

4. **Benchmarking**
   - Included comprehensive benchmarks
   - Tests with varying numbers of ants
   - Grid-based test map for consistent results

## Running Benchmarks

```bash
cargo bench
```

## License

MIT License 