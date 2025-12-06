#[path = "./utils/file.rs"]
mod file;

use std::{i64,};

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, value_name = "FILE", required = true)]
    file: String,
}

#[derive(Debug, Clone)]
struct Problem {
    numbers: Vec<i64>,
    operator: char,
}

fn main() {
    let cli = Cli::parse();
    let file_path = cli.file;
    let raw_data = file::load_file(&file_path);
    let problems = get_problems(&raw_data);
    let mut total = 0;
    for problem in &problems {
        if problem.operator == '+' {
            total += problem.numbers.iter().sum::<i64>();
        }
        if problem.operator == '*' {
            total += problem.numbers.iter().product::<i64>();
        }
    }

    println!("Grand Total: {}", total);
}

fn get_problems(raw_data: &String) -> Vec<Problem> {
    let mut problems: Vec<Problem> = Vec::new();
    let problem_count = raw_data.lines().collect::<Vec<&str>>()[0]
        .split_whitespace()
        .collect::<Vec<&str>>()
        .len();
    for i in 0..problem_count {
        let mut prolem_parts: Vec<&str> = Vec::new();
        for line in raw_data.lines() {
            let splits: Vec<&str> = line.split_whitespace().collect();
            prolem_parts.push(splits[i]);
        }
        let (operator, numbers) = prolem_parts.split_last().unwrap();
        let problem: Problem = Problem {
            numbers: numbers
                .iter()
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<i64>>(),
            operator: operator.chars().nth(0).unwrap(),
        };
        problems.push(problem);
    }
    return problems;
}


fn get_problems_pt2(raw_data: &String) -> Vec<Problem> {
    let mut problems: Vec<Problem> = Vec::new();
    let problem_count = raw_data.lines().collect::<Vec<&str>>()[0]
        .split_whitespace()
        .collect::<Vec<&str>>()
        .len();

    println!("problem count: {}", problem_count);
    for i in 0..problem_count {
        let mut prolem_parts: Vec<&str> = Vec::new();
        for line in raw_data.lines() {
            let splits: Vec<&str> = line.split_whitespace().collect();

            prolem_parts.push(splits[i]);
        }
        println!("prob_parts: {:?}", prolem_parts);
        let (operator, numbers) = prolem_parts.split_last().unwrap();
        let problem: Problem = Problem {
            numbers: numbers
                .iter()
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<i64>>(),
            operator: operator.chars().nth(0).unwrap(),
        };
        problems.push(problem);
    }
    return problems;
}
