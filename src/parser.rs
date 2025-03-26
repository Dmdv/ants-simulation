use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use crate::colony::{Colony, Direction};

pub fn parse_map_file<P: AsRef<Path>>(path: P) -> io::Result<Vec<Colony>> {
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    let mut colonies = Vec::new();
    let mut name_to_idx = HashMap::new();

    // First pass: create colonies and build name->index mapping
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        if parts.is_empty() {
            continue;
        }

        let colony_name = parts[0].to_string();
        let colony = Colony::new(colony_name.clone());
        name_to_idx.insert(colony_name, colonies.len());
        colonies.push(colony);
    }

    // Second pass: add tunnels using indices
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    for (idx, line) in reader.lines().enumerate() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        if parts.is_empty() {
            continue;
        }

        for part in &parts[1..] {
            if let Some((direction, target)) = part.split_once('=') {
                if let Some(dir) = Direction::from_str(direction) {
                    if let Some(&target_idx) = name_to_idx.get(target) {
                        colonies[idx].add_tunnel(dir, target_idx);
                    }
                }
            }
        }
    }

    Ok(colonies)
}