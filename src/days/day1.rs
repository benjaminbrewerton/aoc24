use crate::days::AOCDaySingle;
use std::{fs, path};

#[derive(Debug)]
pub struct Day1 {
    list1: Vec<i32>,
    list2: Vec<i32>,
}

fn _populate_list(list: &mut Vec<i32>, num: i32) { 
    let mut middle = list.len() / 2;
    if num < list[middle] {
        while middle > 0 && num < list[middle-1] {
            middle -= 1;
        }
    } else {
        while middle < list.len() && num > list[middle] {
            middle += 1;
        }
    }
    list.insert(middle, num);
}

impl AOCDaySingle<Self> for Day1 {
    fn interpret_input(file: &str) -> Self {
        // Collect the inputs of each list element (left, right) and insert into the appropriate
        // vector in a sorted manner.
        let mut list_left: Vec<i32> = Vec::new();
        let mut list_right: Vec<i32> = Vec::new();
        for line in fs::read_to_string(path::Path::new(file))
                .expect("Could not open file...")
                .lines()
                .into_iter()
        {
            let list1 = &mut list_left;
            let list2 = &mut list_right;
            let nums: Vec<i32> = line.split_whitespace()
                .map(|x| x.parse::<i32>().expect("Failed int conversion."))
                .collect();
            let (num1, num2) = match &nums[..] {
                &[first, second, ..] => (first, second),
                _ => unreachable!(),
            };

            if list1.is_empty() && list2.is_empty() {
                list1.push(num1);
                list2.push(num2);
                continue;
            }

            _populate_list(list1, num1);
            _populate_list(list2, num2);
        }
        Self { list1: list_left, list2: list_right }
    }

    fn run(&self) {
        let sum: i32 = self.list1.iter().zip(self.list2.iter())
            .map(|(x, y)| (x - y).abs())
            .sum();
        println!("Got total distance of {sum}!");

        let mut right: usize = 0;
        let mut score: i32 = 0;
        for (_, value) in self.list1.iter().enumerate() {
            let mut occurences: usize = 0;
            while *value >= self.list2[right] {
                if *value == self.list2[right] {
                    occurences += 1;
                }
                right += 1;
            }
            if occurences > 0 {
                score += value * occurences as i32;
            }
        }
        println!("Got similarity score of {score}.");
    }
}
