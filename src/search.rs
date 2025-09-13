use std::process::Command;
use std::str;
use crate::types::Match;

// search for symbol in path using ripgrep
pub fn search_symbol(path: &str, symbol: &str) -> Vec<Match> {
    let output = Command::new("rg")
        .arg("--line-number")
        .arg("--no-heading")
        .arg("--color=never")
        .arg(symbol)
        .arg(path)
        .output();

    let output = match output {
        Ok(output) => output,
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                eprintln!("ripgrep (rg) not found. Please install ripgrep to use this tool.");
                eprintln!("You can install it from: https://github.com/BurntSushi/ripgrep#installation");
                std::process::exit(1);
            } else {
                panic!("Failed to run ripgrep: {}", e);
            }
        }
    };

    let stdout = str::from_utf8(&output.stdout).unwrap();
    let mut results = Vec::new();

    for line in stdout.lines() {
        let parts: Vec<&str> = line.splitn(3, ':').collect();
        if parts.len() == 3 {
            results.push(Match {
                file: parts[0].to_string(),
                line: parts[1].parse::<usize>().unwrap_or(1),
                content: parts[2].trim().to_string(),
            });
        }
    }

    results
}

