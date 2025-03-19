use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use crate::colony::{Colony, Direction};

pub fn parse_map_file<P: AsRef<Path>>(path: P) -> io::Result<HashMap<String, Colony>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut colonies = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        if parts.is_empty() {
            continue;
        }

        let colony_name = parts[0].to_string();
        let mut colony = Colony::new(colony_name.clone());

        for part in &parts[1..] {
            if let Some((direction, target)) = part.split_once('=') {
                if let Some(dir) = Direction::from_str(direction) {
                    colony.add_tunnel(dir, target.to_string());
                }
            }
        }

        colonies.insert(colony_name, colony);
    }

    Ok(colonies)
} 