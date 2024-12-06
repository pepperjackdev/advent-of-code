use std::fs;

fn main() {
    let raw = fs::read_to_string("input.txt").expect("input not found");
    let lines: Vec<&str> = raw.lines().collect();

    let safe_count = lines.iter().filter(|&&report| is_safe(report)).count();
    let dampened_safe_count = lines.iter().filter(|&&report| is_safe_with_dampener(report)).count();

    println!("Safe reports without dampener: {}", safe_count);
    println!("Safe reports with dampener: {}", dampened_safe_count);
}

fn is_safe(report: &str) -> bool {
    let levels: Vec<i32> = report.split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    if levels.is_empty() {
        return false;
    }

    let is_increasing = levels.windows(2).all(|w| {
        let diff = w[1] - w[0];
        diff > 0 && diff <= 3
    });

    let is_decreasing = levels.windows(2).all(|w| {
        let diff = w[0] - w[1];
        diff > 0 && diff <= 3
    });

    is_increasing || is_decreasing
}

fn is_safe_with_dampener(report: &str) -> bool {
    let levels: Vec<i32> = report.split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    if levels.is_empty() {
        return false;
    }

    // Check if the report is already safe
    if is_safe(report) {
        return true;
    }

    // Try removing each level and check if the report becomes safe
    for i in 0..levels.len() {
        let mut modified_levels = levels.clone();
        modified_levels.remove(i);
        let modified_report = modified_levels.iter()
            .map(|&level| level.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        
        if is_safe(&modified_report) {
            return true;
        }
    }

    false
}