use crate::days::AOCDay;
use std::{fs,path};

const MIN_DISTANCE: i32 = 1;
const MAX_DISTANCE: i32 = 3;

#[derive(Debug)]
pub struct Report {
    entries: Vec<i32>,
    dampened: bool,
    dampened_idx: usize,
}

impl Report {
    fn dampen_report(&mut self, idx: usize) -> bool {
        if self.dampened {
            return false;
        }
        self.dampened = true;
        self.dampened_idx = idx;
        true
    }

    fn is_report_valid(&mut self) -> bool {
        // Check if this report meets all the conditions of;
        // - Adjacent levels increase by 1 <= delta <= 3
        let mut idx = 0;
        let mut deltas: Vec<i32> = Vec::new();
        while idx < self.entries.len() - 1 {
            let delta = self.entries[idx] - self.entries[idx + 1];
            let delta_abs = delta.abs();
            if delta_abs < MIN_DISTANCE || delta_abs > MAX_DISTANCE
            {
                println!("Dampening report {:?} @ idx {idx}", self.entries);
                if !self.dampen_report(idx) {
                    return false;
                }
            }
            if !self.dampened  {
                deltas.push(delta);
            }
            idx += 1;
        }
        // - either all increasing or decreasing
        let mut cur_idx = idx;
        let mut directions: i32 = 0;
        for (idx, direction) in deltas.iter().enumerate() {
            let mut forward_idx = idx + 1;
            if self.dampened {
                if cur_idx == self.dampened_idx {
                    cur_idx += 1;
                    forward_idx += 1;
                }
                if forward_idx == self.dampened_idx {
                    forward_idx += 1;
                }
            }
            if forward_idx > deltas.len() - 1 {
                break;
            }
            if direction.signum() != deltas[forward_idx].signum() &&
                !self.dampen_report(forward_idx)
            {
                println!("Dampening {:?} increment {:?} @ idx {cur_idx}", self.entries, deltas);
                return false;
            }
            directions += direction.signum();
        }
        if directions.abs() != deltas.len() as i32 {
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
            let report = Self {
                entries: current_entries,
                dampened: false,
                dampened_idx: 0,
            };
            entries.push(report);
        }
        entries
    }

    fn run(filename: &str) -> bool {
        let mut items = Report::interpret_input(filename);
        let mut safe_reports: i32 = 0;
        for report in items.iter_mut() {
            if report.is_report_valid() {
                safe_reports += 1;
            }
        }
        println!("The number of safe reports is {safe_reports}.");
        true
    }
}