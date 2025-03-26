# Invasion

This program simulates the invasion of giant space ants on the planet.

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

## Running Benchmarks

```bash
cargo bench
```

###### Optimization

- For 3 ants: ~97.8% improvement (from ~17ms to ~384µs)
- For 6 ants: ~99.9% improvement (from ~40ms to ~50µs)
- For 9 ants: ~99.9% improvement (from ~60ms to ~7.5µs)

##### Optimization 2

- 3 ants: ~67% faster (from ~480µs to ~159µs)
- 6 ants: ~70% faster (from ~110µs to ~33µs)
- 9 ants: ~70% faster (from ~23µs to ~7µs)


## License

MIT License 
