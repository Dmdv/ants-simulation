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

## Running Benchmarks

```bash
cargo bench
```

## License

MIT License 