#![feature(iter_map_windows)]

use std::path::Path;
use std::fs::File;
use std::io;
use std::io::BufRead;

pub mod day01;
pub mod day02;
pub mod day03;

pub fn get_input(file_name: &str) -> Vec<String> {
    let path_name = format!("./src/input/{}", file_name);
    let path = Path::new((&path_name).into());
    let file = File::open(&path).unwrap();
    let reader = io::BufReader::new(file);
    let mut input = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        input.push(line);
    }
    input
}

fn get_sample_input(file_name: &str) -> Vec<String> {
    let path_name = format!("./src/input_samples/{}", file_name);
    let path = Path::new(&path_name);
    let file = File::open(&path).unwrap();
    let reader = io::BufReader::new(file);
    let mut input = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        input.push(line);
    }
    input
}
