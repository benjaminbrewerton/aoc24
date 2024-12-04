use crate::days::AOCDay;
use std::{fs,path};

const MIN_DISTANCE: i32 = 1;
const MAX_DISTANCE: i32 = 3;

#[derive(Debug)]
pub struct Report {
    entries: Vec<i32>,
}

impl Report {
    fn is_report_valid(&self) -> bool {
        // Check if this report meets all the conditions of;
        // - Adjacent leves increase by 1 <= delta <= 3
        let mut idx = 0;
        let mut deltas: Vec<i32> = Vec::new();
        while idx < self.entries.len() - 1 {
            let delta = self.entries[idx] - self.entries[idx + 1];
            let delta_abs = delta.abs();
            if delta_abs < MIN_DISTANCE || delta_abs > MAX_DISTANCE {
                return false;
            }
            deltas.push(delta);
            idx += 1;
        }
        // - either all increasing or decreasing
        if deltas.iter()
            .map(|x| x.signum())
            .sum::<i32>()
            .abs() != deltas.len() as i32
        {
            return false;
        }
        true
    }
}

impl AOCDay<Vec<Self>> for Report {
    fn interpret_input(file: &str) -> Vec<Self> {
        // Collect each report input on a line and convert each read entry to i32
        let mut entries: Vec<Self> = Vec::new();
        for line in fs::read_to_string(path::Path::new(file))
            .expect("Could not read file.")
            .lines()
            .filter_map(|x| match x.is_empty() {
                false => Some(x),
                true => None,
            })
        {
            let current_entries: Vec<i32> = line.trim()
                .split_whitespace()
                .map(|x| x.parse::<i32>().expect("Bad i32 conversion"))
                .collect();
                entries.push(Self { entries: current_entries })
        }
        entries
    }

    fn run(items: Vec<Self>) {
        let safe_reports: usize = items.iter()
            .filter_map(|x| match x.is_report_valid() {
                true => Some(x),
                false => None,
            }).count();
        println!("The number of safe reports is {safe_reports}.")
    }
}